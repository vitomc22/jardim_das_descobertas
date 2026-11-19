#pragma once
#include "curriculum.h"

#include <QObject>
#include <QQuickAsyncImageProvider>
#include <QVariant>

#include <condition_variable>
#include <map>
#include <mutex>
#include <optional>
#include <set>
#include <thread>

// SVG → QImage em threads do QThreadPool (todos os núcleos); o Qt Quick
// guarda o resultado no cache de texturas pela URL + sourceSize.
class SvgProvider : public QQuickAsyncImageProvider {
public:
    QQuickImageResponse* requestImageResponse(const QString& id, const QSize& requestedSize) override;
};

struct Stat { qint64 solved = 0, tries = 0, sets = 0, rounds = 0; };
struct DailyRec { int solved = 0; bool completed = false; };

// Grava o JSON numa thread própria; rajadas são coalescidas (só a última vale).
class Saver {
public:
    explicit Saver(QString path);
    ~Saver();
    void save(QByteArray bytes);
private:
    QString path_;
    std::mutex m_;
    std::condition_variable cv_;
    std::optional<QByteArray> pending_;
    bool quit_ = false;
    std::thread worker_;
};

// Narração pt-BR: TextToSpeech do Android chamado direto via JNI (QJniObject),
// sem o módulo QtTextToSpeech (que puxaria o QtMultimedia + FFmpeg pro APK).
class Tts {
public:
    Tts();
    void say(const QString& text);
    void stop();
private:
    bool configured_ = false;
#ifdef Q_OS_ANDROID
    void* obj_ = nullptr;  // QJniObject* (evita incluir QJniObject no header)
#endif
};

class App : public QObject {
    Q_OBJECT
    Q_PROPERTY(QString screen READ screen NOTIFY changed)
    Q_PROPERTY(QString group READ group NOTIFY changed)
    Q_PROPERTY(QString groupName READ groupName NOTIFY changed)
    Q_PROPERTY(bool voice READ voice NOTIFY changed)
    Q_PROPERTY(int total READ total NOTIFY changed)
    Q_PROPERTY(QString category READ category NOTIFY changed)
    Q_PROPERTY(QVariantList groups READ groups CONSTANT)
    Q_PROPERTY(QVariantList categories READ categories NOTIFY changed)
    Q_PROPERTY(QVariantList cards READ cards NOTIFY changed)
    Q_PROPERTY(int available READ available NOTIFY changed)
    Q_PROPERTY(int dailyDone READ dailyDone NOTIFY changed)
    Q_PROPERTY(QVariantMap question READ question NOTIFY changed)
    Q_PROPERTY(QVariantMap session READ session NOTIFY changed)
    Q_PROPERTY(QVariantList parentStats READ parentStats NOTIFY changed)
public:
    explicit App(QObject* parent = nullptr);

    QString screen() const { return screen_; }
    QString group() const { return group_; }
    QString groupName() const;
    bool voice() const { return voice_; }
    int total() const;
    QString category() const { return category_; }
    QVariantList groups() const;
    QVariantList categories() const;
    QVariantList cards() const;
    int available() const;
    int dailyDone() const;
    QVariantMap question() const;
    QVariantMap session() const;
    QVariantList parentStats() const;

    Q_INVOKABLE void home();
    Q_INVOKABLE void setGroup(const QString& g);
    Q_INVOKABLE void setCategory(const QString& c);
    Q_INVOKABLE void start(const QString& game);
    Q_INVOKABLE void answer(int i);
    Q_INVOKABLE void flip(int i);
    Q_INVOKABLE void next();
    Q_INVOKABLE void hint();
    Q_INVOKABLE void listen();
    Q_INVOKABLE void countItem(int i);
    Q_INVOKABLE void toggleVoice();
    Q_INVOKABLE void reset();
    Q_INVOKABLE void explore();
    Q_INVOKABLE void stopVoice();
    Q_INVOKABLE void resume();  // app voltou do segundo plano

signals:
    void changed();

private:
    void load();
    void save();
    bool ensureDay();
    void nextQuestion();
    void correct();
    void recordTry();
    void finish();
    void speak(const QString& text);
    QString key(const QString& game) const { return group_ + ":" + game; }
    DailyRec daily(const QString& game) const;

    QString screen_ = "home", group_ = "little", category_ = "all", date_;
    bool voice_ = true;
    std::map<QString, Stat> stats_;
    std::map<QString, DailyRec> daily_;
    uint32_t entrySeed_;
    std::unique_ptr<Tts> tts_;
    std::unique_ptr<Saver> saver_;

    // sessão
    QString game_;
    int round_ = 0, length_ = 0;
    jardim::Question q_;
    bool correct_ = false;
    QString feedback_ = "idle";
    std::vector<int> revealed_, matched_;
    std::set<int> counted_;
    int wiggle_ = -1, wiggleSerial_ = 0;
    bool busy_ = false;
    int flipToken_ = 0;
};
