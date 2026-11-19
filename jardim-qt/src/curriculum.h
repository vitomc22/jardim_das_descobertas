// Port de curriculum.js + extra + toddler + variation.js (C++17, sem Qt).
// A ordem das chamadas ao RNG é a mesma do JS: a mesma semente gera a mesma
// pergunta (verificado por tests/parity.cpp contra tests/js_reference.json).
#pragma once
#include <array>
#include <cstdint>
#include <optional>
#include <string>
#include <vector>

namespace jardim {

class Rng {  // mulberry32, igual a Curriculum.random(seed)
public:
    explicit Rng(uint32_t seed) : s_(seed) {}
    double next() {
        s_ += 0x6d2b79f5u;
        uint32_t a = s_;
        uint32_t t = (a ^ (a >> 15)) * (1u | a);
        t = (t + ((t ^ (t >> 7)) * (61u | t))) ^ t;
        return double(t ^ (t >> 14)) / 4294967296.0;
    }
    size_t below(size_t n) { return size_t(next() * double(n)); }
    int range(int lo, int hi) { return lo + int(next() * double(hi - lo + 1)); }
    template <class T> T pick(const std::vector<T>& xs) { return xs[below(xs.size())]; }
    template <class T> std::vector<T> shuffle(std::vector<T> a) {
        for (size_t i = a.size(); i-- > 1;) {
            size_t j = size_t(next() * double(i + 1));
            std::swap(a[i], a[j]);
        }
        return a;
    }
private:
    uint32_t s_;
};

uint32_t hash(const std::string& utf8);  // FNV-1a sobre unidades UTF-16 (como o JS)
uint32_t seedFor(const std::string& day, const std::string& group, const std::string& game, uint64_t round, uint32_t entry);
uint32_t entrySeed();

struct Group { const char* id; const char* label; const char* name; int max; size_t pairs; };
struct Game { const char* id; const char* name; const char* skill; const char* icon; const char* color; size_t min; const char* category; };

extern const std::array<Group, 3> GROUPS;
extern const std::array<Game, 32> GAMES;
extern const std::array<std::pair<const char*, const char*>, 5> CATEGORIES;
size_t groupIndex(const std::string& id);
const Game& gameById(const std::string& id);

// Visual de uma opção. `kind` decide como desenhar:
//  icon(kind,color) | number | text | swatch | quantity | sized | position |
//  sock | cup | container | tower
struct Visual {
    std::string type;      // "icon", "number", "text", "swatch", "quantity", "sized", "position", "sock", "cup", "container", "tower"
    std::string kind;      // nome do ícone / forma / direção / lugar / padrão da meia
    std::string color;     // "#rrggbb" ou vazio
    std::vector<std::string> colors;  // torre
    double number = 0;     // número, quantidade, tamanho, nível do copo
    std::string text;
};

struct Option { std::string id, label; Visual visual; };

struct Scene {
    std::string type;  // color shape items sum look sequence numbers object-model sock tower friends color-sequence letter word-picture subtract equation tenframe tens clock story
    std::string color, shape, icon, text, word, pattern;
    std::vector<std::string> items;           // sequence / colors
    std::vector<std::optional<int>> numbers;  // escada
    int a = 0, b = 0, count = 0;
};

struct Card { std::string kind; size_t pair; };

struct Presentation {
    std::string surface = "#fffaf0", itemColor = "#e87961";
    int radius = 20, itemGap = 4;
    std::array<int, 24> rotation{};
};

struct Question {
    std::string game, prompt, hint, story;
    bool memory = false;
    std::vector<Option> options;
    size_t answer = 0;
    std::vector<Card> cards;
    std::optional<Scene> scene;
    Presentation presentation;
};

Question generate(const std::string& game, const std::string& group, uint32_t seed);
std::vector<const Game*> catalog(size_t level, uint32_t seed);
std::string shapeName(const std::string& s);

}  // namespace jardim
