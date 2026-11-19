//! Port de curriculum.js + extra-curriculum.js + toddler-curriculum.js + variation.js.
//! A ordem das chamadas ao RNG é a mesma do JS, então a mesma semente gera a
//! mesma pergunta (verificado em `tests/parity.rs`).

use crate::rng::Rng;

pub type Hex = &'static str;

pub struct Color {
    pub id: &'static str,
    pub name: &'static str,
    pub hex: Hex,
}

pub const COLORS: [Color; 4] = [
    Color { id: "coral", name: "vermelha", hex: "#eb6d58" },
    Color { id: "blue", name: "azul", hex: "#589cd4" },
    Color { id: "yellow", name: "amarela", hex: "#efbd46" },
    Color { id: "green", name: "verde", hex: "#60a786" },
];
pub const SHAPES: [&str; 4] = ["circle", "triangle", "square", "star"];

pub fn shape_name(s: &str) -> &'static str {
    match s {
        "circle" => "círculo",
        "triangle" => "triângulo",
        "square" => "quadrado",
        _ => "estrela",
    }
}

pub struct Group {
    pub id: &'static str,
    pub label: &'static str,
    pub name: &'static str,
    pub max: i32,
    pub pairs: usize,
}

pub const GROUPS: [Group; 3] = [
    Group { id: "little", label: "2–3 anos", name: "Primeiros passos", max: 3, pairs: 2 },
    Group { id: "middle", label: "4–5 anos", name: "Mentes curiosas", max: 6, pairs: 3 },
    Group { id: "big", label: "6–8 anos", name: "Grandes ideias", max: 12, pairs: 4 },
];

pub fn group_index(id: &str) -> usize {
    GROUPS.iter().position(|g| g.id == id).unwrap_or(0)
}

pub struct Game {
    pub id: &'static str,
    pub name: &'static str,
    pub skill: &'static str,
    pub icon: &'static str,
    pub color: &'static str,
    pub min: usize,
    pub category: &'static str,
}

macro_rules! g {
    ($id:literal, $name:literal, $skill:literal, $icon:literal, $color:literal, $min:literal, $cat:literal) => {
        Game { id: $id, name: $name, skill: $skill, icon: $icon, color: $color, min: $min, category: $cat }
    };
}

/// Mesma ordem de `Curriculum.GAMES` depois dos `push` (base, extra, toddler).
pub const GAMES: [Game; 32] = [
    g!("colors", "Jardim das cores", "Reconhecer e associar", "flower", "pink", 0, "space"),
    g!("shapes", "Formas pelo caminho", "Percepção visual", "shapes", "blue", 0, "space"),
    g!("count", "Vamos contar?", "Números e quantidades", "count", "yellow", 0, "math"),
    g!("memory", "Pares da floresta", "Memória e atenção", "memory", "green", 0, "logic"),
    g!("pattern", "O que vem depois?", "Sequências e lógica", "pattern", "purple", 1, "logic"),
    g!("odd", "Um é diferente", "Observar e comparar", "odd", "peach", 0, "logic"),
    g!("sum", "Somar sorrisos", "Primeiras operações", "sum", "yellow", 1, "math"),
    g!("order", "Escada dos números", "Ordem e raciocínio", "order", "blue", 1, "math"),
    g!("compare", "Onde tem mais?", "Comparar quantidades", "count", "yellow", 0, "math"),
    g!("sizes", "Grande ou pequeno?", "Comparar tamanhos", "shapes", "blue", 0, "space"),
    g!("position", "Onde está a flor?", "Posição no espaço", "flower", "green", 0, "space"),
    g!("shadow", "De quem é a sombra?", "Contornos e associação", "shapes", "purple", 0, "space"),
    g!("sort", "Cada um no seu grupo", "Classificar e relacionar", "memory", "green", 0, "logic"),
    g!("letters", "Letras companheiras", "Maiúsculas e minúsculas", "letters", "pink", 1, "language"),
    g!("initial", "Começa com qual letra?", "Letra inicial das palavras", "letters", "peach", 1, "language"),
    g!("vowels", "Jardim das vogais", "Reconhecer as vogais", "letters", "pink", 1, "language"),
    g!("syllables", "Palavras em pedacinhos", "Contar sílabas", "letters", "purple", 2, "language"),
    g!("word", "Palavra e desenho", "Leitura de palavras", "letters", "peach", 2, "language"),
    g!("subtract", "Quantas ficaram?", "Subtração com imagens", "count", "yellow", 1, "math"),
    g!("missing", "O número escondido", "Completar uma adição", "sum", "yellow", 2, "math"),
    g!("tenframe", "Casinha dos dez", "Quantidades até dez", "count", "blue", 1, "math"),
    g!("tens", "Dez de cada vez", "Dezenas e unidades", "order", "yellow", 2, "math"),
    g!("clock", "Que horas são?", "Horas inteiras no relógio", "clock", "blue", 2, "math"),
    g!("story", "Histórias do jardim", "Escutar e compreender", "book", "green", 1, "language"),
    g!("twins", "Encontre o igual", "Associar objetos iguais", "memory", "green", 0, "logic"),
    g!("inside", "Dentro ou fora?", "Explorar posições", "container", "blue", 0, "space"),
    g!("fill", "Cheio ou vazio?", "Comparar recipientes", "cup", "yellow", 0, "space"),
    g!("tower", "Torres coloridas", "Copiar um modelo", "tower", "purple", 0, "logic"),
    g!("socks", "Meias companheiras", "Combinar cores e detalhes", "socks", "pink", 0, "logic"),
    g!("oneeach", "Um para cada amigo", "Relacionar um a um", "count", "peach", 0, "math"),
    g!("vocabulary", "Quem foi chamado?", "Ouvir e reconhecer", "book", "green", 0, "language"),
    g!("colorpath", "Caminho das cores", "Continuar uma repetição", "colorpath", "pink", 0, "logic"),
];

