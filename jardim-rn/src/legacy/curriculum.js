/* Original procedural activities. No LogicLike code or content. MIT license. */
(function (root) {
  "use strict";
  const extra =
    typeof module !== "undefined"
      ? require("./extra-curriculum.js")
      : root.ExtraCurriculum;
  const toddler =
    typeof module !== "undefined"
      ? require("./toddler-curriculum.js")
      : root.ToddlerCurriculum;
  const variation =
    typeof module !== "undefined" ? require("./variation.js") : root.Variation;
  const COLORS = [
    { id: "coral", name: "vermelha", hex: "#eb6d58" },
    { id: "blue", name: "azul", hex: "#589cd4" },
    { id: "yellow", name: "amarela", hex: "#efbd46" },
    { id: "green", name: "verde", hex: "#60a786" },
  ];
  const SHAPES = ["circle", "triangle", "square", "star"];
  const NAMES = {
    circle: "círculo",
    triangle: "triângulo",
    square: "quadrado",
    star: "estrela",
  };
  const GROUPS = [
    {
      id: "little",
      label: "2–3 anos",
      name: "Primeiros passos",
      max: 3,
      pairs: 2,
    },
    {
      id: "middle",
      label: "4–5 anos",
      name: "Mentes curiosas",
      max: 6,
      pairs: 3,
    },
    { id: "big", label: "6–8 anos", name: "Grandes ideias", max: 12, pairs: 4 },
  ];
  const GAMES = [
    {
      id: "colors",
      name: "Jardim das cores",
      skill: "Reconhecer e associar",
      icon: "flower",
      color: "pink",
      min: 0,
    },
    {
      id: "shapes",
      name: "Formas pelo caminho",
      skill: "Percepção visual",
      icon: "shapes",
      color: "blue",
      min: 0,
    },
    {
      id: "count",
      name: "Vamos contar?",
      skill: "Números e quantidades",
      icon: "count",
      color: "yellow",
      min: 0,
    },
    {
      id: "memory",
      name: "Pares da floresta",
      skill: "Memória e atenção",
      icon: "memory",
      color: "green",
      min: 0,
    },
    {
      id: "pattern",
      name: "O que vem depois?",
      skill: "Sequências e lógica",
      icon: "pattern",
      color: "purple",
      min: 1,
    },
    {
      id: "odd",
      name: "Um é diferente",
      skill: "Observar e comparar",
      icon: "odd",
      color: "peach",
      min: 0,
    },
    {
      id: "sum",
      name: "Somar sorrisos",
      skill: "Primeiras operações",
      icon: "sum",
      color: "yellow",
      min: 1,
    },
    {
      id: "order",
      name: "Escada dos números",
      skill: "Ordem e raciocínio",
      icon: "order",
      color: "blue",
      min: 1,
    },
  ];
  const CATEGORIES = [
    { id: "all", label: "Todas" },
    { id: "math", label: "Matemática" },
    { id: "logic", label: "Lógica" },
    { id: "space", label: "Formas e espaço" },
    { id: "language", label: "Letras e histórias" },
  ];
  const categoryMap = {
    colors: "space",
    shapes: "space",
    count: "math",
    memory: "logic",
    pattern: "logic",
    odd: "logic",
    sum: "math",
    order: "math",
  };
  GAMES.forEach((g) => (g.category = categoryMap[g.id]));
  GAMES.push(...extra.EXTRA_GAMES, ...toddler.TODDLER_GAMES);
  function random(seed) {
    let a = seed | 0;
    return () => {
      a |= 0;
      a = (a + 0x6d2b79f5) | 0;
      let t = Math.imul(a ^ (a >>> 15), 1 | a);
      t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
      return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
    };
  }
  function shuffle(xs, rng) {
    const a = xs.slice();
    for (let i = a.length - 1; i > 0; i--) {
      const j = Math.floor(rng() * (i + 1));
      [a[i], a[j]] = [a[j], a[i]];
    }
    return a;
  }
  function generateBase(game, group, seed) {
    const rng = random(seed),
      pick = (a) => a[Math.floor(rng() * a.length)],
      int = (lo, hi) => lo + Math.floor(rng() * (hi - lo + 1));
    const g = GROUPS.find((x) => x.id === group) || GROUPS[0],
      level = GROUPS.indexOf(g);
    let q = { game, seed, group: g.id, mode: "choice", hint: "" };
    const options = (correct, items) => {
      q.options = shuffle(items, rng);
      q.answer = q.options.findIndex((x) => x.id === correct);
    };
    if (game === "colors") {
      const target = pick(COLORS);
      q.prompt = `Encontre a flor de cor ${target.name}.`;
      q.hint = "Observe a cor das pétalas.";
      q.scene = { type: "color", color: target.hex };
      const rest = shuffle(
        COLORS.filter((c) => c.id !== target.id),
        rng,
      ).slice(0, level ? 3 : 2);
      options(
        target.id,
        [target, ...rest].map((c) => ({
          id: c.id,
          type: "flower",
          color: c.hex,
          label: c.name,
        })),
      );
    } else if (game === "shapes") {
      const target = pick(SHAPES.slice(0, level ? 4 : 3));
      q.prompt = "Qual forma é igual à do desenho?";
      q.hint = `Procure o ${NAMES[target]}.`;
      q.scene = { type: "shape", shape: target, color: "#559d8b" };
      options(
        target,
        shuffle(
          SHAPES.filter((s) => s !== target),
          rng,
        )
          .slice(0, level ? 3 : 2)
          .concat(target)
          .map((s) => ({
            id: s,
            type: "shape",
            shape: s,
            color: "#e9a94b",
            label: NAMES[s],
          })),
      );
    } else if (game === "count") {
      const n = int(1, g.max);
      q.prompt = "Quantas maçãs você consegue contar?";
      q.hint = "Toque em cada maçã e conte devagar.";
      q.scene = { type: "items", item: "apple", count: n };
      const pool = shuffle(
        Array.from({ length: g.max + 2 }, (_, i) => i + 1).filter(
          (i) => i !== n,
        ),
        rng,
      ).slice(0, level ? 3 : 2);
      options(
        String(n),
        [n, ...pool].map((v) => ({
          id: String(v),
          type: "number",
          value: v,
          label: String(v),
        })),
      );
    } else if (game === "memory") {
      q.mode = "memory";
      q.prompt = "Encontre os pares que são iguais.";
      q.hint = "Vire duas cartas. Lembre onde cada desenho está.";
      const kinds = shuffle(["apple", "bird", "flower", "sun"], rng).slice(
        0,
        g.pairs,
      );
      q.cards = shuffle(
        kinds.flatMap((kind, pair) => [
          { id: pair + "a", kind, pair },
          { id: pair + "b", kind, pair },
        ]),
        rng,
      );
    } else if (game === "pattern") {
      const a = pick(SHAPES),
        b = pick(SHAPES.filter((s) => s !== a));
      const seq =
        level === 2
          ? pick([
              [a, a, b, a, a, b],
              [a, b, b, a, b, b],
              [a, b, a, b, a, b],
            ])
          : [a, b, a, b];
      const target = seq[seq.length - 1];
      q.prompt = "Qual desenho completa a sequência?";
      q.hint = "Diga os nomes das formas e procure a repetição.";
      q.scene = { type: "sequence", items: seq.slice(0, -1) };
      options(
        target,
        SHAPES.map((s) => ({
          id: s,
          type: "shape",
          shape: s,
          color: "#8b7bbd",
          label: NAMES[s],
        })),
      );
    } else if (game === "odd") {
      const normal = pick(SHAPES),
        other = pick(SHAPES.filter((s) => s !== normal)),
        size = level ? 4 : 3,
        index = int(0, size - 1);
      q.prompt = "Qual desenho tem uma forma diferente?";
      q.hint = "A cor é igual. Compare o contorno dos desenhos.";
      q.scene = { type: "look" };
      q.options = Array.from({ length: size }, (_, i) => ({
        id: String(i),
        type: "shape",
        shape: i === index ? other : normal,
        color: "#599b84",
        label: `Desenho ${i + 1}: ${NAMES[i === index ? other : normal]}`,
      }));
      q.answer = index;
    } else if (game === "sum") {
      const a = int(1, level === 2 ? 9 : 3),
        b = int(1, level === 2 ? 9 : 3),
        n = a + b;
      q.prompt = "Juntando os dois grupos, quantas maçãs temos?";
      q.hint = "Conte as maçãs de um grupo e continue no outro.";
      q.scene = { type: "sum", a, b };
      const pool = shuffle(
        Array.from({ length: Math.max(n + 3, 10) }, (_, i) => i + 1).filter(
          (i) => i !== n,
        ),
        rng,
      ).slice(0, 3);
      options(
        String(n),
        [n, ...pool].map((v) => ({
          id: String(v),
          type: "number",
          value: v,
          label: String(v),
        })),
      );
    } else if (game === "order") {
      const step = level === 2 ? pick([1, 2, 5]) : 1,
        start = int(1, level === 2 ? 10 : 4),
        index = int(1, 3),
        seq = Array.from({ length: 4 }, (_, i) => start + i * step),
        n = seq[index];
      q.prompt = "Qual número falta na escada?";
      q.hint =
        step === 1
          ? "Conte os números em ordem."
          : `Conte de ${step} em ${step}.`;
      q.scene = {
        type: "numbers",
        items: seq.map((v, i) => (i === index ? null : v)),
      };
      const pool = shuffle(
        Array.from({ length: seq[3] + 3 }, (_, i) => i + 1).filter(
          (i) => i !== n,
        ),
        rng,
      ).slice(0, 3);
      options(
        String(n),
        [n, ...pool].map((v) => ({
          id: String(v),
          type: "number",
          value: v,
          label: String(v),
        })),
      );
    } else {
      const toddlerResult = toddler.generateToddler(game, group, seed, {
        random,
        shuffle,
        COLORS,
        GROUPS,
      });
      if (toddlerResult) return toddlerResult;
      const result = extra.generateExtra(game, group, seed, {
        random,
        shuffle,
        COLORS,
        SHAPES,
        NAMES,
        GROUPS,
      });
      if (result) return result;
      throw new Error("Unknown game: " + game);
    }
    return q;
  }
  function generate(game, group, seed) {
    return variation.varyQuestion(
      generateBase(game, group, seed),
      seed,
      random,
    );
  }
  const api = {
    COLORS,
    SHAPES,
    NAMES,
    GROUPS,
    GAMES,
    CATEGORIES,
    WORDS: extra.WORDS,
    STORIES: extra.STORIES,
    generate,
    shuffle,
    random,
  };
  if (typeof module !== "undefined") module.exports = api;
  else root.Curriculum = api;
})(typeof window !== "undefined" ? window : globalThis);
