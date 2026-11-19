#include "app.h"
#include "art.h"

#include <QDate>
#include <QDir>
#include <QFile>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QLocale>
#include <QPainter>
#include <QRunnable>
#include <QStandardPaths>
#include <QSvgRenderer>
#include <QThreadPool>
#include <QTimer>
#ifdef Q_OS_ANDROID
#include <QCoreApplication>
#include <QJniEnvironment>
#include <QJniObject>
#endif

using namespace jardim;
static constexpr int GOAL = 5;

// ================================================================ SvgProvider

namespace {
class SvgResponse : public QQuickImageResponse, public QRunnable {
public:
    SvgResponse(QString id, QSize size) : id_(std::move(id)), size_(size) { setAutoDelete(false); }
    void run() override {
        const QByteArray svg = QByteArray::fromStdString(art::fromSpec(id_.toStdString()));
        QSvgRenderer r(svg);
        QSize s = size_.isValid() && !size_.isEmpty() ? size_ : r.defaultSize();
        image_ = QImage(s, QImage::Format_ARGB32_Premultiplied);  // formato nativo de upload
        image_.fill(Qt::transparent);
        QPainter p(&image_);
        p.setRenderHint(QPainter::Antialiasing);
        r.render(&p);
        p.end();
        emit finished();
    }
    QQuickTextureFactory* textureFactory() const override { return QQuickTextureFactory::textureFactoryForImage(image_); }
private:
    QString id_;
    QSize size_;
    QImage image_;
};
}  // namespace

QQuickImageResponse* SvgProvider::requestImageResponse(const QString& id, const QSize& requestedSize) {
    auto* r = new SvgResponse(id, requestedSize);
    QThreadPool::globalInstance()->start(r);
    return r;
}

// ================================================================ Saver

Saver::Saver(QString path) : path_(std::move(path)) {
    worker_ = std::thread([this] {
        std::unique_lock lk(m_);
        for (;;) {
            cv_.wait(lk, [this] { return quit_ || pending_; });
            if (pending_) {
                QByteArray bytes = std::move(*pending_);
                pending_.reset();
                lk.unlock();
                QFile f(path_ + ".tmp");
                if (f.open(QIODevice::WriteOnly) && f.write(bytes) == bytes.size()) {
                    f.close();
                    QFile::remove(path_);
                    f.rename(path_);
                }
                lk.lock();
            } else if (quit_) return;
        }
    });
}
Saver::~Saver() {
    { std::lock_guard g(m_); quit_ = true; }
    cv_.notify_one();
    worker_.join();
}
void Saver::save(QByteArray bytes) {
    { std::lock_guard g(m_); pending_ = std::move(bytes); }
    cv_.notify_one();
}

// ================================================================ Tts

#ifdef Q_OS_ANDROID
static QJniObject* jni(void* p) { return static_cast<QJniObject*>(p); }
Tts::Tts() {
    QJniObject ctx = QNativeInterface::QAndroidApplication::context();
    auto* o = new QJniObject("android/speech/tts/TextToSpeech",
                             "(Landroid/content/Context;Landroid/speech/tts/TextToSpeech$OnInitListener;)V",
                             ctx.object(), nullptr);
    QJniEnvironment env;
    env.checkAndClearExceptions();
    if (o->isValid()) obj_ = o; else delete o;
}
void Tts::say(const QString& text) {
    if (!obj_) return;
    QJniEnvironment env;
    if (!configured_) {  // o motor liga de forma assíncrona: configura na 1ª fala em que responde
        QJniObject locale("java/util/Locale", "(Ljava/lang/String;Ljava/lang/String;)V",
                          QJniObject::fromString("pt").object<jstring>(), QJniObject::fromString("BR").object<jstring>());
        jint r = jni(obj_)->callMethod<jint>("setLanguage", "(Ljava/util/Locale;)I", locale.object());
        env.checkAndClearExceptions();
        if (r >= 0) { jni(obj_)->callMethod<jint>("setSpeechRate", "(F)I", jfloat(0.88f)); configured_ = true; }
    }
    jni(obj_)->callMethod<jint>("speak", "(Ljava/lang/CharSequence;ILandroid/os/Bundle;Ljava/lang/String;)I",
                                QJniObject::fromString(text).object<jstring>(), jint(0), nullptr,
                                QJniObject::fromString("jardim").object<jstring>());
    env.checkAndClearExceptions();
}
void Tts::stop() {
    if (!obj_) return;
    jni(obj_)->callMethod<jint>("stop", "()I");
    QJniEnvironment().checkAndClearExceptions();
}
#else
Tts::Tts() {}
void Tts::say(const QString& text) { qInfo("[voz] %s", qPrintable(text)); }
void Tts::stop() {}
#endif