pub fn game(id: &str) -> &'static Game {
    GAMES.iter().find(|g| g.id == id).unwrap_or(&GAMES[0])
}

pub const CATEGORIES: [(&str, &str); 5] = [
    ("all", "Todas"),
    ("math", "Matemática"),
    ("logic", "Lógica"),
    ("space", "Formas e espaço"),
    ("language", "Letras e histórias"),
];

struct Word {
    word: &'static str,
    icon: &'static str,
    syllables: &'static [&'static str],
}
const WORDS: [Word; 10] = [
    Word { word: "SOL", icon: "sun", syllables: &["SOL"] },
    Word { word: "FLOR", icon: "flower", syllables: &["FLOR"] },
    Word { word: "MAÇÃ", icon: "apple", syllables: &["MA", "ÇÃ"] },
    Word { word: "GATO", icon: "cat", syllables: &["GA", "TO"] },
    Word { word: "BOLA", icon: "ball", syllables: &["BO", "LA"] },
    Word { word: "LUA", icon: "moon", syllables: &["LU", "A"] },
    Word { word: "CASA", icon: "house", syllables: &["CA", "SA"] },
    Word { word: "PEIXE", icon: "fish", syllables: &["PEI", "XE"] },
    Word { word: "COELHO", icon: "bunny", syllables: &["CO", "E", "LHO"] },
    Word { word: "PÁSSARO", icon: "bird", syllables: &["PÁS", "SA", "RO"] },
];

