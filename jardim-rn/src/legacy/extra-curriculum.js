/* Original Portuguese activities. MIT. References inform topic selection only. */
(function (root) {
  "use strict";
  const WORDS = [
    { word: "SOL", icon: "sun", syllables: ["SOL"] },
    { word: "FLOR", icon: "flower", syllables: ["FLOR"] },
    { word: "MAÇÃ", icon: "apple", syllables: ["MA", "ÇÃ"] },
    { word: "GATO", icon: "cat", syllables: ["GA", "TO"] },
    { word: "BOLA", icon: "ball", syllables: ["BO", "LA"] },
    { word: "LUA", icon: "moon", syllables: ["LU", "A"] },
    { word: "CASA", icon: "house", syllables: ["CA", "SA"] },
    { word: "PEIXE", icon: "fish", syllables: ["PEI", "XE"] },
    { word: "COELHO", icon: "bunny", syllables: ["CO", "E", "LHO"] },
    { word: "PÁSSARO", icon: "bird", syllables: ["PÁS", "SA", "RO"] },
  ];
  const STORIES = [
    {
      text: "Nina, a coelha, encontrou uma maçã no jardim. Ela dividiu a maçã com seu amigo passarinho.",
      question: "Qual fruta Nina dividiu?",
      icon: "bunny",
      answer: "apple",
      choices: ["apple", "flower", "ball"],
      labels: ["maçã", "flor", "bola"],
    },
    {
      text: "O passarinho Tito recolheu uma flor amarela. Depois, levou a flor para perto de sua casa.",
      question: "O que Tito recolheu?",
      icon: "bird",
      answer: "flower",
      choices: ["flower", "apple", "fish"],
      labels: ["flor", "maçã", "peixe"],
    },
    {
      text: "O gato Bento brincava com uma bola. Quando o sol apareceu, Bento levou a bola para o jardim.",
      question: "Com o que Bento brincava?",
      icon: "cat",
      answer: "ball",
      choices: ["ball", "apple", "flower"],
      labels: ["bola", "maçã", "flor"],
    },
    {
      text: "A lua apareceu no céu. Nina, a coelha, guardou seus brinquedos e foi descansar.",
      question: "O que apareceu no céu?",
      icon: "bunny",
      answer: "moon",
      choices: ["moon", "sun", "flower"],
      labels: ["lua", "sol", "flor"],
    },
    {
      text: "Tito viu um peixe nadando no lago. O peixe passou por uma pedra e mergulhou devagar.",
      question: "Qual animal estava no lago?",
      icon: "bird",
      answer: "fish",
      choices: ["fish", "cat", "bunny"],
      labels: ["peixe", "gato", "coelho"],
    },
    {
      text: "Bento encontrou Nina em frente à casa. O gato e a coelha entraram juntos para brincar.",
      question: "Onde os amigos entraram?",
      icon: "cat",
      answer: "house",
      choices: ["house", "flower", "apple"],
      labels: ["casa", "flor", "maçã"],
    },
  ];
  const EXTRA_GAMES = [
    {
      id: "compare",
      name: "Onde tem mais?",
      skill: "Comparar quantidades",
      icon: "count",
      color: "yellow",
      min: 0,
      category: "math",
    },
    {
      id: "sizes",
      name: "Grande ou pequeno?",
      skill: "Comparar tamanhos",
      icon: "shapes",
      color: "blue",
      min: 0,
      category: "space",
    },
    {
      id: "position",
      name: "Onde está a flor?",
      skill: "Posição no espaço",
      icon: "flower",
      color: "green",
      min: 0,
      category: "space",
    },
    {
      id: "shadow",
      name: "De quem é a sombra?",
      skill: "Contornos e associação",
      icon: "shapes",
      color: "purple",
      min: 0,
      category: "space",
    },
    {
      id: "sort",
      name: "Cada um no seu grupo",
      skill: "Classificar e relacionar",
      icon: "memory",
      color: "green",
      min: 0,
      category: "logic",
    },
    {
      id: "letters",
      name: "Letras companheiras",
      skill: "Maiúsculas e minúsculas",
      icon: "letters",
      color: "pink",
      min: 1,
      category: "language",
    },
    {
      id: "initial",
      name: "Começa com qual letra?",
      skill: "Letra inicial das palavras",
      icon: "letters",
      color: "peach",
      min: 1,
      category: "language",
    },
    {
      id: "vowels",
      name: "Jardim das vogais",
      skill: "Reconhecer as vogais",
      icon: "letters",
      color: "pink",
      min: 1,
      category: "language",
    },
    {
      id: "syllables",
      name: "Palavras em pedacinhos",
      skill: "Contar sílabas",
      icon: "letters",
      color: "purple",
      min: 2,
      category: "language",
    },
    {
      id: "word",
      name: "Palavra e desenho",
      skill: "Leitura de palavras",
      icon: "letters",
      color: "peach",
      min: 2,
      category: "language",
    },
    {
      id: "subtract",
      name: "Quantas ficaram?",
      skill: "Subtração com imagens",
      icon: "count",
      color: "yellow",
      min: 1,
      category: "math",
    },
    {
      id: "missing",
      name: "O número escondido",
      skill: "Completar uma adição",
      icon: "sum",
      color: "yellow",
      min: 2,
      category: "math",
    },
    {
      id: "tenframe",
      name: "Casinha dos dez",
      skill: "Quantidades até dez",
      icon: "count",
      color: "blue",
      min: 1,
      category: "math",
    },
    {
      id: "tens",
      name: "Dez de cada vez",
      skill: "Dezenas e unidades",
      icon: "order",
      color: "yellow",
      min: 2,
      category: "math",
    },
    {
      id: "clock",
      name: "Que horas são?",
      skill: "Horas inteiras no relógio",
      icon: "clock",
      color: "blue",
      min: 2,
      category: "math",
    },
    {
      id: "story",
      name: "Histórias do jardim",
      skill: "Escutar e compreender",
      icon: "book",
      color: "green",
      min: 1,
      category: "language",
    },
  ];
  function generateExtra(game, group, seed, helpers) {
    const { random, shuffle, COLORS, SHAPES, NAMES, GROUPS } = helpers;
    const rng = random(seed),
      pick = (a) => a[Math.floor(rng() * a.length)],
      int = (lo, hi) => lo + Math.floor(rng() * (hi - lo + 1));
    const g = GROUPS.find((x) => x.id === group) || GROUPS[0],
      level = GROUPS.indexOf(g);
    const q = { game, group: g.id, seed, mode: "choice", hint: "" };
    const options = (id, items) => {
      q.options = shuffle(items, rng);
      q.answer = q.options.findIndex((x) => x.id === id);
    };
    const numbers = (answer, lo = 0, hi = Math.max(10, answer + 3)) => {
      const rest = shuffle(
        Array.from({ length: hi - lo + 1 }, (_, i) => i + lo).filter(
          (n) => n !== answer,
        ),
        rng,
      ).slice(0, level ? 3 : 2);
      options(
        String(answer),
        [answer, ...rest].map((n) => ({
          id: String(n),
          type: "number",
          value: n,
          label: String(n),
        })),
      );
    };
    if (game === "compare") {
      const counts = shuffle(
          Array.from(
            { length: level === 0 ? 3 : level === 1 ? 6 : 10 },
            (_, i) => i + 1,
          ),
          rng,
        ).slice(0, 3),
        want = pick(["more", "less"]);
      const target =
        want === "more" ? Math.max(...counts) : Math.min(...counts);
      q.prompt =
        want === "more"
          ? "Qual grupo tem mais maçãs?"
          : "Qual grupo tem menos maçãs?";
      q.hint = "Conte as maçãs de cada grupo e compare.";
      q.scene = { type: "look" };
      q.rule = want;
      options(
        String(target),
        counts.map((n) => ({
          id: String(n),
          type: "quantity",
          count: n,
          label: `Grupo com ${n} maçãs`,
        })),
      );
    } else if (game === "sizes") {
      const shape = pick(SHAPES),
        want = pick(["big", "small"]),
        sizes = shuffle([0.46, 0.7, 1], rng);
      q.prompt =
        want === "big" ? "Qual desenho é o maior?" : "Qual desenho é o menor?";
      q.hint = "As formas são iguais. Compare o espaço que cada uma ocupa.";
      q.scene = { type: "look" };
      q.rule = want;
      const target = want === "big" ? 1 : 0.46;
      options(
        String(target),
        sizes.map((size) => ({
          id: String(size),
          type: "sized",
          shape,
          size,
          color: "#7aaa8b",
          label:
            size === 1
              ? "Desenho grande"
              : size === 0.46
                ? "Desenho pequeno"
                : "Desenho médio",
        })),
      );
    } else if (game === "position") {
      const dirs =
        level === 0 ? ["above", "below"] : ["above", "below", "left", "right"];
      const target = pick(dirs),
        names = {
          above: "acima",
          below: "abaixo",
          left: "à esquerda",
          right: "à direita",
        };
      q.prompt = `Em qual desenho a flor está ${names[target]} da caixa?`;
      q.hint =
        target === "left"
          ? "A esquerda fica deste lado: ←."
          : target === "right"
            ? "A direita fica deste lado: →."
            : target === "above"
              ? "Procure a flor na parte de cima da caixa."
              : "Procure a flor na parte de baixo da caixa.";
      q.target = target;
      q.scene = { type: "look" };
      options(
        target,
        dirs.map((dir) => ({
          id: dir,
          type: "position",
          direction: dir,
          label: `Flor ${names[dir]} da caixa`,
        })),
      );
    } else if (game === "shadow") {
      const target = pick(SHAPES.slice(0, level ? 4 : 3));
      q.prompt = "Qual desenho combina com esta sombra?";
      q.hint = "Observe o contorno, sem se preocupar com a cor.";
      q.scene = { type: "shape", shape: target, color: "#344c47" };
      options(
        target,
        SHAPES.slice(0, level ? 4 : 3).map((s, i) => ({
          id: s,
          type: "shape",
          shape: s,
          color: COLORS[i].hex,
          label: NAMES[s],
        })),
      );
    } else if (game === "sort") {
      const groups = [
        {
          id: "animals",
          name: "um animal",
          items: ["bird", "bunny", "cat", "fish"],
        },
        {
          id: "plants",
          name: "uma planta ou parte de uma planta",
          items: ["apple", "flower", "leaf"],
        },
      ];
      const targetGroup = pick(groups),
        other = groups.find((x) => x !== targetGroup),
        target = pick(targetGroup.items);
      const labels = {
        bird: "passarinho",
        bunny: "coelho",
        cat: "gato",
        fish: "peixe",
        apple: "maçã",
        flower: "flor",
        leaf: "folha",
      };
      q.prompt = `Qual destes desenhos é ${targetGroup.name}?`;
      q.hint =
        targetGroup.id === "animals"
          ? "Procure um ser vivo que é um animal."
          : "Folhas, flores e frutos fazem parte de plantas.";
      q.targetGroup = targetGroup.id;
      q.scene = { type: "look" };
      options(
        target,
        [target, ...shuffle(other.items, rng).slice(0, 2)].map((icon) => ({
          id: icon,
          type: icon,
          label: labels[icon],
        })),
      );
    } else if (game === "letters") {
      const alphabet = "ABCDEFGHJKLMNPRSTUVZ".split(""),
        target = pick(alphabet);
      q.prompt = `Qual é a letra minúscula que combina com ${target}?`;
      q.hint = `A letra ${target} também pode ser escrita assim: ${target.toLowerCase()}.`;
      q.scene = { type: "letter", value: target };
      const rest = shuffle(
        alphabet.filter((c) => c !== target),
        rng,
      ).slice(0, 3);
      options(
        target,
        [target, ...rest].map((c) => ({
          id: c,
          type: "text",
          value: c.toLowerCase(),
          label: `Letra ${c.toLowerCase()}`,
        })),
      );
    } else if (game === "initial") {
      const target = pick(WORDS);
      q.prompt = `Qual é a primeira letra de ${target.word.toLocaleLowerCase("pt-BR")}?`;
      q.hint = `Observe a primeira letra: ${target.word[0]}.`;
      q.word = target.word;
      q.scene = { type: "word-picture", word: target.word, icon: target.icon };
      const first = target.word[0],
        rest = shuffle(
          "ABCDEFGHILMNOPRSTUV".split("").filter((c) => c !== first),
          rng,
        ).slice(0, 3);
      options(
        first,
        [first, ...rest].map((c) => ({
          id: c,
          type: "text",
          value: c,
          label: `Letra ${c}`,
        })),
      );
    } else if (game === "vowels") {
      const vowel = pick("AEIOU".split("")),
        others = shuffle("BCDFGLMNPRSTV".split(""), rng).slice(0, 3);
      q.prompt = "Qual destas letras é uma vogal?";
      q.hint = "As vogais são A, E, I, O e U.";
      q.scene = { type: "letter", value: "A E I O U" };
      options(
        vowel,
        [vowel, ...others].map((c) => ({
          id: c,
          type: "text",
          value: c,
          label: `Letra ${c}`,
        })),
      );
    } else if (game === "syllables") {
      const target = pick(WORDS);
      q.prompt = `Quantos pedacinhos tem a palavra ${target.word.toLocaleLowerCase("pt-BR")}?`;
      q.hint = `Fale devagar: ${target.syllables.join(", ")}. Conte uma batida para cada pedacinho.`;
      q.word = target.word;
      q.syllables = target.syllables;
      q.scene = { type: "word-picture", word: target.word, icon: target.icon };
      numbers(target.syllables.length, 1, 5);
    } else if (game === "word") {
      const target = pick(WORDS);
      q.prompt = "Qual palavra combina com o desenho?";
      q.hint = `O desenho é ${target.word.toLocaleLowerCase("pt-BR")}. Procure essa palavra.`;
      q.word = target.word;
      q.scene = { type: "word-picture", word: "", icon: target.icon };
      const rest = shuffle(
        WORDS.filter((x) => x !== target),
        rng,
      ).slice(0, 3);
      options(
        target.word,
        [target, ...rest].map((w) => ({
          id: w.word,
          type: "text",
          value: w.word,
          label: w.word,
        })),
      );
    } else if (game === "subtract") {
      const a = int(2, level === 2 ? 12 : 5),
        b = int(1, a);
      q.prompt = `Havia ${a} maçãs. Tiramos ${b}. Quantas ficaram?`;
      q.hint = "As maçãs riscadas saíram do grupo. Conte as que ficaram.";
      q.scene = { type: "subtract", a, b };
      numbers(a - b, 0, level === 2 ? 15 : 7);
    } else if (game === "missing") {
      const a = int(1, level === 2 ? 9 : 3),
        b = int(1, level === 2 ? 9 : 3);
      q.prompt = "Qual número está escondido na adição?";
      q.hint = `Quanto falta para ir de ${a} até ${a + b}?`;
      q.scene = { type: "equation", a, total: a + b };
      q.b = b;
      numbers(b, 0, level === 2 ? 12 : 6);
    } else if (game === "tenframe") {
      const n = int(1, 10);
      q.prompt = "Quantos lugares estão ocupados na casinha?";
      q.hint =
        "A primeira linha tem cinco lugares. Conte os círculos preenchidos.";
      q.scene = { type: "tenframe", count: n };
      numbers(n, 0, 12);
    } else if (game === "tens") {
      const tens = int(1, level === 2 ? 4 : 2),
        units = int(0, 9),
        n = tens * 10 + units;
      q.prompt = "Qual número estas dezenas e unidades formam?";
      q.hint = `Cada barra tem dez quadradinhos. Há ${tens} dezenas e ${units} unidades.`;
      q.scene = { type: "tens", tens, units };
      numbers(n, 10, 59);
    } else if (game === "clock") {
      const hour = int(1, 12);
      q.prompt = "Que horas o relógio está marcando?";
      q.hint =
        "O ponteiro grande está no doze. O ponteiro pequeno mostra a hora.";
      q.scene = { type: "clock", hour };
      numbers(hour, 1, 12);
    } else if (game === "story") {
      const story = pick(STORIES);
      q.prompt = story.question;
      q.story = story.text;
      q.hint = "Ouça a história outra vez e observe o que aconteceu.";
      q.scene = { type: "story", text: story.text, icon: story.icon };
      options(
        story.answer,
        story.choices.map((icon, i) => ({
          id: icon,
          type: icon,
          label: story.labels[i],
        })),
      );
    } else return null;
    return q;
  }
  const api = { WORDS, STORIES, EXTRA_GAMES, generateExtra };
  if (typeof module !== "undefined") module.exports = api;
  else root.ExtraCurriculum = api;
})(typeof window !== "undefined" ? window : globalThis);