// ================================================================ App

static QString today() { return QDate::currentDate().toString("yyyy-MM-dd"); }
static QString qs(const std::string& s) { return QString::fromStdString(s); }
static std::string noHash(const std::string& h) { return h.empty() ? "-" : (h[0] == '#' ? h.substr(1) : h); }

App::App(QObject* parent) : QObject(parent), entrySeed_(entrySeed()) {
    const QString dir = QStandardPaths::writableLocation(QStandardPaths::AppDataLocation);
    QDir().mkpath(dir);
    saver_ = std::make_unique<Saver>(dir + "/jardim-v1.json");
    load();
    tts_ = std::make_unique<Tts>();
}

void App::load() {
    date_ = today();
    QFile f(QStandardPaths::writableLocation(QStandardPaths::AppDataLocation) + "/jardim-v1.json");
    if (!f.open(QIODevice::ReadOnly)) return;
    const QJsonObject o = QJsonDocument::fromJson(f.readAll()).object();
    if (o["version"].toInt() != 1) return;
    const QString g = o["group"].toString();
    bool validGroup = false;
    for (auto& gr : GROUPS) validGroup |= g == gr.id;
    if (!validGroup) return;
    auto validKey = [](const QString& k) {
        auto parts = k.split(':');
        if (parts.size() != 2) return false;
        bool a = false, b = false;
        for (auto& gr : GROUPS) a |= parts[0] == gr.id;
        for (auto& gm : GAMES) b |= parts[1] == gm.id;
        return a && b;
    };
    group_ = g;
    voice_ = o["voice"].toBool(true);
    const QJsonObject st = o["stats"].toObject();
    for (auto it = st.begin(); it != st.end(); ++it) {
        if (!validKey(it.key())) continue;
        auto v = it.value().toObject();
        stats_[it.key()] = {v["solved"].toInteger(), v["tries"].toInteger(), v["sets"].toInteger(), v["rounds"].toInteger()};
    }
    const QJsonObject d = o["daily"].toObject();
    if (d["date"].toString() == date_) {
        const QJsonObject acts = d["activities"].toObject();
        for (auto it = acts.begin(); it != acts.end(); ++it) {
            if (!validKey(it.key())) continue;
            int s = std::min(GOAL, it.value().toObject()["solved"].toInt());
            daily_[it.key()] = {s, s == GOAL};
        }
    }
}

void App::save() {
    QJsonObject st, acts;
    for (auto& [k, s] : stats_) st[k] = QJsonObject{{"solved", s.solved}, {"tries", s.tries}, {"sets", s.sets}, {"rounds", s.rounds}};
    for (auto& [k, d] : daily_) acts[k] = QJsonObject{{"solved", d.solved}, {"completed", d.completed}};
    QJsonObject o{{"version", 1}, {"group", group_}, {"voice", voice_}, {"stats", st},
                  {"daily", QJsonObject{{"date", date_}, {"activities", acts}}}};
    saver_->save(QJsonDocument(o).toJson(QJsonDocument::Compact));  // serializa aqui, grava na outra thread
}

bool App::ensureDay() {
    const QString t = today();
    if (t == date_) return false;
    date_ = t;
    daily_.clear();
    save();
    return true;
}

void App::speak(const QString& text) {
    if (voice_ && tts_) tts_->say(text);  // QUEUE_FLUSH já interrompe a fala anterior
}
void App::stopVoice() { if (tts_) tts_->stop(); }

DailyRec App::daily(const QString& game) const {
    auto it = daily_.find(key(game));
    return it == daily_.end() ? DailyRec{} : it->second;
}

QString App::groupName() const { return GROUPS[groupIndex(group_.toStdString())].name; }
int App::total() const { qint64 t = 0; for (auto& [k, s] : stats_) t += s.solved; return int(t); }

QVariantList App::groups() const {
    QVariantList l;
    for (auto& g : GROUPS) l.append(QVariantMap{{"id", g.id}, {"label", g.label}});
    return l;
}

static std::vector<const Game*> catalogFor(const QString& date, const QString& group, uint32_t entry) {
    return catalog(groupIndex(group.toStdString()), seedFor(date.toStdString(), group.toStdString(), "catalog", 0, entry));
}

QVariantList App::categories() const {
    auto avail = catalogFor(date_, group_, entrySeed_);
    QVariantList l;
    for (auto& [id, label] : CATEGORIES) {
        bool has = std::string(id) == "all";
        for (auto* g : avail) has |= std::string(g->category) == id;
        if (has) l.append(QVariantMap{{"id", id}, {"label", label}});
    }
    return l;
}

