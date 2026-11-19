// Gera as mesmas 96 perguntas (32 jogos × 3 faixas, semente 12345) que
// tests/gen_reference.js gerou com o JS original, e compara.
#include "../src/curriculum.h"
#include <cstdio>
#include <fstream>
#include <regex>
#include <sstream>

using namespace jardim;

static std::string field(const std::string& json, const std::string& key, const std::string& name) {
    // extrai "key":{..."name":valor...} sem dependência de biblioteca JSON
    auto k = json.find("\"" + key + "\":{");
    if (k == std::string::npos) return "?";
    auto end = json.find('}', k);
    std::string obj = json.substr(k, end - k);
    std::smatch m;
    std::regex re("\"" + name + "\":(\"([^\"]*)\"|([0-9]+))");
    if (!std::regex_search(obj, m, re)) return "?";
    return m[2].matched ? m[2].str() : m[3].str();
}

int main(int argc, char** argv) {
    std::ifstream f(argc > 1 ? argv[1] : "tests/js_reference.json");
    std::stringstream ss; ss << f.rdbuf();
    std::string ref = ss.str();
    int ok = 0, bad = 0;
    // valores conhecidos do RNG/hash (iguais aos testes da versão Rust)
    Rng r(42);
    if (r.next() != 0.6011037519201636 || r.next() != 0.44829055899754167 || r.next() != 0.8524657934904099) { puts("RNG difere"); return 1; }
    if (hash("a|b") != 692878806u || seedFor("2026-11-19", "little", "colors", 1, 7) != 987461503u) { puts("hash difere"); return 1; }
    for (auto& g : GAMES)
        for (const char* grp : {"little", "middle", "big"}) {
            Question q = generate(g.id, grp, 12345);
            std::string key = std::string(g.id) + ":" + grp;
            size_t n = q.memory ? q.cards.size() : q.options.size();
            bool same = field(ref, key, "prompt") == q.prompt && field(ref, key, "n") == std::to_string(n) &&
                        field(ref, key, "surface") == q.presentation.surface &&
                        (q.memory || field(ref, key, "answer") == std::to_string(q.answer));
            if (same) ++ok; else { ++bad; printf("DIFERE %s: '%s' n=%zu ans=%zu\n", key.c_str(), q.prompt.c_str(), n, q.answer); }
        }
    printf("paridade: %d/96 iguais ao JS original\n", ok);
    return bad ? 1 : 0;
}