struct Story {
    text: &'static str,
    question: &'static str,
    icon: &'static str,
    answer: &'static str,
    choices: [&'static str; 3],
    labels: [&'static str; 3],
}
const STORIES: [Story; 6] = [
    Story { text: "Nina, a coelha, encontrou uma maçã no jardim. Ela dividiu a maçã com seu amigo passarinho.", question: "Qual fruta Nina dividiu?", icon: "bunny", answer: "apple", choices: ["apple", "flower", "ball"], labels: ["maçã", "flor", "bola"] },
    Story { text: "O passarinho Tito recolheu uma flor amarela. Depois, levou a flor para perto de sua casa.", question: "O que Tito recolheu?", icon: "bird", answer: "flower", choices: ["flower", "apple", "fish"], labels: ["flor", "maçã", "peixe"] },
    Story { text: "O gato Bento brincava com uma bola. Quando o sol apareceu, Bento levou a bola para o jardim.", question: "Com o que Bento brincava?", icon: "cat", answer: "ball", choices: ["ball", "apple", "flower"], labels: ["bola", "maçã", "flor"] },
    Story { text: "A lua apareceu no céu. Nina, a coelha, guardou seus brinquedos e foi descansar.", question: "O que apareceu no céu?", icon: "bunny", answer: "moon", choices: ["moon", "sun", "flower"], labels: ["lua", "sol", "flor"] },
    Story { text: "Tito viu um peixe nadando no lago. O peixe passou por uma pedra e mergulhou devagar.", question: "Qual animal estava no lago?", icon: "bird", answer: "fish", choices: ["fish", "cat", "bunny"], labels: ["peixe", "gato", "coelho"] },
    Story { text: "Bento encontrou Nina em frente à casa. O gato e a coelha entraram juntos para brincar.", question: "Onde os amigos entraram?", icon: "cat", answer: "house", choices: ["house", "flower", "apple"], labels: ["casa", "flor", "maçã"] },
];

const OBJECTS: [(&str, &str); 10] = [
    ("apple", "a maçã"),
    ("flower", "a flor"),
    ("bird", "o passarinho"),
    ("bunny", "o coelho"),
    ("cat", "o gato"),
    ("fish", "o peixe"),
    ("ball", "a bola"),
    ("moon", "a lua"),
    ("house", "a casa"),
    ("sun", "o sol"),
];

// ---------------------------------------------------------------- modelo

#[derive(Clone, Debug, PartialEq)]
pub enum Visual {
    /// Desenho do `Art.icon(kind, color)`.
    Icon { kind: &'static str, color: Option<String> },
    Number(i32),
    Text(String),
    Swatch(Hex),
    Quantity(u32),
    Sized { shape: &'static str, size: f64, color: String },
    Position(&'static str),
    Sock { color: Hex, pattern: &'static str },
    Cup(f64),
    Container(&'static str),
    Tower(Vec<Hex>),
}

#[derive(Clone, Debug)]
pub struct Opt {
    pub id: String,
    pub label: String,
    pub visual: Visual,
}

#[derive(Clone, Debug)]
pub enum Scene {
    Color(String),
    Shape { shape: &'static str, color: String },
    Items(u32),
    Sum(u32, u32),
    Look,
    Sequence(Vec<&'static str>),
    Numbers(Vec<Option<i32>>),
    ObjectModel(&'static str),
    Sock { color: Hex, pattern: &'static str },
    Tower(Vec<Hex>),
    Friends(u32),
    ColorSequence(Vec<Hex>),
    Letter(String),
    WordPicture { word: &'static str, icon: &'static str },
    Subtract { a: u32, b: u32 },
    Equation { a: i32, total: i32 },
    TenFrame(u32),
    Tens { tens: u32, units: u32 },
    Clock(u32),
    Story { text: &'static str, icon: &'static str },
}

#[derive(Clone, Debug)]
pub struct Card {
    pub kind: &'static str,
    pub pair: usize,
}

#[derive(Clone, Debug)]
pub struct Presentation {
    pub surface: Hex,
    pub radius: u32,
    pub item_color: Hex,
    pub item_gap: u32,
    pub rotation: [i32; 24],
}

#[derive(Clone, Debug)]
pub struct Question {
    pub game: &'static str,
    pub prompt: String,
    pub hint: String,
    pub story: Option<&'static str>,
    pub memory: bool,
    pub options: Vec<Opt>,
    pub answer: usize,
    pub cards: Vec<Card>,
    pub scene: Option<Scene>,
    pub presentation: Presentation,
}

impl Question {
    fn new(game: &'static str) -> Self {
        Question {
            game,
            prompt: String::new(),
            hint: String::new(),
            story: None,
            memory: false,
            options: Vec::new(),
            answer: 0,
            cards: Vec::new(),
            scene: None,
            presentation: Presentation {
                surface: "#fffaf0",
                radius: 20,
                item_color: "#e87961",
                item_gap: 4,
                rotation: [0; 24],
            },
        }
    }
    /// `options(correct, items)` do JS: embaralha e localiza a resposta.
    fn set_options(&mut self, rng: &mut Rng, correct: &str, items: Vec<Opt>) {
        self.options = rng.shuffle(&items);
        self.answer = self.options.iter().position(|o| o.id == correct).unwrap_or(0);
    }
}

fn opt(id: impl Into<String>, label: impl Into<String>, visual: Visual) -> Opt {
    Opt { id: id.into(), label: label.into(), visual }
}
fn icon(kind: &'static str, color: Option<&str>) -> Visual {
    Visual::Icon { kind, color: color.map(str::to_owned) }
}
fn num_opt(v: i32) -> Opt {
    opt(v.to_string(), v.to_string(), Visual::Number(v))
}
fn lower_pt(s: &str) -> String {
    s.to_lowercase()
}

// ---------------------------------------------------------------- geração

fn generate_base(game: &'static str, group: &str, seed: u32) -> Question {
    let mut rng = Rng::new(seed);
    let level = group_index(group);
    let g = &GROUPS[level];
    let mut q = Question::new(game);
    let r = &mut rng;
    match game {
        "colors" => {
            let t = &COLORS[r.below(4)];
            q.prompt = format!("Encontre a flor de cor {}.", t.name);
            q.hint = "Observe a cor das pétalas.".into();
            q.scene = Some(Scene::Color(t.hex.into()));
            let others: Vec<usize> = (0..4).filter(|&i| COLORS[i].id != t.id).collect();
            let rest: Vec<usize> = r.shuffle(&others).into_iter().take(if level > 0 { 3 } else { 2 }).collect();
            let items = std::iter::once(COLORS.iter().position(|c| c.id == t.id).unwrap())
                .chain(rest)
                .map(|i| opt(COLORS[i].id, COLORS[i].name, icon("flower", Some(COLORS[i].hex))))
                .collect();
            q.set_options(r, t.id, items);
        }
        "shapes" => {
            let pool = &SHAPES[..if level > 0 { 4 } else { 3 }];
            let t = r.pick(pool);
            q.prompt = "Qual forma é igual à do desenho?".into();
            q.hint = format!("Procure o {}.", shape_name(t));
            q.scene = Some(Scene::Shape { shape: t, color: "#559d8b".into() });
            let others: Vec<&str> = SHAPES.iter().copied().filter(|&s| s != t).collect();
            let mut list: Vec<&'static str> = r.shuffle(&others).into_iter().take(if level > 0 { 3 } else { 2 }).collect();
            list.push(t);
            let items = list.into_iter().map(|s| opt(s, shape_name(s), icon(s, Some("#e9a94b")))).collect();
            q.set_options(r, t, items);
        }
        "count" => {
            let n = r.int(1, g.max);
            q.prompt = "Quantas maçãs você consegue contar?".into();
            q.hint = "Toque em cada maçã e conte devagar.".into();
            q.scene = Some(Scene::Items(n as u32));
            let pool: Vec<i32> = (1..=g.max + 2).filter(|&i| i != n).collect();
            let rest = r.shuffle(&pool).into_iter().take(if level > 0 { 3 } else { 2 });
            let items = std::iter::once(n).chain(rest).map(num_opt).collect();
            q.set_options(r, &n.to_string(), items);
        }
        "memory" => {
            q.memory = true;
            q.prompt = "Encontre os pares que são iguais.".into();
            q.hint = "Vire duas cartas. Lembre onde cada desenho está.".into();
            let kinds: Vec<&'static str> = r.shuffle(&["apple", "bird", "flower", "sun"]).into_iter().take(g.pairs).collect();
            let cards: Vec<Card> = kinds
                .iter()
                .enumerate()
                .flat_map(|(pair, &kind)| [Card { kind, pair }, Card { kind, pair }])
                .collect();
            q.cards = r.shuffle(&cards);
        }
        "pattern" => {
            let a = r.pick(&SHAPES);
            let others: Vec<&str> = SHAPES.iter().copied().filter(|&s| s != a).collect();
            let b = r.pick(&others);
            let seq: Vec<&'static str> = if level == 2 {
                let variants = [vec![a, a, b, a, a, b], vec![a, b, b, a, b, b], vec![a, b, a, b, a, b]];
                r.pick(&variants)
            } else {
                vec![a, b, a, b]
            };
            let target = *seq.last().unwrap();
            q.prompt = "Qual desenho completa a sequência?".into();
            q.hint = "Diga os nomes das formas e procure a repetição.".into();
            q.scene = Some(Scene::Sequence(seq[..seq.len() - 1].to_vec()));
            let items = SHAPES.iter().map(|&s| opt(s, shape_name(s), icon(s, Some("#8b7bbd")))).collect();
            q.set_options(r, target, items);
        }
        "odd" => {
            let normal = r.pick(&SHAPES);
            let others: Vec<&str> = SHAPES.iter().copied().filter(|&s| s != normal).collect();
            let other = r.pick(&others);
            let size = if level > 0 { 4 } else { 3 };
            let index = r.int(0, size - 1) as usize;
            q.prompt = "Qual desenho tem uma forma diferente?".into();
            q.hint = "A cor é igual. Compare o contorno dos desenhos.".into();
            q.scene = Some(Scene::Look);
            q.options = (0..size as usize)
                .map(|i| {
                    let s = if i == index { other } else { normal };
                    opt(i.to_string(), format!("Desenho {}: {}", i + 1, shape_name(s)), icon(s, Some("#599b84")))
                })
                .collect();
            q.answer = index;
        }
        "sum" => {
            let hi = if level == 2 { 9 } else { 3 };
            let a = r.int(1, hi);
            let b = r.int(1, hi);
            let n = a + b;
            q.prompt = "Juntando os dois grupos, quantas maçãs temos?".into();
            q.hint = "Conte as maçãs de um grupo e continue no outro.".into();
            q.scene = Some(Scene::Sum(a as u32, b as u32));
            let pool: Vec<i32> = (1..=(n + 3).max(10)).filter(|&i| i != n).collect();
            let rest = r.shuffle(&pool).into_iter().take(3);
            let items = std::iter::once(n).chain(rest).map(num_opt).collect();
            q.set_options(r, &n.to_string(), items);
        }
        "order" => {
            let step = if level == 2 { r.pick(&[1, 2, 5]) } else { 1 };
            let start = r.int(1, if level == 2 { 10 } else { 4 });
            let index = r.int(1, 3) as usize;
            let seq: Vec<i32> = (0..4).map(|i| start + i * step).collect();
            let n = seq[index];
            q.prompt = "Qual número falta na escada?".into();
            q.hint = if step == 1 { "Conte os números em ordem.".into() } else { format!("Conte de {step} em {step}.") };
            q.scene = Some(Scene::Numbers(seq.iter().enumerate().map(|(i, &v)| (i != index).then_some(v)).collect()));
            let pool: Vec<i32> = (1..=seq[3] + 3).filter(|&i| i != n).collect();
            let rest = r.shuffle(&pool).into_iter().take(3);
            let items = std::iter::once(n).chain(rest).map(num_opt).collect();
            q.set_options(r, &n.to_string(), items);
        }
        _ => {
            if !generate_toddler(&mut q, game, level, seed) && !generate_extra(&mut q, game, level, seed) {
                q.prompt = "Atividade desconhecida".into();
            }
        }
    }
    q
}

fn generate_extra(q: &mut Question, game: &str, level: usize, seed: u32) -> bool {
    let mut rng = Rng::new(seed);
    let r = &mut rng;
    let take = if level > 0 { 3 } else { 2 };
    // `numbers(answer, lo, hi)` do JS.
    let numbers = |q: &mut Question, r: &mut Rng, answer: i32, lo: i32, hi: i32| {
        let pool: Vec<i32> = (lo..=hi).filter(|&n| n != answer).collect();
        let rest = r.shuffle(&pool).into_iter().take(take);
        let items = std::iter::once(answer).chain(rest).map(num_opt).collect();
        q.set_options(r, &answer.to_string(), items);
    };
    match game {
        "compare" => {
            let len = [3, 6, 10][level];
            let pool: Vec<u32> = (1..=len).collect();
            let counts: Vec<u32> = r.shuffle(&pool).into_iter().take(3).collect();
            let want = r.pick(&["more", "less"]);
            let target = if want == "more" { *counts.iter().max().unwrap() } else { *counts.iter().min().unwrap() };
            q.prompt = if want == "more" { "Qual grupo tem mais maçãs?" } else { "Qual grupo tem menos maçãs?" }.into();
            q.hint = "Conte as maçãs de cada grupo e compare.".into();
            q.scene = Some(Scene::Look);
            let items = counts.iter().map(|&n| opt(n.to_string(), format!("Grupo com {n} maçãs"), Visual::Quantity(n))).collect();
            q.set_options(r, &target.to_string(), items);
        }
        "sizes" => {
            let shape = r.pick(&SHAPES);
            let want = r.pick(&["big", "small"]);
            let sizes = r.shuffle(&[0.46, 0.7, 1.0]);
            q.prompt = if want == "big" { "Qual desenho é o maior?" } else { "Qual desenho é o menor?" }.into();
            q.hint = "As formas são iguais. Compare o espaço que cada uma ocupa.".into();
            q.scene = Some(Scene::Look);
            let target = if want == "big" { "1" } else { "0.46" };
            let items = sizes
                .iter()
                .map(|&size| {
                    let id = if size == 1.0 { "1".to_string() } else { size.to_string() };
                    let label = if size == 1.0 { "Desenho grande" } else if size == 0.46 { "Desenho pequeno" } else { "Desenho médio" };
                    opt(id, label, Visual::Sized { shape, size, color: "#7aaa8b".into() })
                })
                .collect();
            q.set_options(r, target, items);
        }
        "position" => {
            let dirs: &[&'static str] = if level == 0 { &["above", "below"] } else { &["above", "below", "left", "right"] };
            let target = r.pick(dirs);
            let name = |d: &str| match d {
                "above" => "acima",
                "below" => "abaixo",
                "left" => "à esquerda",
                _ => "à direita",
            };
            q.prompt = format!("Em qual desenho a flor está {} da caixa?", name(target));
            q.hint = match target {
                "left" => "A esquerda fica deste lado: ←.",
                "right" => "A direita fica deste lado: →.",
                "above" => "Procure a flor na parte de cima da caixa.",
                _ => "Procure a flor na parte de baixo da caixa.",
            }
            .into();
            q.scene = Some(Scene::Look);
            let items = dirs.iter().map(|&d| opt(d, format!("Flor {} da caixa", name(d)), Visual::Position(d))).collect();
            q.set_options(r, target, items);
        }
        "shadow" => {
            let pool = &SHAPES[..if level > 0 { 4 } else { 3 }];
            let target = r.pick(pool);
            q.prompt = "Qual desenho combina com esta sombra?".into();
            q.hint = "Observe o contorno, sem se preocupar com a cor.".into();
            q.scene = Some(Scene::Shape { shape: target, color: "#344c47".into() });
            let items = pool.iter().enumerate().map(|(i, &s)| opt(s, shape_name(s), icon(s, Some(COLORS[i].hex)))).collect();
            q.set_options(r, target, items);
        }
        "sort" => {
            let animals: &[&'static str] = &["bird", "bunny", "cat", "fish"];
            let plants: &[&'static str] = &["apple", "flower", "leaf"];
            let is_animals = r.below(2) == 0;
            let (target_items, other_items) = if is_animals { (animals, plants) } else { (plants, animals) };
            let target = r.pick(target_items);
            let label = |k: &str| match k {
                "bird" => "passarinho",
                "bunny" => "coelho",
                "cat" => "gato",
                "fish" => "peixe",
                "apple" => "maçã",
                "flower" => "flor",
                _ => "folha",
            };
            q.prompt = format!(
                "Qual destes desenhos é {}?",
                if is_animals { "um animal" } else { "uma planta ou parte de uma planta" }
            );
            q.hint = if is_animals { "Procure um ser vivo que é um animal." } else { "Folhas, flores e frutos fazem parte de plantas." }.into();
            q.scene = Some(Scene::Look);
            let items = std::iter::once(target)
                .chain(r.shuffle(other_items).into_iter().take(2))
                .map(|k| opt(k, label(k), icon(k, None)))
                .collect();
            q.set_options(r, target, items);
        }
        "letters" => {
            let alphabet: Vec<char> = "ABCDEFGHJKLMNPRSTUVZ".chars().collect();
            let t = r.pick(&alphabet);
            let lo = t.to_lowercase().to_string();
            q.prompt = format!("Qual é a letra minúscula que combina com {t}?");
            q.hint = format!("A letra {t} também pode ser escrita assim: {lo}.");
            q.scene = Some(Scene::Letter(t.to_string()));
            let others: Vec<char> = alphabet.iter().copied().filter(|&c| c != t).collect();
            let items = std::iter::once(t)
                .chain(r.shuffle(&others).into_iter().take(3))
                .map(|c| {
                    let l = c.to_lowercase().to_string();
                    opt(c.to_string(), format!("Letra {l}"), Visual::Text(l))
                })
                .collect();
            q.set_options(r, &t.to_string(), items);
        }
        "initial" => {
            let w = &WORDS[r.below(WORDS.len())];
            let first = w.word.chars().next().unwrap();
            q.prompt = format!("Qual é a primeira letra de {}?", lower_pt(w.word));
            q.hint = format!("Observe a primeira letra: {first}.");
            q.scene = Some(Scene::WordPicture { word: w.word, icon: w.icon });
            let others: Vec<char> = "ABCDEFGHILMNOPRSTUV".chars().filter(|&c| c != first).collect();
            let items = std::iter::once(first)
                .chain(r.shuffle(&others).into_iter().take(3))
                .map(|c| opt(c.to_string(), format!("Letra {c}"), Visual::Text(c.to_string())))
                .collect();
            q.set_options(r, &first.to_string(), items);
        }
        "vowels" => {
            let vowels: Vec<char> = "AEIOU".chars().collect();
            let v = r.pick(&vowels);
            let cons: Vec<char> = "BCDFGLMNPRSTV".chars().collect();
            let others: Vec<char> = r.shuffle(&cons).into_iter().take(3).collect();
            q.prompt = "Qual destas letras é uma vogal?".into();
            q.hint = "As vogais são A, E, I, O e U.".into();
            q.scene = Some(Scene::Letter("A E I O U".into()));
            let items = std::iter::once(v)
                .chain(others)
                .map(|c| opt(c.to_string(), format!("Letra {c}"), Visual::Text(c.to_string())))
                .collect();
            q.set_options(r, &v.to_string(), items);
        }
        "syllables" => {
            let w = &WORDS[r.below(WORDS.len())];
            q.prompt = format!("Quantos pedacinhos tem a palavra {}?", lower_pt(w.word));
            q.hint = format!("Fale devagar: {}. Conte uma batida para cada pedacinho.", w.syllables.join(", "));
            q.scene = Some(Scene::WordPicture { word: w.word, icon: w.icon });
            numbers(q, r, w.syllables.len() as i32, 1, 5);
        }
        "word" => {
            let ti = r.below(WORDS.len());
            let w = &WORDS[ti];
            q.prompt = "Qual palavra combina com o desenho?".into();
            q.hint = format!("O desenho é {}. Procure essa palavra.", lower_pt(w.word));
            q.scene = Some(Scene::WordPicture { word: "", icon: w.icon });
            let others: Vec<usize> = (0..WORDS.len()).filter(|&i| i != ti).collect();
            let items = std::iter::once(ti)
                .chain(r.shuffle(&others).into_iter().take(3))
                .map(|i| opt(WORDS[i].word, WORDS[i].word, Visual::Text(WORDS[i].word.into())))
                .collect();
            q.set_options(r, w.word, items);
        }
        "subtract" => {
            let a = r.int(2, if level == 2 { 12 } else { 5 });
            let b = r.int(1, a);
            q.prompt = format!("Havia {a} maçãs. Tiramos {b}. Quantas ficaram?");
            q.hint = "As maçãs riscadas saíram do grupo. Conte as que ficaram.".into();
            q.scene = Some(Scene::Subtract { a: a as u32, b: b as u32 });
            numbers(q, r, a - b, 0, if level == 2 { 15 } else { 7 });
        }
        "missing" => {
            let hi = if level == 2 { 9 } else { 3 };
            let a = r.int(1, hi);
            let b = r.int(1, hi);
            q.prompt = "Qual número está escondido na adição?".into();
            q.hint = format!("Quanto falta para ir de {a} até {}?", a + b);
            q.scene = Some(Scene::Equation { a, total: a + b });
            numbers(q, r, b, 0, if level == 2 { 12 } else { 6 });
        }
        "tenframe" => {
            let n = r.int(1, 10);
            q.prompt = "Quantos lugares estão ocupados na casinha?".into();
            q.hint = "A primeira linha tem cinco lugares. Conte os círculos preenchidos.".into();
            q.scene = Some(Scene::TenFrame(n as u32));
            numbers(q, r, n, 0, 12);
        }
        "tens" => {
            let tens = r.int(1, if level == 2 { 4 } else { 2 });
            let units = r.int(0, 9);
            q.prompt = "Qual número estas dezenas e unidades formam?".into();
            q.hint = format!("Cada barra tem dez quadradinhos. Há {tens} dezenas e {units} unidades.");
            q.scene = Some(Scene::Tens { tens: tens as u32, units: units as u32 });
            numbers(q, r, tens * 10 + units, 10, 59);
        }
        "clock" => {
            let hour = r.int(1, 12);
            q.prompt = "Que horas o relógio está marcando?".into();
            q.hint = "O ponteiro grande está no doze. O ponteiro pequeno mostra a hora.".into();
            q.scene = Some(Scene::Clock(hour as u32));
            numbers(q, r, hour, 1, 12);
        }
        "story" => {
            let s = &STORIES[r.below(STORIES.len())];
            q.prompt = s.question.into();
            q.story = Some(s.text);
            q.hint = "Ouça a história outra vez e observe o que aconteceu.".into();
            q.scene = Some(Scene::Story { text: s.text, icon: s.icon });
            let items = (0..3).map(|i| opt(s.choices[i], s.labels[i], icon(s.choices[i], None))).collect();
            q.set_options(r, s.answer, items);
        }
        _ => return false,
    }
    true
}

fn generate_toddler(q: &mut Question, game: &str, level: usize, seed: u32) -> bool {
    let mut rng = Rng::new(seed);
    let r = &mut rng;
    let choices = if level == 0 { 2 } else { 3 };
    match game {
        "twins" | "vocabulary" => {
            let ti = r.below(OBJECTS.len());
            let (tid, tname) = OBJECTS[ti];
            let twins = game == "twins";
            q.prompt = if twins { "Qual desenho é igual ao modelo?".into() } else { format!("Encontre {tname}.") };
            q.hint = if twins { "Olhe o desenho grande e procure outro igual.".into() } else { format!("Vamos procurar {tname} juntos.") };
            q.scene = Some(if twins { Scene::ObjectModel(tid) } else { Scene::Look });
            let others: Vec<usize> = (0..OBJECTS.len()).filter(|&i| i != ti).collect();
            let items = std::iter::once(ti)
                .chain(r.shuffle(&others).into_iter().take(choices - 1))
                .map(|i| opt(OBJECTS[i].0, OBJECTS[i].1, icon(OBJECTS[i].0, None)))
                .collect();
            q.set_options(r, tid, items);
        }
        "inside" => {
            let target = r.pick(&["inside", "outside"]);
            q.prompt = if target == "inside" { "Em qual desenho a flor está dentro da caixa?" } else { "Em qual desenho a flor está fora da caixa?" }.into();
            q.hint = if target == "inside" { "Procure a flor no espaço de dentro da caixa." } else { "Procure a flor ao lado de fora da caixa." }.into();
            q.scene = Some(Scene::Look);
            let items = ["inside", "outside"]
                .iter()
                .map(|&p| opt(p, if p == "inside" { "Flor dentro da caixa" } else { "Flor fora da caixa" }, Visual::Container(p)))
                .collect();
            q.set_options(r, target, items);
        }
        "fill" => {
            let target = r.pick(&[0, 1]);
            q.prompt = if target == 1 { "Qual copo está cheio?" } else { "Qual copo está vazio?" }.into();
            q.hint = if target == 1 { "Procure o copo com água até perto da borda." } else { "Procure o copo sem água." }.into();
            q.scene = Some(Scene::Look);
            let amounts: &[f64] = if level == 0 { &[0.0, 1.0] } else { &[0.0, 0.5, 1.0] };
            let items = amounts
                .iter()
                .map(|&a| {
                    let id = if a == 0.5 { "0.5".to_string() } else { (a as i32).to_string() };
                    let label = if a == 1.0 { "Copo cheio" } else if a == 0.0 { "Copo vazio" } else { "Copo pela metade" };
                    opt(id, label, Visual::Cup(a))
                })
                .collect();
            q.set_options(r, &target.to_string(), items);
        }
        "tower" => {
            let idx: Vec<usize> = (0..4).collect();
            let palette: Vec<Hex> = r.shuffle(&idx).into_iter().take(if level == 0 { 2 } else { 3 }).map(|i| COLORS[i].hex).collect();
            q.prompt = "Qual torre é igual à torre do modelo?".into();
            q.hint = "Olhe a cor dos blocos de baixo para cima.".into();
            q.scene = Some(Scene::Tower(palette.clone()));
            let mut variants = vec![palette.clone(), palette.iter().rev().copied().collect::<Vec<_>>()];
            if choices == 3 {
                variants.push(vec![palette[1], palette[0], palette[2]]);
            }
            let items = variants.into_iter().enumerate().map(|(i, c)| opt(c.join(":"), format!("Torre {}", i + 1), Visual::Tower(c))).collect();
            q.set_options(r, &palette.join(":"), items);
        }
        "socks" => {
            let color = COLORS[r.below(4)].hex;
            let others: Vec<Hex> = COLORS.iter().map(|c| c.hex).filter(|&h| h != color).collect();
            let other = r.pick(&others);
            let pattern = r.pick(&["dots", "stripes"]);
            q.prompt = "Qual meia faz par com esta?".into();
            q.hint = "Procure a mesma cor e o mesmo desenho na meia.".into();
            q.scene = Some(Scene::Sock { color, pattern });
            let mut variants = vec![(color, pattern), (other, pattern)];
            if choices == 3 {
                variants.push((color, if pattern == "dots" { "stripes" } else { "dots" }));
            }
            let items = variants
                .into_iter()
                .enumerate()
                .map(|(i, (c, p))| opt(format!("{c}{p}"), format!("Meia {}", i + 1), Visual::Sock { color: c, pattern: p }))
                .collect();
            q.set_options(r, &format!("{color}{pattern}"), items);
        }
        "oneeach" => {
            let n = 1 + r.below(if level == 0 { 3 } else { 5 }) as u32;
            q.prompt = "Qual grupo tem uma maçã para cada coelho?".into();
            q.hint = "Dê uma maçã para um coelho. Depois, uma para o próximo.".into();
            q.scene = Some(Scene::Friends(n));
            let pool: Vec<u32> = (1..=if level == 0 { 3 } else { 6 }).filter(|&x| x != n).collect();
            let items = std::iter::once(n)
                .chain(r.shuffle(&pool).into_iter().take(choices - 1))
                .map(|c| opt(c.to_string(), format!("Grupo com {c} maçãs"), Visual::Quantity(c)))
                .collect();
            q.set_options(r, &n.to_string(), items);
        }
        "colorpath" => {
            let idx: Vec<usize> = (0..4).collect();
            let ab: Vec<usize> = r.shuffle(&idx).into_iter().take(2).collect();
            let (a, b) = (&COLORS[ab[0]], &COLORS[ab[1]]);
            q.prompt = "Qual cor continua o caminho?".into();
            q.hint = "As cores aparecem uma de cada vez e depois se repetem.".into();
            q.scene = Some(Scene::ColorSequence(vec![a.hex, b.hex, a.hex]));
            let items = [a, b].iter().map(|c| opt(c.id, format!("Cor {}", c.name), Visual::Swatch(c.hex))).collect();
            q.set_options(r, b.id, items);
        }
        _ => return false,
    }
    true
}

/// Port de `Variation.varyQuestion`.
fn vary(mut q: Question, seed: u32) -> Question {
    let mut r = Rng::new(seed ^ 0xa4b1_c987);
    const PALETTE: [Hex; 5] = ["#e97e68", "#5799be", "#e0b849", "#68a585", "#9b86bd"];
    let p = &mut q.presentation;
    p.surface = r.pick(&["#fffaf0", "#f2f7ed", "#eff6f7", "#faf1ed", "#f5f0f8"]);
    p.radius = 20 + r.below(10) as u32;
    p.item_color = r.pick(&["#e87961", "#83ad68", "#e6b85c"]);
    p.item_gap = 4 + r.below(5) as u32;
    for x in p.rotation.iter_mut() {
        *x = r.below(17) as i32 - 8;
    }
    if q.game == "colors" {
        for o in q.options.iter_mut() {
            let shades: [Hex; 3] = match o.id.as_str() {
                "coral" => ["#e46b5b", "#df6357", "#e67a69"],
                "blue" => ["#548fc5", "#5b9cce", "#478bbb"],
                "yellow" => ["#edc14c", "#e6b73e", "#efc656"],
                _ => ["#60a277", "#6caa80", "#56976c"],
            };
            let c = r.pick(&shades);
            if let Visual::Icon { color, .. } = &mut o.visual {
                *color = Some(c.into());
            }
        }
        if let Some(Visual::Icon { color: Some(c), .. }) = q.options.get(q.answer).map(|o| &o.visual) {
            q.scene = Some(Scene::Color(c.clone()));
        }
    }
    let shared = r.pick(&PALETTE);
    let odd = q.game == "odd";
    for o in q.options.iter_mut() {
        match &mut o.visual {
            Visual::Icon { kind, color } if SHAPES.contains(kind) => {
                *color = Some(if odd { shared } else { r.pick(&PALETTE) }.into());
            }
            Visual::Sized { size, color, .. } => {
                let base = *size;
                *size = if base < 0.5 { 0.4 + r.next() * 0.08 } else if base < 0.8 { 0.63 + r.next() * 0.08 } else { 0.91 + r.next() * 0.08 };
                *color = shared.into();
            }
            _ => {}
        }
    }
    if q.game != "shadow" {
        if let Some(Scene::Shape { color, .. }) = &mut q.scene {
            *color = r.pick(&PALETTE).into();
        }
    }
    q
}

pub fn generate(game: &'static str, group: &str, seed: u32) -> Question {
    vary(generate_base(game, group, seed), seed)
}

/// Catálogo da tela inicial: jogos liberados para a faixa, embaralhados como no JS.
pub fn catalog(level: usize, seed: u32) -> Vec<&'static Game> {
    let avail: Vec<&'static Game> = GAMES.iter().filter(|g| g.min <= level).collect();
    Rng::new(seed).shuffle(&avail)
}