QVariantList App::cards() const {
    QVariantList l;
    for (auto* g : catalogFor(date_, group_, entrySeed_)) {
        if (category_ != "all" && category_ != g->category) continue;
        auto d = daily(g->id);
        l.append(QVariantMap{{"id", g->id}, {"name", g->name}, {"skill", g->skill}, {"color", g->color},
                             {"art", "tile/" + QString(g->icon)}, {"wide", art::tile(g->icon).find("0 0 240 120") != std::string::npos},
                             {"solved", d.solved}, {"completed", d.completed}});
    }
    return l;
}

int App::available() const { return int(catalogFor(date_, group_, entrySeed_).size()); }
int App::dailyDone() const {
    int n = 0;
    for (auto* g : catalogFor(date_, group_, entrySeed_)) n += daily(g->id).completed;
    return n;
}

static QVariantMap visualMap(const Visual& v) {
    QVariantMap m{{"type", qs(v.type)}, {"number", v.number}, {"text", qs(v.text)}, {"color", qs(v.color)}};
    if (v.type == "icon") m["art"] = qs("icon/" + v.kind + "/" + noHash(v.color));
    else if (v.type == "sized") m["art"] = qs("icon/" + v.kind + "/" + noHash(v.color));
    else if (v.type == "position") m["art"] = qs("position/" + v.kind);
    else if (v.type == "sock") m["art"] = qs("sock/" + noHash(v.color) + "/" + v.kind);
    else if (v.type == "cup") m["art"] = qs("cup/" + std::to_string(v.number));
    else if (v.type == "container") m["art"] = qs("container/" + v.kind);
    else if (v.type == "tower") {
        std::string cs; for (size_t i = 0; i < v.colors.size(); ++i) cs += (i ? "," : "") + noHash(v.colors[i]);
        m["art"] = qs("tower/" + cs);
    }
    return m;
}

QVariantMap App::question() const {
    if (screen_ != "game") return {};
    QVariantList opts, cards, rot;
    for (auto& o : q_.options) opts.append(QVariantMap{{"label", qs(o.label)}, {"visual", visualMap(o.visual)}});
    for (auto& c : q_.cards) cards.append(QVariantMap{{"kind", qs(c.kind)}, {"pair", int(c.pair)}});
    for (int r : q_.presentation.rotation) rot.append(r);
    QVariantMap scene;
    if (q_.scene) {
        auto& s = *q_.scene;
        QVariantList items, numbers;
        for (auto& i : s.items) items.append(qs(i));
        for (auto& n : s.numbers) numbers.append(n ? QVariant(*n) : QVariant());
        scene = {{"type", qs(s.type)}, {"color", qs(s.color)}, {"shape", qs(s.shape)}, {"icon", qs(s.icon)}, {"text", qs(s.text)},
                 {"word", qs(s.word)}, {"pattern", qs(s.pattern)}, {"items", items}, {"numbers", numbers},
                 {"a", s.a}, {"b", s.b}, {"count", s.count}};
        if (s.type == "shape") scene["art"] = qs("icon/" + s.shape + "/" + noHash(s.color));
        if (s.type == "sock") scene["art"] = qs("sock/" + noHash(s.color) + "/" + s.pattern);
        if (s.type == "tower") { std::string cs; for (size_t i = 0; i < s.items.size(); ++i) cs += (i ? "," : "") + noHash(s.items[i]); scene["art"] = qs("tower/" + cs); }
        if (s.type == "clock") scene["art"] = qs("clock/" + std::to_string(s.count));
    }
    return {{"game", qs(q_.game)}, {"gameName", gameById(q_.game).name}, {"prompt", qs(q_.prompt)}, {"hint", qs(q_.hint)},
            {"memory", q_.memory}, {"options", opts}, {"answer", int(q_.answer)}, {"cards", cards}, {"scene", scene},
            {"hasScene", bool(q_.scene)}, {"surface", qs(q_.presentation.surface)}, {"radius", q_.presentation.radius},
            {"itemColor", qs(q_.presentation.itemColor)}, {"itemGap", q_.presentation.itemGap}, {"rotation", rot}};
}

QVariantMap App::session() const {
    QVariantList rev, mat, cnt;
    for (int i : revealed_) rev.append(i);
    for (int i : matched_) mat.append(i);
    for (int i : counted_) cnt.append(i);
    auto d = daily(game_);
    return {{"round", round_}, {"length", length_}, {"correct", correct_}, {"feedback", feedback_},
            {"revealed", rev}, {"matched", mat}, {"counted", cnt}, {"wiggle", wiggle_}, {"wiggleSerial", wiggleSerial_},
            {"dailySolved", d.solved}, {"dailyCompleted", d.completed}};
}

