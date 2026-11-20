/* Original matching and spatial games for toddlers. MIT. */
(function (root) {
  "use strict";
  const OBJECTS = [
    { id: "apple", name: "a maçã" },
    { id: "flower", name: "a flor" },
    { id: "bird", name: "o passarinho" },
    { id: "bunny", name: "o coelho" },
    { id: "cat", name: "o gato" },
    { id: "fish", name: "o peixe" },
    { id: "ball", name: "a bola" },
    { id: "moon", name: "a lua" },
    { id: "house", name: "a casa" },
    { id: "sun", name: "o sol" },
  ];
  const TODDLER_GAMES = [
    {
      id: "twins",
      name: "Encontre o igual",
      skill: "Associar objetos iguais",
      icon: "memory",
      color: "green",
      min: 0,
      category: "logic",
    },
    {
      id: "inside",
      name: "Dentro ou fora?",
      skill: "Explorar posições",
      icon: "container",
      color: "blue",
      min: 0,
      category: "space",
    },
    {
      id: "fill",
      name: "Cheio ou vazio?",
      skill: "Comparar recipientes",
      icon: "cup",
      color: "yellow",
      min: 0,
      category: "space",
    },
    {
      id: "tower",
      name: "Torres coloridas",
      skill: "Copiar um modelo",
      icon: "tower",
      color: "purple",
      min: 0,
      category: "logic",
    },
    {
      id: "socks",
      name: "Meias companheiras",
      skill: "Combinar cores e detalhes",
      icon: "socks",
      color: "pink",
      min: 0,
      category: "logic",
    },
    {
      id: "oneeach",
      name: "Um para cada amigo",
      skill: "Relacionar um a um",
      icon: "count",
      color: "peach",
      min: 0,
      category: "math",
    },
    {
      id: "vocabulary",
      name: "Quem foi chamado?",
      skill: "Ouvir e reconhecer",
      icon: "book",
      color: "green",
      min: 0,
      category: "language",
    },
    {
      id: "colorpath",
      name: "Caminho das cores",
      skill: "Continuar uma repetição",
      icon: "colorpath",
      color: "pink",
      min: 0,
      category: "logic",
    },
  ];
  function generateToddler(game, group, seed, helpers) {
    const { random, shuffle, COLORS, GROUPS } = helpers;
    const rng = random(seed),
      pick = (xs) => xs[Math.floor(rng() * xs.length)];
    const g = GROUPS.find((x) => x.id === group) || GROUPS[0],
      level = GROUPS.indexOf(g);
    const q = { game, group: g.id, seed, mode: "choice" };
    const choices = level === 0 ? 2 : 3;
    const options = (id, items) => {
      q.options = shuffle(items, rng);
      q.answer = q.options.findIndex((x) => x.id === id);
    };
    if (game === "twins" || game === "vocabulary") {
      const target = pick(OBJECTS);
      q.target = target.id;
      q.prompt =
        game === "twins"
          ? "Qual desenho é igual ao modelo?"
          : `Encontre ${target.name}.`;
      q.hint =
        game === "twins"
          ? "Olhe o desenho grande e procure outro igual."
          : `Vamos procurar ${target.name} juntos.`;
      q.scene =
        game === "twins"
          ? { type: "object-model", icon: target.id }
          : { type: "look" };
      options(
        target.id,
        [
          target,
          ...shuffle(
            OBJECTS.filter((o) => o.id !== target.id),
            rng,
          ).slice(0, choices - 1),
        ].map((o) => ({ id: o.id, type: o.id, label: o.name })),
      );
    } else if (game === "inside") {
      const target = pick(["inside", "outside"]);
      q.target = target;
      q.prompt =
        target === "inside"
          ? "Em qual desenho a flor está dentro da caixa?"
          : "Em qual desenho a flor está fora da caixa?";
      q.hint =
        target === "inside"
          ? "Procure a flor no espaço de dentro da caixa."
          : "Procure a flor ao lado de fora da caixa.";
      q.scene = { type: "look" };
      options(
        target,
        ["inside", "outside"].map((place) => ({
          id: place,
          type: "container",
          place,
          label:
            place === "inside" ? "Flor dentro da caixa" : "Flor fora da caixa",
        })),
      );
    } else if (game === "fill") {
      const target = pick([0, 1]);
      q.target = target;
      q.prompt =
        target === 1 ? "Qual copo está cheio?" : "Qual copo está vazio?";
      q.hint =
        target === 1
          ? "Procure o copo com água até perto da borda."
          : "Procure o copo sem água.";
      q.scene = { type: "look" };
      options(
        String(target),
        (level === 0 ? [0, 1] : [0, 0.5, 1]).map((amount) => ({
          id: String(amount),
          type: "cup",
          amount,
          label:
            amount === 1
              ? "Copo cheio"
              : amount === 0
                ? "Copo vazio"
                : "Copo pela metade",
        })),
      );
    } else if (game === "tower") {
      const palette = shuffle(COLORS, rng)
        .slice(0, level === 0 ? 2 : 3)
        .map((c) => c.hex);
      q.prompt = "Qual torre é igual à torre do modelo?";
      q.hint = "Olhe a cor dos blocos de baixo para cima.";
      q.scene = { type: "tower", colors: palette };
      const variants = [palette, palette.slice().reverse()];
      if (choices === 3) variants.push([palette[1], palette[0], palette[2]]);
      options(
        palette.join(":"),
        variants.map((colors, i) => ({
          id: colors.join(":"),
          type: "tower",
          colors,
          label: `Torre ${i + 1}`,
        })),
      );
    } else if (game === "socks") {
      const color = pick(COLORS).hex,
        other = pick(COLORS.filter((c) => c.hex !== color)).hex,
        pattern = pick(["dots", "stripes"]);
      q.prompt = "Qual meia faz par com esta?";
      q.hint = "Procure a mesma cor e o mesmo desenho na meia.";
      q.scene = { type: "sock", color, pattern };
      const variants = [
        { color, pattern },
        { color: other, pattern },
      ];
      if (choices === 3)
        variants.push({
          color,
          pattern: pattern === "dots" ? "stripes" : "dots",
        });
      options(
        color + pattern,
        variants.map((s, i) => ({
          id: s.color + s.pattern,
          type: "sock",
          ...s,
          label: `Meia ${i + 1}`,
        })),
      );
    } else if (game === "oneeach") {
      const n = 1 + Math.floor(rng() * (level === 0 ? 3 : 5));
      q.target = n;
      q.prompt = "Qual grupo tem uma maçã para cada coelho?";
      q.hint = "Dê uma maçã para um coelho. Depois, uma para o próximo.";
      q.scene = { type: "friends", count: n };
      const rest = shuffle(
        Array.from({ length: level === 0 ? 3 : 6 }, (_, i) => i + 1).filter(
          (x) => x !== n,
        ),
        rng,
      ).slice(0, choices - 1);
      options(
        String(n),
        [n, ...rest].map((count) => ({
          id: String(count),
          type: "quantity",
          count,
          label: `Grupo com ${count} maçãs`,
        })),
      );
    } else if (game === "colorpath") {
      const [a, b] = shuffle(COLORS, rng).slice(0, 2);
      q.target = b.hex;
      q.prompt = "Qual cor continua o caminho?";
      q.hint = "As cores aparecem uma de cada vez e depois se repetem.";
      q.scene = { type: "color-sequence", colors: [a.hex, b.hex, a.hex] };
      options(
        b.id,
        [a, b].map((c) => ({
          id: c.id,
          type: "swatch",
          color: c.hex,
          label: `Cor ${c.name}`,
        })),
      );
    } else return null;
    return q;
  }
  const api = { OBJECTS, TODDLER_GAMES, generateToddler };
  if (typeof module !== "undefined") module.exports = api;
  else root.ToddlerCurriculum = api;
})(typeof window !== "undefined" ? window : globalThis);
