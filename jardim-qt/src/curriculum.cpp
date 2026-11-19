#include "curriculum.h"

#include <algorithm>
#include <chrono>
#include <random>

namespace jardim {

// ---------------------------------------------------------------- hash/seed

static std::u16string toUtf16(const std::string& s) {
    std::u16string out;
    for (size_t i = 0; i < s.size();) {
        unsigned char c = s[i];
        uint32_t cp; int n;
        if (c < 0x80) { cp = c; n = 1; }
        else if ((c >> 5) == 6) { cp = c & 0x1f; n = 2; }
        else if ((c >> 4) == 14) { cp = c & 0x0f; n = 3; }
        else { cp = c & 0x07; n = 4; }
        for (int k = 1; k < n && i + k < s.size(); ++k) cp = (cp << 6) | (s[i + k] & 0x3f);
        i += n;
        if (cp >= 0x10000) { cp -= 0x10000; out.push_back(char16_t(0xd800 + (cp >> 10))); out.push_back(char16_t(0xdc00 + (cp & 0x3ff))); }
        else out.push_back(char16_t(cp));
    }
    return out;
}

uint32_t hash(const std::string& text) {
    // JS: for (const ch of text) v ^= ch.charCodeAt(0) → 1ª unidade de cada code point.
    uint32_t v = 2166136261u;
    std::u16string u = toUtf16(text);
    for (size_t i = 0; i < u.size(); ++i) {
        v ^= u[i];
        v *= 16777619u;
        if (u[i] >= 0xd800 && u[i] < 0xdc00) ++i;  // pula a 2ª metade do par
    }
    return v;
}

uint32_t seedFor(const std::string& day, const std::string& group, const std::string& game, uint64_t round, uint32_t entry) {
    return hash(day + "|" + group + "|" + game + "|" + std::to_string(round) + "|" + std::to_string(entry));
}

uint32_t entrySeed() {
    std::random_device rd;
    return rd() ^ uint32_t(std::chrono::steady_clock::now().time_since_epoch().count());
}

// ---------------------------------------------------------------- dados

struct Color { const char* id; const char* name; const char* hex; };
static const std::vector<Color> COLORS = {
    {"coral", "vermelha", "#eb6d58"}, {"blue", "azul", "#589cd4"},
    {"yellow", "amarela", "#efbd46"}, {"green", "verde", "#60a786"}};
static const std::vector<std::string> SHAPES = {"circle", "triangle", "square", "star"};

std::string shapeName(const std::string& s) {
    if (s == "circle") return "círculo";
    if (s == "triangle") return "triângulo";
    if (s == "square") return "quadrado";
    return "estrela";
}

const std::array<Group, 3> GROUPS = {{
    {"little", "2–3 anos", "Primeiros passos", 3, 2},
    {"middle", "4–5 anos", "Mentes curiosas", 6, 3},
    {"big", "6–8 anos", "Grandes ideias", 12, 4},
}};

const std::array<Game, 32> GAMES = {{
    {"colors", "Jardim das cores", "Reconhecer e associar", "flower", "pink", 0, "space"},
    {"shapes", "Formas pelo caminho", "Percepção visual", "shapes", "blue", 0, "space"},
    {"count", "Vamos contar?", "Números e quantidades", "count", "yellow", 0, "math"},
    {"memory", "Pares da floresta", "Memória e atenção", "memory", "green", 0, "logic"},
    {"pattern", "O que vem depois?", "Sequências e lógica", "pattern", "purple", 1, "logic"},
    {"odd", "Um é diferente", "Observar e comparar", "odd", "peach", 0, "logic"},
    {"sum", "Somar sorrisos", "Primeiras operações", "sum", "yellow", 1, "math"},
    {"order", "Escada dos números", "Ordem e raciocínio", "order", "blue", 1, "math"},
    {"compare", "Onde tem mais?", "Comparar quantidades", "count", "yellow", 0, "math"},
    {"sizes", "Grande ou pequeno?", "Comparar tamanhos", "shapes", "blue", 0, "space"},
    {"position", "Onde está a flor?", "Posição no espaço", "flower", "green", 0, "space"},
    {"shadow", "De quem é a sombra?", "Contornos e associação", "shapes", "purple", 0, "space"},
    {"sort", "Cada um no seu grupo", "Classificar e relacionar", "memory", "green", 0, "logic"},
    {"letters", "Letras companheiras", "Maiúsculas e minúsculas", "letters", "pink", 1, "language"},
    {"initial", "Começa com qual letra?", "Letra inicial das palavras", "letters", "peach", 1, "language"},
    {"vowels", "Jardim das vogais", "Reconhecer as vogais", "letters", "pink", 1, "language"},
    {"syllables", "Palavras em pedacinhos", "Contar sílabas", "letters", "purple", 2, "language"},
    {"word", "Palavra e desenho", "Leitura de palavras", "letters", "peach", 2, "language"},
    {"subtract", "Quantas ficaram?", "Subtração com imagens", "count", "yellow", 1, "math"},
    {"missing", "O número escondido", "Completar uma adição", "sum", "yellow", 2, "math"},
    {"tenframe", "Casinha dos dez", "Quantidades até dez", "count", "blue", 1, "math"},
    {"tens", "Dez de cada vez", "Dezenas e unidades", "order", "yellow", 2, "math"},
    {"clock", "Que horas são?", "Horas inteiras no relógio", "clock", "blue", 2, "math"},
    {"story", "Histórias do jardim", "Escutar e compreender", "book", "green", 1, "language"},
    {"twins", "Encontre o igual", "Associar objetos iguais", "memory", "green", 0, "logic"},
    {"inside", "Dentro ou fora?", "Explorar posições", "container", "blue", 0, "space"},
    {"fill", "Cheio ou vazio?", "Comparar recipientes", "cup", "yellow", 0, "space"},
    {"tower", "Torres coloridas", "Copiar um modelo", "tower", "purple", 0, "logic"},
    {"socks", "Meias companheiras", "Combinar cores e detalhes", "socks", "pink", 0, "logic"},
    {"oneeach", "Um para cada amigo", "Relacionar um a um", "count", "peach", 0, "math"},
    {"vocabulary", "Quem foi chamado?", "Ouvir e reconhecer", "book", "green", 0, "language"},
    {"colorpath", "Caminho das cores", "Continuar uma repetição", "colorpath", "pink", 0, "logic"},
}};

const std::array<std::pair<const char*, const char*>, 5> CATEGORIES = {{
    {"all", "Todas"}, {"math", "Matemática"}, {"logic", "Lógica"},
    {"space", "Formas e espaço"}, {"language", "Letras e histórias"}}};

size_t groupIndex(const std::string& id) {
    for (size_t i = 0; i < GROUPS.size(); ++i) if (id == GROUPS[i].id) return i;
    return 0;
}
const Game& gameById(const std::string& id) {
    for (auto& g : GAMES) if (id == g.id) return g;
    return GAMES[0];
}

struct Word { const char* word; const char* icon; std::vector<std::string> syl; };
static const std::vector<Word> WORDS = {
    {"SOL", "sun", {"SOL"}}, {"FLOR", "flower", {"FLOR"}}, {"MAÇÃ", "apple", {"MA", "ÇÃ"}},
    {"GATO", "cat", {"GA", "TO"}}, {"BOLA", "ball", {"BO", "LA"}}, {"LUA", "moon", {"LU", "A"}},
    {"CASA", "house", {"CA", "SA"}}, {"PEIXE", "fish", {"PEI", "XE"}},
    {"COELHO", "bunny", {"CO", "E", "LHO"}}, {"PÁSSARO", "bird", {"PÁS", "SA", "RO"}}};

struct Story { const char* text; const char* question; const char* icon; const char* answer; std::array<const char*, 3> choices, labels; };
static const std::vector<Story> STORIES = {
    {"Nina, a coelha, encontrou uma maçã no jardim. Ela dividiu a maçã com seu amigo passarinho.", "Qual fruta Nina dividiu?", "bunny", "apple", {"apple", "flower", "ball"}, {"maçã", "flor", "bola"}},
    {"O passarinho Tito recolheu uma flor amarela. Depois, levou a flor para perto de sua casa.", "O que Tito recolheu?", "bird", "flower", {"flower", "apple", "fish"}, {"flor", "maçã", "peixe"}},
    {"O gato Bento brincava com uma bola. Quando o sol apareceu, Bento levou a bola para o jardim.", "Com o que Bento brincava?", "cat", "ball", {"ball", "apple", "flower"}, {"bola", "maçã", "flor"}},
    {"A lua apareceu no céu. Nina, a coelha, guardou seus brinquedos e foi descansar.", "O que apareceu no céu?", "bunny", "moon", {"moon", "sun", "flower"}, {"lua", "sol", "flor"}},
    {"Tito viu um peixe nadando no lago. O peixe passou por uma pedra e mergulhou devagar.", "Qual animal estava no lago?", "bird", "fish", {"fish", "cat", "bunny"}, {"peixe", "gato", "coelho"}},
    {"Bento encontrou Nina em frente à casa. O gato e a coelha entraram juntos para brincar.", "Onde os amigos entraram?", "cat", "house", {"house", "flower", "apple"}, {"casa", "flor", "maçã"}}};

static const std::vector<std::pair<std::string, std::string>> OBJECTS = {
    {"apple", "a maçã"}, {"flower", "a flor"}, {"bird", "o passarinho"}, {"bunny", "o coelho"},
    {"cat", "o gato"}, {"fish", "o peixe"}, {"ball", "a bola"}, {"moon", "a lua"},
    {"house", "a casa"}, {"sun", "o sol"}};

// ---------------------------------------------------------------- helpers

static std::string lowerPt(std::string s) {
    for (auto& c : s) if (c >= 'A' && c <= 'Z') c = char(c - 'A' + 'a');
    const std::pair<const char*, const char*> map[] = {{"Ç", "ç"}, {"Ã", "ã"}, {"Á", "á"}};
    for (auto& [from, to] : map)
        for (size_t p; (p = s.find(from)) != std::string::npos;) s.replace(p, std::string(from).size(), to);
    return s;
}

static Visual icon(const std::string& kind, const std::string& color = "") { Visual v; v.type = "icon"; v.kind = kind; v.color = color; return v; }
static Visual numberV(int n) { Visual v; v.type = "number"; v.number = n; return v; }
static Visual textV(const std::string& t) { Visual v; v.type = "text"; v.text = t; return v; }
static Option opt(std::string id, std::string label, Visual v) { return {std::move(id), std::move(label), std::move(v)}; }
static Option numOpt(int n) { return opt(std::to_string(n), std::to_string(n), numberV(n)); }

static void setOptions(Question& q, Rng& r, const std::string& correct, std::vector<Option> items) {
    q.options = r.shuffle(std::move(items));
    q.answer = 0;
    for (size_t i = 0; i < q.options.size(); ++i) if (q.options[i].id == correct) { q.answer = i; break; }
}

template <class T> static std::vector<T> take(std::vector<T> v, size_t n) { if (v.size() > n) v.resize(n); return v; }
static std::vector<int> rangeInts(int lo, int hi, int except) {
    std::vector<int> v; for (int i = lo; i <= hi; ++i) if (i != except) v.push_back(i); return v;
}
static Scene sc(const std::string& type) { Scene s; s.type = type; return s; }

// ---------------------------------------------------------------- jogos

static bool generateToddler(Question& q, const std::string& game, size_t level, uint32_t seed) {
    Rng r(seed);
    size_t choices = level == 0 ? 2 : 3;
    if (game == "twins" || game == "vocabulary") {
        size_t ti = r.below(OBJECTS.size());
        auto [tid, tname] = OBJECTS[ti];
        bool twins = game == "twins";
        q.prompt = twins ? "Qual desenho é igual ao modelo?" : "Encontre " + tname + ".";
        q.hint = twins ? "Olhe o desenho grande e procure outro igual." : "Vamos procurar " + tname + " juntos.";
        if (twins) { Scene s = sc("object-model"); s.icon = tid; q.scene = s; } else q.scene = sc("look");
        std::vector<size_t> others; for (size_t i = 0; i < OBJECTS.size(); ++i) if (i != ti) others.push_back(i);
        std::vector<size_t> idx{ti}; for (auto i : take(r.shuffle(others), choices - 1)) idx.push_back(i);
        std::vector<Option> items; for (auto i : idx) items.push_back(opt(OBJECTS[i].first, OBJECTS[i].second, icon(OBJECTS[i].first)));
        setOptions(q, r, tid, items);
    } else if (game == "inside") {
        std::string target = r.pick(std::vector<std::string>{"inside", "outside"});
        q.prompt = target == "inside" ? "Em qual desenho a flor está dentro da caixa?" : "Em qual desenho a flor está fora da caixa?";
        q.hint = target == "inside" ? "Procure a flor no espaço de dentro da caixa." : "Procure a flor ao lado de fora da caixa.";
        q.scene = sc("look");
        std::vector<Option> items;
        for (std::string p : {"inside", "outside"}) { Visual v; v.type = "container"; v.kind = p; items.push_back(opt(p, p == "inside" ? "Flor dentro da caixa" : "Flor fora da caixa", v)); }
        setOptions(q, r, target, items);
    } else if (game == "fill") {
        int target = r.pick(std::vector<int>{0, 1});
        q.prompt = target == 1 ? "Qual copo está cheio?" : "Qual copo está vazio?";
        q.hint = target == 1 ? "Procure o copo com água até perto da borda." : "Procure o copo sem água.";
        q.scene = sc("look");
        std::vector<double> amounts = level == 0 ? std::vector<double>{0, 1} : std::vector<double>{0, 0.5, 1};
        std::vector<Option> items;
        for (double a : amounts) {
            Visual v; v.type = "cup"; v.number = a;
            std::string id = a == 0.5 ? "0.5" : std::to_string(int(a));
            items.push_back(opt(id, a == 1 ? "Copo cheio" : a == 0 ? "Copo vazio" : "Copo pela metade", v));
        }
        setOptions(q, r, std::to_string(target), items);
    } else if (game == "tower") {
        std::vector<size_t> idx{0, 1, 2, 3};
        std::vector<std::string> palette; for (auto i : take(r.shuffle(idx), level == 0 ? 2 : 3)) palette.push_back(COLORS[i].hex);
        q.prompt = "Qual torre é igual à torre do modelo?";
        q.hint = "Olhe a cor dos blocos de baixo para cima.";
        Scene s = sc("tower"); s.items = palette; q.scene = s;
        std::vector<std::vector<std::string>> variants{palette, {palette.rbegin(), palette.rend()}};
        if (choices == 3) variants.push_back({palette[1], palette[0], palette[2]});
        auto join = [](const std::vector<std::string>& c) { std::string s; for (size_t i = 0; i < c.size(); ++i) s += (i ? ":" : "") + c[i]; return s; };
        std::vector<Option> items;
        for (size_t i = 0; i < variants.size(); ++i) { Visual v; v.type = "tower"; v.colors = variants[i]; items.push_back(opt(join(variants[i]), "Torre " + std::to_string(i + 1), v)); }
        setOptions(q, r, join(palette), items);
    } else if (game == "socks") {
        std::string color = COLORS[r.below(4)].hex;
        std::vector<std::string> others; for (auto& c : COLORS) if (color != c.hex) others.push_back(c.hex);
        std::string other = r.pick(others);
        std::string pattern = r.pick(std::vector<std::string>{"dots", "stripes"});
        q.prompt = "Qual meia faz par com esta?";
        q.hint = "Procure a mesma cor e o mesmo desenho na meia.";
        Scene s = sc("sock"); s.color = color; s.pattern = pattern; q.scene = s;
        std::vector<std::pair<std::string, std::string>> variants{{color, pattern}, {other, pattern}};
        if (choices == 3) variants.push_back({color, pattern == "dots" ? "stripes" : "dots"});
        std::vector<Option> items;
        for (size_t i = 0; i < variants.size(); ++i) { Visual v; v.type = "sock"; v.color = variants[i].first; v.kind = variants[i].second; items.push_back(opt(variants[i].first + variants[i].second, "Meia " + std::to_string(i + 1), v)); }
        setOptions(q, r, color + pattern, items);
    } else if (game == "oneeach") {
        int n = 1 + int(r.below(level == 0 ? 3 : 5));
        q.prompt = "Qual grupo tem uma maçã para cada coelho?";
        q.hint = "Dê uma maçã para um coelho. Depois, uma para o próximo.";
        Scene s = sc("friends"); s.count = n; q.scene = s;
        std::vector<int> ns{n}; for (int c : take(r.shuffle(rangeInts(1, level == 0 ? 3 : 6, n)), choices - 1)) ns.push_back(c);
        std::vector<Option> items;
        for (int c : ns) { Visual v; v.type = "quantity"; v.number = c; items.push_back(opt(std::to_string(c), "Grupo com " + std::to_string(c) + " maçãs", v)); }
        setOptions(q, r, std::to_string(n), items);
    } else if (game == "colorpath") {
        std::vector<size_t> idx{0, 1, 2, 3};
        auto ab = take(r.shuffle(idx), 2);
        const Color &a = COLORS[ab[0]], &b = COLORS[ab[1]];
        q.prompt = "Qual cor continua o caminho?";
        q.hint = "As cores aparecem uma de cada vez e depois se repetem.";
        Scene s = sc("color-sequence"); s.items = {a.hex, b.hex, a.hex}; q.scene = s;
        std::vector<Option> items;
        for (const Color* c : {&a, &b}) { Visual v; v.type = "swatch"; v.color = c->hex; items.push_back(opt(c->id, std::string("Cor ") + c->name, v)); }
        setOptions(q, r, b.id, items);
    } else return false;
    return true;
}

static bool generateExtra(Question& q, const std::string& game, size_t level, uint32_t seed) {
    Rng r(seed);
    size_t tk = level ? 3 : 2;
    auto numbers = [&](int answer, int lo, int hi) {
        std::vector<Option> items{numOpt(answer)};
        for (int n : take(r.shuffle(rangeInts(lo, hi, answer)), tk)) items.push_back(numOpt(n));
        setOptions(q, r, std::to_string(answer), items);
    };
    if (game == "compare") {
        int len = level == 0 ? 3 : level == 1 ? 6 : 10;
        auto counts = take(r.shuffle(rangeInts(1, len, -1)), 3);
        std::string want = r.pick(std::vector<std::string>{"more", "less"});
        int target = want == "more" ? *std::max_element(counts.begin(), counts.end()) : *std::min_element(counts.begin(), counts.end());
        q.prompt = want == "more" ? "Qual grupo tem mais maçãs?" : "Qual grupo tem menos maçãs?";
        q.hint = "Conte as maçãs de cada grupo e compare.";
        q.scene = sc("look");
        std::vector<Option> items;
        for (int n : counts) { Visual v; v.type = "quantity"; v.number = n; items.push_back(opt(std::to_string(n), "Grupo com " + std::to_string(n) + " maçãs", v)); }
        setOptions(q, r, std::to_string(target), items);
    } else if (game == "sizes") {
        std::string shape = r.pick(SHAPES);
        std::string want = r.pick(std::vector<std::string>{"big", "small"});
        auto sizes = r.shuffle(std::vector<double>{0.46, 0.7, 1.0});
        q.prompt = want == "big" ? "Qual desenho é o maior?" : "Qual desenho é o menor?";
        q.hint = "As formas são iguais. Compare o espaço que cada uma ocupa.";
        q.scene = sc("look");
        std::vector<Option> items;
        for (double s : sizes) {
            Visual v; v.type = "sized"; v.kind = shape; v.number = s; v.color = "#7aaa8b";
            std::string id = s == 1.0 ? "1" : s == 0.46 ? "0.46" : "0.7";
            items.push_back(opt(id, s == 1.0 ? "Desenho grande" : s == 0.46 ? "Desenho pequeno" : "Desenho médio", v));
        }
        setOptions(q, r, want == "big" ? "1" : "0.46", items);
    } else if (game == "position") {
        std::vector<std::string> dirs = level == 0 ? std::vector<std::string>{"above", "below"} : std::vector<std::string>{"above", "below", "left", "right"};
        std::string target = r.pick(dirs);
        auto name = [](const std::string& d) -> std::string { return d == "above" ? "acima" : d == "below" ? "abaixo" : d == "left" ? "à esquerda" : "à direita"; };
        q.prompt = "Em qual desenho a flor está " + name(target) + " da caixa?";
        q.hint = target == "left" ? "A esquerda fica deste lado: ←." : target == "right" ? "A direita fica deste lado: →." : target == "above" ? "Procure a flor na parte de cima da caixa." : "Procure a flor na parte de baixo da caixa.";
        q.scene = sc("look");
        std::vector<Option> items;
        for (auto& d : dirs) { Visual v; v.type = "position"; v.kind = d; items.push_back(opt(d, "Flor " + name(d) + " da caixa", v)); }
        setOptions(q, r, target, items);
    } else if (game == "shadow") {
        std::vector<std::string> pool(SHAPES.begin(), SHAPES.begin() + (level ? 4 : 3));
        std::string target = r.pick(pool);
        q.prompt = "Qual desenho combina com esta sombra?";
        q.hint = "Observe o contorno, sem se preocupar com a cor.";
        Scene s = sc("shape"); s.shape = target; s.color = "#344c47"; q.scene = s;
        std::vector<Option> items;
        for (size_t i = 0; i < pool.size(); ++i) items.push_back(opt(pool[i], shapeName(pool[i]), icon(pool[i], COLORS[i].hex)));
        setOptions(q, r, target, items);
    } else if (game == "sort") {
        std::vector<std::string> animals{"bird", "bunny", "cat", "fish"}, plants{"apple", "flower", "leaf"};
        bool isAnimals = r.below(2) == 0;
        auto& ti = isAnimals ? animals : plants; auto& oi = isAnimals ? plants : animals;
        std::string target = r.pick(ti);
        auto label = [](const std::string& k) -> std::string {
            if (k == "bird") return "passarinho"; if (k == "bunny") return "coelho"; if (k == "cat") return "gato"; if (k == "fish") return "peixe";
            if (k == "apple") return "maçã"; if (k == "flower") return "flor"; return "folha"; };
        q.prompt = std::string("Qual destes desenhos é ") + (isAnimals ? "um animal" : "uma planta ou parte de uma planta") + "?";
        q.hint = isAnimals ? "Procure um ser vivo que é um animal." : "Folhas, flores e frutos fazem parte de plantas.";
        q.scene = sc("look");
        std::vector<Option> items{opt(target, label(target), icon(target))};
        for (auto& k : take(r.shuffle(oi), 2)) items.push_back(opt(k, label(k), icon(k)));
        setOptions(q, r, target, items);
    } else if (game == "letters") {
        std::vector<char> alphabet; for (char c : std::string("ABCDEFGHJKLMNPRSTUVZ")) alphabet.push_back(c);
        char t = r.pick(alphabet);
        std::string T(1, t), lo(1, char(t - 'A' + 'a'));
        q.prompt = "Qual é a letra minúscula que combina com " + T + "?";
        q.hint = "A letra " + T + " também pode ser escrita assim: " + lo + ".";
        Scene s = sc("letter"); s.text = T; q.scene = s;
        std::vector<char> others; for (char c : alphabet) if (c != t) others.push_back(c);
        std::vector<char> cs{t}; for (char c : take(r.shuffle(others), 3)) cs.push_back(c);
        std::vector<Option> items;
        for (char c : cs) { std::string l(1, char(c - 'A' + 'a')); items.push_back(opt(std::string(1, c), "Letra " + l, textV(l))); }
        setOptions(q, r, T, items);
    } else if (game == "initial") {
        const Word& w = WORDS[r.below(WORDS.size())];
        char first = w.word[0];
        q.prompt = "Qual é a primeira letra de " + lowerPt(w.word) + "?";
        q.hint = std::string("Observe a primeira letra: ") + first + ".";
        Scene s = sc("word-picture"); s.word = w.word; s.icon = w.icon; q.scene = s;
        std::vector<char> others; for (char c : std::string("ABCDEFGHILMNOPRSTUV")) if (c != first) others.push_back(c);
        std::vector<char> cs{first}; for (char c : take(r.shuffle(others), 3)) cs.push_back(c);
        std::vector<Option> items; for (char c : cs) items.push_back(opt(std::string(1, c), std::string("Letra ") + c, textV(std::string(1, c))));
        setOptions(q, r, std::string(1, first), items);
    } else if (game == "vowels") {
        char v = r.pick(std::vector<char>{'A', 'E', 'I', 'O', 'U'});
        std::vector<char> cons; for (char c : std::string("BCDFGLMNPRSTV")) cons.push_back(c);
        auto others = take(r.shuffle(cons), 3);
        q.prompt = "Qual destas letras é uma vogal?";
        q.hint = "As vogais são A, E, I, O e U.";
        Scene s = sc("letter"); s.text = "A E I O U"; q.scene = s;
        std::vector<char> cs{v}; for (char c : others) cs.push_back(c);
        std::vector<Option> items; for (char c : cs) items.push_back(opt(std::string(1, c), std::string("Letra ") + c, textV(std::string(1, c))));
        setOptions(q, r, std::string(1, v), items);
    } else if (game == "syllables") {
        const Word& w = WORDS[r.below(WORDS.size())];
        q.prompt = "Quantos pedacinhos tem a palavra " + lowerPt(w.word) + "?";
        std::string joined; for (size_t i = 0; i < w.syl.size(); ++i) joined += (i ? ", " : "") + w.syl[i];
        q.hint = "Fale devagar: " + joined + ". Conte uma batida para cada pedacinho.";
        Scene s = sc("word-picture"); s.word = w.word; s.icon = w.icon; q.scene = s;
        numbers(int(w.syl.size()), 1, 5);
    } else if (game == "word") {
        size_t ti = r.below(WORDS.size());
        const Word& w = WORDS[ti];
        q.prompt = "Qual palavra combina com o desenho?";
        q.hint = "O desenho é " + lowerPt(w.word) + ". Procure essa palavra.";
        Scene s = sc("word-picture"); s.icon = w.icon; q.scene = s;
        std::vector<size_t> others; for (size_t i = 0; i < WORDS.size(); ++i) if (i != ti) others.push_back(i);
        std::vector<size_t> idx{ti}; for (auto i : take(r.shuffle(others), 3)) idx.push_back(i);
        std::vector<Option> items; for (auto i : idx) items.push_back(opt(WORDS[i].word, WORDS[i].word, textV(WORDS[i].word)));
        setOptions(q, r, w.word, items);
    } else if (game == "subtract") {
        int a = r.range(2, level == 2 ? 12 : 5), b = r.range(1, a);
        q.prompt = "Havia " + std::to_string(a) + " maçãs. Tiramos " + std::to_string(b) + ". Quantas ficaram?";
        q.hint = "As maçãs riscadas saíram do grupo. Conte as que ficaram.";
        Scene s = sc("subtract"); s.a = a; s.b = b; q.scene = s;
        numbers(a - b, 0, level == 2 ? 15 : 7);
    } else if (game == "missing") {
        int hi = level == 2 ? 9 : 3, a = r.range(1, hi), b = r.range(1, hi);
        q.prompt = "Qual número está escondido na adição?";
        q.hint = "Quanto falta para ir de " + std::to_string(a) + " até " + std::to_string(a + b) + "?";
        Scene s = sc("equation"); s.a = a; s.count = a + b; q.scene = s;
        numbers(b, 0, level == 2 ? 12 : 6);
    } else if (game == "tenframe") {
        int n = r.range(1, 10);
        q.prompt = "Quantos lugares estão ocupados na casinha?";
        q.hint = "A primeira linha tem cinco lugares. Conte os círculos preenchidos.";
        Scene s = sc("tenframe"); s.count = n; q.scene = s;
        numbers(n, 0, 12);
    } else if (game == "tens") {
        int tens = r.range(1, level == 2 ? 4 : 2), units = r.range(0, 9);
        q.prompt = "Qual número estas dezenas e unidades formam?";
        q.hint = "Cada barra tem dez quadradinhos. Há " + std::to_string(tens) + " dezenas e " + std::to_string(units) + " unidades.";
        Scene s = sc("tens"); s.a = tens; s.b = units; q.scene = s;
        numbers(tens * 10 + units, 10, 59);
    } else if (game == "clock") {
        int hour = r.range(1, 12);
        q.prompt = "Que horas o relógio está marcando?";
        q.hint = "O ponteiro grande está no doze. O ponteiro pequeno mostra a hora.";
        Scene s = sc("clock"); s.count = hour; q.scene = s;
        numbers(hour, 1, 12);
    } else if (game == "story") {
        const Story& st = STORIES[r.below(STORIES.size())];
        q.prompt = st.question;
        q.story = st.text;
        q.hint = "Ouça a história outra vez e observe o que aconteceu.";
        Scene s = sc("story"); s.text = st.text; s.icon = st.icon; q.scene = s;
        std::vector<Option> items; for (int i = 0; i < 3; ++i) items.push_back(opt(st.choices[i], st.labels[i], icon(st.choices[i])));
        setOptions(q, r, st.answer, items);
    } else return false;
    return true;
}

static Question generateBase(const std::string& game, const std::string& group, uint32_t seed) {
    Rng r(seed);
    size_t level = groupIndex(group);
    const Group& g = GROUPS[level];
    Question q; q.game = game;
    if (game == "colors") {
        size_t ti = r.below(4);
        const Color& t = COLORS[ti];
        q.prompt = std::string("Encontre a flor de cor ") + t.name + ".";
        q.hint = "Observe a cor das pétalas.";
        Scene s = sc("color"); s.color = t.hex; q.scene = s;
        std::vector<size_t> others; for (size_t i = 0; i < 4; ++i) if (i != ti) others.push_back(i);
        std::vector<size_t> idx{ti}; for (auto i : take(r.shuffle(others), level ? 3 : 2)) idx.push_back(i);
        std::vector<Option> items; for (auto i : idx) items.push_back(opt(COLORS[i].id, COLORS[i].name, icon("flower", COLORS[i].hex)));
        setOptions(q, r, t.id, items);
    } else if (game == "shapes") {
        std::vector<std::string> pool(SHAPES.begin(), SHAPES.begin() + (level ? 4 : 3));
        std::string t = r.pick(pool);
        q.prompt = "Qual forma é igual à do desenho?";
        q.hint = "Procure o " + shapeName(t) + ".";
        Scene s = sc("shape"); s.shape = t; s.color = "#559d8b"; q.scene = s;
        std::vector<std::string> others; for (auto& x : SHAPES) if (x != t) others.push_back(x);
        auto list = take(r.shuffle(others), level ? 3 : 2); list.push_back(t);
        std::vector<Option> items; for (auto& x : list) items.push_back(opt(x, shapeName(x), icon(x, "#e9a94b")));
        setOptions(q, r, t, items);
    } else if (game == "count") {
        int n = r.range(1, g.max);
        q.prompt = "Quantas maçãs você consegue contar?";
        q.hint = "Toque em cada maçã e conte devagar.";
        Scene s = sc("items"); s.count = n; q.scene = s;
        std::vector<Option> items{numOpt(n)};
        for (int v : take(r.shuffle(rangeInts(1, g.max + 2, n)), level ? 3 : 2)) items.push_back(numOpt(v));
        setOptions(q, r, std::to_string(n), items);
    } else if (game == "memory") {
        q.memory = true;
        q.prompt = "Encontre os pares que são iguais.";
        q.hint = "Vire duas cartas. Lembre onde cada desenho está.";
        auto kinds = take(r.shuffle(std::vector<std::string>{"apple", "bird", "flower", "sun"}), g.pairs);
        std::vector<Card> cards; for (size_t p = 0; p < kinds.size(); ++p) { cards.push_back({kinds[p], p}); cards.push_back({kinds[p], p}); }
        q.cards = r.shuffle(cards);
    } else if (game == "pattern") {
        std::string a = r.pick(SHAPES);
        std::vector<std::string> others; for (auto& x : SHAPES) if (x != a) others.push_back(x);
        std::string b = r.pick(others);
        std::vector<std::string> seq = level == 2
            ? r.pick(std::vector<std::vector<std::string>>{{a, a, b, a, a, b}, {a, b, b, a, b, b}, {a, b, a, b, a, b}})
            : std::vector<std::string>{a, b, a, b};
        std::string target = seq.back();
        q.prompt = "Qual desenho completa a sequência?";
        q.hint = "Diga os nomes das formas e procure a repetição.";
        Scene s = sc("sequence"); s.items.assign(seq.begin(), seq.end() - 1); q.scene = s;
        std::vector<Option> items; for (auto& x : SHAPES) items.push_back(opt(x, shapeName(x), icon(x, "#8b7bbd")));
        setOptions(q, r, target, items);
    } else if (game == "odd") {
        std::string normal = r.pick(SHAPES);
        std::vector<std::string> others; for (auto& x : SHAPES) if (x != normal) others.push_back(x);
        std::string other = r.pick(others);
        int size = level ? 4 : 3;
        int index = r.range(0, size - 1);
        q.prompt = "Qual desenho tem uma forma diferente?";
        q.hint = "A cor é igual. Compare o contorno dos desenhos.";
        q.scene = sc("look");
        for (int i = 0; i < size; ++i) {
            std::string s = i == index ? other : normal;
            q.options.push_back(opt(std::to_string(i), "Desenho " + std::to_string(i + 1) + ": " + shapeName(s), icon(s, "#599b84")));
        }
        q.answer = size_t(index);
    } else if (game == "sum") {
        int hi = level == 2 ? 9 : 3, a = r.range(1, hi), b = r.range(1, hi), n = a + b;
        q.prompt = "Juntando os dois grupos, quantas maçãs temos?";
        q.hint = "Conte as maçãs de um grupo e continue no outro.";
        Scene s = sc("sum"); s.a = a; s.b = b; q.scene = s;
        std::vector<Option> items{numOpt(n)};
        for (int v : take(r.shuffle(rangeInts(1, std::max(n + 3, 10), n)), 3)) items.push_back(numOpt(v));
        setOptions(q, r, std::to_string(n), items);
    } else if (game == "order") {
        int step = level == 2 ? r.pick(std::vector<int>{1, 2, 5}) : 1;
        int start = r.range(1, level == 2 ? 10 : 4);
        int index = r.range(1, 3);
        std::vector<int> seq; for (int i = 0; i < 4; ++i) seq.push_back(start + i * step);
        int n = seq[index];
        q.prompt = "Qual número falta na escada?";
        q.hint = step == 1 ? "Conte os números em ordem." : "Conte de " + std::to_string(step) + " em " + std::to_string(step) + ".";
        Scene s = sc("numbers"); for (int i = 0; i < 4; ++i) s.numbers.push_back(i == index ? std::nullopt : std::optional<int>(seq[i])); q.scene = s;
        std::vector<Option> items{numOpt(n)};
        for (int v : take(r.shuffle(rangeInts(1, seq[3] + 3, n)), 3)) items.push_back(numOpt(v));
        setOptions(q, r, std::to_string(n), items);
    } else if (!generateToddler(q, game, level, seed) && !generateExtra(q, game, level, seed)) {
        q.prompt = "Atividade desconhecida";
    }
    return q;
}

static bool isShape(const std::string& k) { return std::find(SHAPES.begin(), SHAPES.end(), k) != SHAPES.end(); }

static Question vary(Question q, uint32_t seed) {  // Variation.varyQuestion
    Rng r(seed ^ 0xa4b1c987u);
    const std::vector<std::string> palette{"#e97e68", "#5799be", "#e0b849", "#68a585", "#9b86bd"};
    auto& p = q.presentation;
    p.surface = r.pick(std::vector<std::string>{"#fffaf0", "#f2f7ed", "#eff6f7", "#faf1ed", "#f5f0f8"});
    p.radius = 20 + int(r.below(10));
    p.itemColor = r.pick(std::vector<std::string>{"#e87961", "#83ad68", "#e6b85c"});
    p.itemGap = 4 + int(r.below(5));
    for (auto& x : p.rotation) x = int(r.below(17)) - 8;
    if (q.game == "colors") {
        for (auto& o : q.options) {
            std::vector<std::string> shades =
                o.id == "coral" ? std::vector<std::string>{"#e46b5b", "#df6357", "#e67a69"} :
                o.id == "blue" ? std::vector<std::string>{"#548fc5", "#5b9cce", "#478bbb"} :
                o.id == "yellow" ? std::vector<std::string>{"#edc14c", "#e6b73e", "#efc656"} :
                                   std::vector<std::string>{"#60a277", "#6caa80", "#56976c"};
            o.visual.color = r.pick(shades);
        }
        if (q.answer < q.options.size() && q.scene) q.scene->color = q.options[q.answer].visual.color;
    }
    std::string shared = r.pick(palette);
    bool odd = q.game == "odd";
    for (auto& o : q.options) {
        if (o.visual.type == "icon" && isShape(o.visual.kind)) o.visual.color = odd ? shared : r.pick(palette);
        else if (o.visual.type == "sized") {
            double base = o.visual.number;
            o.visual.number = base < 0.5 ? 0.4 + r.next() * 0.08 : base < 0.8 ? 0.63 + r.next() * 0.08 : 0.91 + r.next() * 0.08;
            o.visual.color = shared;
        }
    }
    if (q.game != "shadow" && q.scene && q.scene->type == "shape") q.scene->color = r.pick(palette);
    return q;
}

Question generate(const std::string& game, const std::string& group, uint32_t seed) {
    return vary(generateBase(game, group, seed), seed);
}

std::vector<const Game*> catalog(size_t level, uint32_t seed) {
    std::vector<const Game*> avail;
    for (auto& g : GAMES) if (g.min <= level) avail.push_back(&g);
    Rng r(seed);
    return r.shuffle(avail);
}

}  // namespace jardim