QVariantList App::parentStats() const {
    QVariantList l;
    for (auto& g : GAMES) {
        auto it = stats_.find(key(g.id));
        l.append(QVariantMap{{"name", g.name}, {"today", daily(g.id).solved}, {"total", it == stats_.end() ? 0 : int(it->second.solved)}});
    }
    return l;
}

// ---------------------------------------------------------------- ações

void App::home() {
    ensureDay();
    stopVoice();
    screen_ = "home";
    game_.clear();
    emit changed();
}
void App::setGroup(const QString& g) { group_ = g; category_ = "all"; save(); emit changed(); }
void App::setCategory(const QString& c) { category_ = c; emit changed(); }
void App::explore() { speak("Escolha uma atividade para começar."); }

void App::start(const QString& game) {
    ensureDay();
    game_ = game;
    auto d = daily(game);
    length_ = d.completed ? GOAL : GOAL - d.solved;
    round_ = 0;
    screen_ = "game";
    nextQuestion();
}

void App::nextQuestion() {
    ensureDay();
    Stat& s = stats_[key(game_)];
    s.rounds++;
    save();
    uint32_t seed = seedFor(date_.toStdString(), group_.toStdString(), game_.toStdString(), uint64_t(s.rounds), entrySeed_);
    q_ = generate(game_.toStdString(), group_.toStdString(), seed);
    correct_ = false;
    feedback_ = "idle";
    revealed_.clear(); matched_.clear(); counted_.clear();
    wiggle_ = -1; busy_ = false; ++flipToken_;
    emit changed();
    speak(qs((q_.story.empty() ? "" : q_.story + " ") + q_.prompt));
}

void App::recordTry() { stats_[key(game_)].tries++; save(); }

void App::correct() {
    if (correct_) return;
    correct_ = true;
    feedback_ = "success";
    ensureDay();
    DailyRec& d = daily_[key(game_)];
    d.solved = std::min(GOAL, d.solved + 1);
    d.completed = d.solved == GOAL;
    stats_[key(game_)].solved++;
    save();
    emit changed();
    speak("Você descobriu! Muito bem.");
}

void App::answer(int i) {
    if (screen_ != "game" || correct_) return;
    recordTry();
    if (size_t(i) == q_.answer) { correct(); return; }
    wiggle_ = i;
    const int serial = ++wiggleSerial_;
    feedback_ = "retry";
    emit changed();
    QTimer::singleShot(500, this, [this, serial] {  // como o app.js: tira o destaque após 0,5 s
        if (serial == wiggleSerial_ && wiggle_ >= 0) { wiggle_ = -1; emit changed(); }
    });
    speak("Vamos tentar de novo? " + qs(q_.hint));
}

void App::flip(int i) {
    auto has = [](const std::vector<int>& v, int x) { return std::find(v.begin(), v.end(), x) != v.end(); };
    if (busy_ || correct_ || has(revealed_, i) || has(matched_, i)) return;
    revealed_.push_back(i);
    if (revealed_.size() < 2) { emit changed(); return; }
    recordTry();
    int a = revealed_[0], d = revealed_[1];
    if (q_.cards[a].pair == q_.cards[d].pair) {
        matched_.push_back(a); matched_.push_back(d);
        revealed_.clear();
        if (matched_.size() == q_.cards.size()) correct();
        else { emit changed(); speak("Um par!"); }
    } else {
        busy_ = true;
        emit changed();
        int token = ++flipToken_;
        QTimer::singleShot(1200, this, [this, token] {
            if (token != flipToken_) return;
            revealed_.clear();
            busy_ = false;
            emit changed();
        });
    }
}

void App::next() {
    if (!correct_) return;
    if (++round_ == length_) finish(); else nextQuestion();
}

void App::finish() {
    ensureDay();
    stopVoice();
    stats_[key(game_)].sets++;
    save();
    screen_ = "finish";
    emit changed();
    QString headline = length_ == 1 ? "Uma nova descoberta!" : QString::number(length_) + " novas descobertas!";
    speak(headline + " Que tal fazer uma pausa?");
}

void App::hint() {
    if (!correct_) feedback_ = "hint";
    emit changed();
    speak(qs((q_.story.empty() ? "" : q_.story + " ") + q_.hint));
}
void App::listen() { speak(qs((q_.story.empty() ? "" : q_.story + " ") + q_.prompt)); }
void App::countItem(int i) { if (!counted_.erase(i)) counted_.insert(i); emit changed(); }
void App::toggleVoice() {
    voice_ = !voice_;
    save();
    if (!voice_) stopVoice(); else if (screen_ == "game") speak(qs(q_.prompt));
    emit changed();
}
void App::reset() { stats_.clear(); daily_.clear(); save(); emit changed(); }
void App::resume() {
    entrySeed_ = entrySeed();
    ensureDay();
    emit changed();
}
