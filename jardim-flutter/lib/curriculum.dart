// Port de curriculum.js + extra + toddler + variation.js para Dart.
// A ordem das chamadas ao RNG é a mesma do JS: a mesma semente gera a mesma
// pergunta (test/parity_test.dart confere 96/96 contra o JS original).
import 'dart:math' as math;

const _m32 = 0xFFFFFFFF;

/// mulberry32, igual a `Curriculum.random(seed)`. Inteiros do Dart têm 64
/// bits: toda operação é mascarada para 32 bits sem sinal, como no JS.
class Rng {
  int _s;
  Rng(int seed) : _s = seed & _m32;

  double next() {
    _s = (_s + 0x6d2b79f5) & _m32;
    final a = _s;
    var t = ((a ^ (a >> 15)) * (1 | a)) & _m32;
    t = ((t + (((t ^ (t >> 7)) * (61 | t)) & _m32)) & _m32) ^ t;
    return ((t ^ (t >> 14)) & _m32) / 4294967296.0;
  }

  int below(int n) => (next() * n).floor();
  int range(int lo, int hi) => lo + (next() * (hi - lo + 1)).floor();
  T pick<T>(List<T> xs) => xs[below(xs.length)];
  List<T> shuffle<T>(List<T> xs) {
    final a = List<T>.of(xs);
    for (var i = a.length - 1; i > 0; i--) {
      final j = (next() * (i + 1)).floor();
      final tmp = a[i];
      a[i] = a[j];
      a[j] = tmp;
    }
    return a;
  }
}

/// FNV-1a sobre a 1ª unidade UTF-16 de cada code point (como o JS).
int hash(String text) {
  var v = 2166136261;
  for (final r in text.runes) {
    final unit = r > 0xFFFF ? 0xD800 + ((r - 0x10000) >> 10) : r;
    v ^= unit;
    v = (v * 16777619) & _m32;
  }
  return v;
}

int seedFor(String day, String group, String game, int round, int entry) =>
    hash('$day|$group|$game|$round|$entry');

int entrySeed() => math.Random.secure().nextInt(1 << 32);

class Group {
  final String id, label, name;
  final int max, pairs;
  const Group(this.id, this.label, this.name, this.max, this.pairs);
}

class Game {
  final String id, name, skill, icon, color, category;
  final int min;
  const Game(this.id, this.name, this.skill, this.icon, this.color, this.min, this.category);
}

const groups = [
  Group('little', '2–3 anos', 'Primeiros passos', 3, 2),
  Group('middle', '4–5 anos', 'Mentes curiosas', 6, 3),
  Group('big', '6–8 anos', 'Grandes ideias', 12, 4),
];

int groupIndex(String id) => math.max(0, groups.indexWhere((g) => g.id == id));

const games = [
  Game('colors', 'Jardim das cores', 'Reconhecer e associar', 'flower', 'pink', 0, 'space'),
  Game('shapes', 'Formas pelo caminho', 'Percepção visual', 'shapes', 'blue', 0, 'space'),
  Game('count', 'Vamos contar?', 'Números e quantidades', 'count', 'yellow', 0, 'math'),
  Game('memory', 'Pares da floresta', 'Memória e atenção', 'memory', 'green', 0, 'logic'),
  Game('pattern', 'O que vem depois?', 'Sequências e lógica', 'pattern', 'purple', 1, 'logic'),
  Game('odd', 'Um é diferente', 'Observar e comparar', 'odd', 'peach', 0, 'logic'),
  Game('sum', 'Somar sorrisos', 'Primeiras operações', 'sum', 'yellow', 1, 'math'),
  Game('order', 'Escada dos números', 'Ordem e raciocínio', 'order', 'blue', 1, 'math'),
  Game('compare', 'Onde tem mais?', 'Comparar quantidades', 'count', 'yellow', 0, 'math'),
  Game('sizes', 'Grande ou pequeno?', 'Comparar tamanhos', 'shapes', 'blue', 0, 'space'),
  Game('position', 'Onde está a flor?', 'Posição no espaço', 'flower', 'green', 0, 'space'),
  Game('shadow', 'De quem é a sombra?', 'Contornos e associação', 'shapes', 'purple', 0, 'space'),
  Game('sort', 'Cada um no seu grupo', 'Classificar e relacionar', 'memory', 'green', 0, 'logic'),
  Game('letters', 'Letras companheiras', 'Maiúsculas e minúsculas', 'letters', 'pink', 1, 'language'),
  Game('initial', 'Começa com qual letra?', 'Letra inicial das palavras', 'letters', 'peach', 1, 'language'),
  Game('vowels', 'Jardim das vogais', 'Reconhecer as vogais', 'letters', 'pink', 1, 'language'),
  Game('syllables', 'Palavras em pedacinhos', 'Contar sílabas', 'letters', 'purple', 2, 'language'),
  Game('word', 'Palavra e desenho', 'Leitura de palavras', 'letters', 'peach', 2, 'language'),
  Game('subtract', 'Quantas ficaram?', 'Subtração com imagens', 'count', 'yellow', 1, 'math'),
  Game('missing', 'O número escondido', 'Completar uma adição', 'sum', 'yellow', 2, 'math'),
  Game('tenframe', 'Casinha dos dez', 'Quantidades até dez', 'count', 'blue', 1, 'math'),
  Game('tens', 'Dez de cada vez', 'Dezenas e unidades', 'order', 'yellow', 2, 'math'),
  Game('clock', 'Que horas são?', 'Horas inteiras no relógio', 'clock', 'blue', 2, 'math'),
  Game('story', 'Histórias do jardim', 'Escutar e compreender', 'book', 'green', 1, 'language'),
  Game('twins', 'Encontre o igual', 'Associar objetos iguais', 'memory', 'green', 0, 'logic'),
  Game('inside', 'Dentro ou fora?', 'Explorar posições', 'container', 'blue', 0, 'space'),
  Game('fill', 'Cheio ou vazio?', 'Comparar recipientes', 'cup', 'yellow', 0, 'space'),
  Game('tower', 'Torres coloridas', 'Copiar um modelo', 'tower', 'purple', 0, 'logic'),
  Game('socks', 'Meias companheiras', 'Combinar cores e detalhes', 'socks', 'pink', 0, 'logic'),
  Game('oneeach', 'Um para cada amigo', 'Relacionar um a um', 'count', 'peach', 0, 'math'),
  Game('vocabulary', 'Quem foi chamado?', 'Ouvir e reconhecer', 'book', 'green', 0, 'language'),
  Game('colorpath', 'Caminho das cores', 'Continuar uma repetição', 'colorpath', 'pink', 0, 'logic'),
];

Game gameById(String id) => games.firstWhere((g) => g.id == id, orElse: () => games[0]);

const categories = [
  ('all', 'Todas'),
  ('math', 'Matemática'),
  ('logic', 'Lógica'),
  ('space', 'Formas e espaço'),
  ('language', 'Letras e histórias'),
];

class _Color {
  final String id, name, hex;
  const _Color(this.id, this.name, this.hex);
}

const _colors = [
  _Color('coral', 'vermelha', '#eb6d58'),
  _Color('blue', 'azul', '#589cd4'),
  _Color('yellow', 'amarela', '#efbd46'),
  _Color('green', 'verde', '#60a786'),
];
const shapes = ['circle', 'triangle', 'square', 'star'];

String shapeName(String s) => switch (s) {
      'circle' => 'círculo',
      'triangle' => 'triângulo',
      'square' => 'quadrado',
      _ => 'estrela',
    };

class _Word {
  final String word, icon;
  final List<String> syl;
  const _Word(this.word, this.icon, this.syl);
}

const _words = [
  _Word('SOL', 'sun', ['SOL']),
  _Word('FLOR', 'flower', ['FLOR']),
  _Word('MAÇÃ', 'apple', ['MA', 'ÇÃ']),
  _Word('GATO', 'cat', ['GA', 'TO']),
  _Word('BOLA', 'ball', ['BO', 'LA']),
  _Word('LUA', 'moon', ['LU', 'A']),
  _Word('CASA', 'house', ['CA', 'SA']),
  _Word('PEIXE', 'fish', ['PEI', 'XE']),
  _Word('COELHO', 'bunny', ['CO', 'E', 'LHO']),
  _Word('PÁSSARO', 'bird', ['PÁS', 'SA', 'RO']),
];

class _Story {
  final String text, question, icon, answer;
  final List<String> choices, labels;
  const _Story(this.text, this.question, this.icon, this.answer, this.choices, this.labels);
}

const _stories = [
  _Story('Nina, a coelha, encontrou uma maçã no jardim. Ela dividiu a maçã com seu amigo passarinho.', 'Qual fruta Nina dividiu?', 'bunny', 'apple', ['apple', 'flower', 'ball'], ['maçã', 'flor', 'bola']),
  _Story('O passarinho Tito recolheu uma flor amarela. Depois, levou a flor para perto de sua casa.', 'O que Tito recolheu?', 'bird', 'flower', ['flower', 'apple', 'fish'], ['flor', 'maçã', 'peixe']),
  _Story('O gato Bento brincava com uma bola. Quando o sol apareceu, Bento levou a bola para o jardim.', 'Com o que Bento brincava?', 'cat', 'ball', ['ball', 'apple', 'flower'], ['bola', 'maçã', 'flor']),
  _Story('A lua apareceu no céu. Nina, a coelha, guardou seus brinquedos e foi descansar.', 'O que apareceu no céu?', 'bunny', 'moon', ['moon', 'sun', 'flower'], ['lua', 'sol', 'flor']),
  _Story('Tito viu um peixe nadando no lago. O peixe passou por uma pedra e mergulhou devagar.', 'Qual animal estava no lago?', 'bird', 'fish', ['fish', 'cat', 'bunny'], ['peixe', 'gato', 'coelho']),
  _Story('Bento encontrou Nina em frente à casa. O gato e a coelha entraram juntos para brincar.', 'Onde os amigos entraram?', 'cat', 'house', ['house', 'flower', 'apple'], ['casa', 'flor', 'maçã']),
];

const _objects = [
  ('apple', 'a maçã'), ('flower', 'a flor'), ('bird', 'o passarinho'), ('bunny', 'o coelho'), ('cat', 'o gato'),
  ('fish', 'o peixe'), ('ball', 'a bola'), ('moon', 'a lua'), ('house', 'a casa'), ('sun', 'o sol'),
];

// ---------------------------------------------------------------- modelo

class Visual {
  final String type; // icon number text swatch quantity sized position sock cup container tower
  String kind;
  String color;
  final List<String> colors;
  double number;
  final String text;
  Visual(this.type, {this.kind = '', this.color = '', this.colors = const [], this.number = 0, this.text = ''});
}

class Option {
  final String id, label;
  final Visual visual;
  Option(this.id, this.label, this.visual);
}

class Scene {
  final String type;
  String color;
  final String shape, icon, text, word, pattern;
  final List<String> items;
  final List<int?> numbers;
  final int a, b, count;
  Scene(this.type,
      {this.color = '', this.shape = '', this.icon = '', this.text = '', this.word = '', this.pattern = '',
      this.items = const [], this.numbers = const [], this.a = 0, this.b = 0, this.count = 0});
}

class Card {
  final String kind;
  final int pair;
  const Card(this.kind, this.pair);
}

class Presentation {
  String surface = '#fffaf0', itemColor = '#e87961';
  int radius = 20, itemGap = 4;
  List<int> rotation = List.filled(24, 0);
}

class Question {
  final String game;
  String prompt = '', hint = '', story = '';
  bool memory = false;
  List<Option> options = [];
  int answer = 0;
  List<Card> cards = [];
  Scene? scene;
  final Presentation presentation = Presentation();
  Question(this.game);

  void setOptions(Rng r, String correct, List<Option> items) {
    options = r.shuffle(items);
    answer = math.max(0, options.indexWhere((o) => o.id == correct));
  }
}

Visual _icon(String kind, [String color = '']) => Visual('icon', kind: kind, color: color);
Option _num(int n) => Option('$n', '$n', Visual('number', number: n.toDouble()));
List<int> _range(int lo, int hi, int except) => [for (var i = lo; i <= hi; i++) if (i != except) i];
List<T> _take<T>(List<T> xs, int n) => xs.length > n ? xs.sublist(0, n) : xs;

// ---------------------------------------------------------------- jogos

bool _toddler(Question q, String game, int level, int seed) {
  final r = Rng(seed);
  final choices = level == 0 ? 2 : 3;
  switch (game) {
    case 'twins' || 'vocabulary':
      final ti = r.below(_objects.length);
      final (tid, tname) = _objects[ti];
      final twins = game == 'twins';
      q.prompt = twins ? 'Qual desenho é igual ao modelo?' : 'Encontre $tname.';
      q.hint = twins ? 'Olhe o desenho grande e procure outro igual.' : 'Vamos procurar $tname juntos.';
      q.scene = twins ? Scene('object-model', icon: tid) : Scene('look');
      final others = [for (var i = 0; i < _objects.length; i++) if (i != ti) i];
      final idx = [ti, ..._take(r.shuffle(others), choices - 1)];
      q.setOptions(r, tid, [for (final i in idx) Option(_objects[i].$1, _objects[i].$2, _icon(_objects[i].$1))]);
    case 'inside':
      final target = r.pick(['inside', 'outside']);
      q.prompt = target == 'inside' ? 'Em qual desenho a flor está dentro da caixa?' : 'Em qual desenho a flor está fora da caixa?';
      q.hint = target == 'inside' ? 'Procure a flor no espaço de dentro da caixa.' : 'Procure a flor ao lado de fora da caixa.';
      q.scene = Scene('look');
      q.setOptions(r, target, [
        for (final p in ['inside', 'outside'])
          Option(p, p == 'inside' ? 'Flor dentro da caixa' : 'Flor fora da caixa', Visual('container', kind: p))
      ]);
    case 'fill':
      final target = r.pick([0, 1]);
      q.prompt = target == 1 ? 'Qual copo está cheio?' : 'Qual copo está vazio?';
      q.hint = target == 1 ? 'Procure o copo com água até perto da borda.' : 'Procure o copo sem água.';
      q.scene = Scene('look');
      final amounts = level == 0 ? [0.0, 1.0] : [0.0, 0.5, 1.0];
      q.setOptions(r, '$target', [
        for (final a in amounts)
          Option(a == 0.5 ? '0.5' : '${a.toInt()}', a == 1 ? 'Copo cheio' : a == 0 ? 'Copo vazio' : 'Copo pela metade',
              Visual('cup', number: a))
      ]);
    case 'tower':
      final palette = [for (final i in _take(r.shuffle([0, 1, 2, 3]), level == 0 ? 2 : 3)) _colors[i].hex];
      q.prompt = 'Qual torre é igual à torre do modelo?';
      q.hint = 'Olhe a cor dos blocos de baixo para cima.';
      q.scene = Scene('tower', items: palette);
      final variants = [palette, palette.reversed.toList(), if (choices == 3) [palette[1], palette[0], palette[2]]];
      q.setOptions(r, palette.join(':'), [
        for (var i = 0; i < variants.length; i++) Option(variants[i].join(':'), 'Torre ${i + 1}', Visual('tower', colors: variants[i]))
      ]);
    case 'socks':
      final color = _colors[r.below(4)].hex;
      final other = r.pick([for (final c in _colors) if (c.hex != color) c.hex]);
      final pattern = r.pick(['dots', 'stripes']);
      q.prompt = 'Qual meia faz par com esta?';
      q.hint = 'Procure a mesma cor e o mesmo desenho na meia.';
      q.scene = Scene('sock', color: color, pattern: pattern);
      final variants = [(color, pattern), (other, pattern), if (choices == 3) (color, pattern == 'dots' ? 'stripes' : 'dots')];
      q.setOptions(r, '$color$pattern', [
        for (var i = 0; i < variants.length; i++)
          Option('${variants[i].$1}${variants[i].$2}', 'Meia ${i + 1}', Visual('sock', color: variants[i].$1, kind: variants[i].$2))
      ]);
    case 'oneeach':
      final n = 1 + r.below(level == 0 ? 3 : 5);
      q.prompt = 'Qual grupo tem uma maçã para cada coelho?';
      q.hint = 'Dê uma maçã para um coelho. Depois, uma para o próximo.';
      q.scene = Scene('friends', count: n);
      final ns = [n, ..._take(r.shuffle(_range(1, level == 0 ? 3 : 6, n)), choices - 1)];
      q.setOptions(r, '$n', [for (final c in ns) Option('$c', 'Grupo com $c maçãs', Visual('quantity', number: c.toDouble()))]);
    case 'colorpath':
      final ab = _take(r.shuffle([0, 1, 2, 3]), 2);
      final a = _colors[ab[0]], b = _colors[ab[1]];
      q.prompt = 'Qual cor continua o caminho?';
      q.hint = 'As cores aparecem uma de cada vez e depois se repetem.';
      q.scene = Scene('color-sequence', items: [a.hex, b.hex, a.hex]);
      q.setOptions(r, b.id, [for (final c in [a, b]) Option(c.id, 'Cor ${c.name}', Visual('swatch', color: c.hex))]);
    default:
      return false;
  }
  return true;
}

bool _extra(Question q, String game, int level, int seed) {
  final r = Rng(seed);
  final tk = level > 0 ? 3 : 2;
  void numbers(int answer, int lo, int hi) =>
      q.setOptions(r, '$answer', [_num(answer), for (final n in _take(r.shuffle(_range(lo, hi, answer)), tk)) _num(n)]);
  switch (game) {
    case 'compare':
      final counts = _take(r.shuffle(_range(1, [3, 6, 10][level], -1)), 3);
      final want = r.pick(['more', 'less']);
      final target = want == 'more' ? counts.reduce(math.max) : counts.reduce(math.min);
      q.prompt = want == 'more' ? 'Qual grupo tem mais maçãs?' : 'Qual grupo tem menos maçãs?';
      q.hint = 'Conte as maçãs de cada grupo e compare.';
      q.scene = Scene('look');
      q.setOptions(r, '$target', [for (final n in counts) Option('$n', 'Grupo com $n maçãs', Visual('quantity', number: n.toDouble()))]);
    case 'sizes':
      final shape = r.pick(shapes);
      final want = r.pick(['big', 'small']);
      final sizes = r.shuffle([0.46, 0.7, 1.0]);
      q.prompt = want == 'big' ? 'Qual desenho é o maior?' : 'Qual desenho é o menor?';
      q.hint = 'As formas são iguais. Compare o espaço que cada uma ocupa.';
      q.scene = Scene('look');
      q.setOptions(r, want == 'big' ? '1' : '0.46', [
        for (final s in sizes)
          Option(s == 1.0 ? '1' : '$s', s == 1.0 ? 'Desenho grande' : s == 0.46 ? 'Desenho pequeno' : 'Desenho médio',
              Visual('sized', kind: shape, number: s, color: '#7aaa8b'))
      ]);
    case 'position':
      final dirs = level == 0 ? ['above', 'below'] : ['above', 'below', 'left', 'right'];
      final target = r.pick(dirs);
      String name(String d) => switch (d) { 'above' => 'acima', 'below' => 'abaixo', 'left' => 'à esquerda', _ => 'à direita' };
      q.prompt = 'Em qual desenho a flor está ${name(target)} da caixa?';
      q.hint = switch (target) {
        'left' => 'A esquerda fica deste lado: ←.',
        'right' => 'A direita fica deste lado: →.',
        'above' => 'Procure a flor na parte de cima da caixa.',
        _ => 'Procure a flor na parte de baixo da caixa.',
      };
      q.scene = Scene('look');
      q.setOptions(r, target, [for (final d in dirs) Option(d, 'Flor ${name(d)} da caixa', Visual('position', kind: d))]);
    case 'shadow':
      final pool = shapes.sublist(0, level > 0 ? 4 : 3);
      final target = r.pick(pool);
      q.prompt = 'Qual desenho combina com esta sombra?';
      q.hint = 'Observe o contorno, sem se preocupar com a cor.';
      q.scene = Scene('shape', shape: target, color: '#344c47');
      q.setOptions(r, target, [for (var i = 0; i < pool.length; i++) Option(pool[i], shapeName(pool[i]), _icon(pool[i], _colors[i].hex))]);
    case 'sort':
      const animals = ['bird', 'bunny', 'cat', 'fish'], plants = ['apple', 'flower', 'leaf'];
      final isAnimals = r.below(2) == 0;
      final target = r.pick(isAnimals ? animals : plants);
      String label(String k) => const {
            'bird': 'passarinho', 'bunny': 'coelho', 'cat': 'gato', 'fish': 'peixe', 'apple': 'maçã', 'flower': 'flor'
          }[k] ??
          'folha';
      q.prompt = 'Qual destes desenhos é ${isAnimals ? 'um animal' : 'uma planta ou parte de uma planta'}?';
      q.hint = isAnimals ? 'Procure um ser vivo que é um animal.' : 'Folhas, flores e frutos fazem parte de plantas.';
      q.scene = Scene('look');
      q.setOptions(r, target, [
        Option(target, label(target), _icon(target)),
        for (final k in _take(r.shuffle(isAnimals ? plants : animals), 2)) Option(k, label(k), _icon(k)),
      ]);
    case 'letters':
      final alphabet = 'ABCDEFGHJKLMNPRSTUVZ'.split('');
      final t = r.pick(alphabet);
      final lo = t.toLowerCase();
      q.prompt = 'Qual é a letra minúscula que combina com $t?';
      q.hint = 'A letra $t também pode ser escrita assim: $lo.';
      q.scene = Scene('letter', text: t);
      final cs = [t, ..._take(r.shuffle([for (final c in alphabet) if (c != t) c]), 3)];
      q.setOptions(r, t, [for (final c in cs) Option(c, 'Letra ${c.toLowerCase()}', Visual('text', text: c.toLowerCase()))]);
    case 'initial':
      final w = _words[r.below(_words.length)];
      final first = w.word[0];
      q.prompt = 'Qual é a primeira letra de ${w.word.toLowerCase()}?';
      q.hint = 'Observe a primeira letra: $first.';
      q.scene = Scene('word-picture', word: w.word, icon: w.icon);
      final cs = [first, ..._take(r.shuffle([for (final c in 'ABCDEFGHILMNOPRSTUV'.split('')) if (c != first) c]), 3)];
      q.setOptions(r, first, [for (final c in cs) Option(c, 'Letra $c', Visual('text', text: c))]);
    case 'vowels':
      final v = r.pick('AEIOU'.split(''));
      final others = _take(r.shuffle('BCDFGLMNPRSTV'.split('')), 3);
      q.prompt = 'Qual destas letras é uma vogal?';
      q.hint = 'As vogais são A, E, I, O e U.';
      q.scene = Scene('letter', text: 'A E I O U');
      q.setOptions(r, v, [for (final c in [v, ...others]) Option(c, 'Letra $c', Visual('text', text: c))]);
    case 'syllables':
      final w = _words[r.below(_words.length)];
      q.prompt = 'Quantos pedacinhos tem a palavra ${w.word.toLowerCase()}?';
      q.hint = 'Fale devagar: ${w.syl.join(', ')}. Conte uma batida para cada pedacinho.';
      q.scene = Scene('word-picture', word: w.word, icon: w.icon);
      numbers(w.syl.length, 1, 5);
    case 'word':
      final ti = r.below(_words.length);
      final w = _words[ti];
      q.prompt = 'Qual palavra combina com o desenho?';
      q.hint = 'O desenho é ${w.word.toLowerCase()}. Procure essa palavra.';
      q.scene = Scene('word-picture', icon: w.icon);
      final idx = [ti, ..._take(r.shuffle([for (var i = 0; i < _words.length; i++) if (i != ti) i]), 3)];
      q.setOptions(r, w.word, [for (final i in idx) Option(_words[i].word, _words[i].word, Visual('text', text: _words[i].word))]);
    case 'subtract':
      final a = r.range(2, level == 2 ? 12 : 5), b = r.range(1, a);
      q.prompt = 'Havia $a maçãs. Tiramos $b. Quantas ficaram?';
      q.hint = 'As maçãs riscadas saíram do grupo. Conte as que ficaram.';
      q.scene = Scene('subtract', a: a, b: b);
      numbers(a - b, 0, level == 2 ? 15 : 7);
    case 'missing':
      final hi = level == 2 ? 9 : 3;
      final a = r.range(1, hi), b = r.range(1, hi);
      q.prompt = 'Qual número está escondido na adição?';
      q.hint = 'Quanto falta para ir de $a até ${a + b}?';
      q.scene = Scene('equation', a: a, count: a + b);
      numbers(b, 0, level == 2 ? 12 : 6);
    case 'tenframe':
      final n = r.range(1, 10);
      q.prompt = 'Quantos lugares estão ocupados na casinha?';
      q.hint = 'A primeira linha tem cinco lugares. Conte os círculos preenchidos.';
      q.scene = Scene('tenframe', count: n);
      numbers(n, 0, 12);
    case 'tens':
      final tens = r.range(1, level == 2 ? 4 : 2), units = r.range(0, 9);
      q.prompt = 'Qual número estas dezenas e unidades formam?';
      q.hint = 'Cada barra tem dez quadradinhos. Há $tens dezenas e $units unidades.';
      q.scene = Scene('tens', a: tens, b: units);
      numbers(tens * 10 + units, 10, 59);
    case 'clock':
      final hour = r.range(1, 12);
      q.prompt = 'Que horas o relógio está marcando?';
      q.hint = 'O ponteiro grande está no doze. O ponteiro pequeno mostra a hora.';
      q.scene = Scene('clock', count: hour);
      numbers(hour, 1, 12);
    case 'story':
      final s = _stories[r.below(_stories.length)];
      q.prompt = s.question;
      q.story = s.text;
      q.hint = 'Ouça a história outra vez e observe o que aconteceu.';
      q.scene = Scene('story', text: s.text, icon: s.icon);
      q.setOptions(r, s.answer, [for (var i = 0; i < 3; i++) Option(s.choices[i], s.labels[i], _icon(s.choices[i]))]);
    default:
      return false;
  }
  return true;
}

Question _base(String game, String group, int seed) {
  final r = Rng(seed);
  final level = groupIndex(group);
  final g = groups[level];
  final q = Question(game);
  switch (game) {
    case 'colors':
      final ti = r.below(4);
      final t = _colors[ti];
      q.prompt = 'Encontre a flor de cor ${t.name}.';
      q.hint = 'Observe a cor das pétalas.';
      q.scene = Scene('color', color: t.hex);
      final idx = [ti, ..._take(r.shuffle([for (var i = 0; i < 4; i++) if (i != ti) i]), level > 0 ? 3 : 2)];
      q.setOptions(r, t.id, [for (final i in idx) Option(_colors[i].id, _colors[i].name, _icon('flower', _colors[i].hex))]);
    case 'shapes':
      final t = r.pick(shapes.sublist(0, level > 0 ? 4 : 3));
      q.prompt = 'Qual forma é igual à do desenho?';
      q.hint = 'Procure o ${shapeName(t)}.';
      q.scene = Scene('shape', shape: t, color: '#559d8b');
      final list = [..._take(r.shuffle([for (final s in shapes) if (s != t) s]), level > 0 ? 3 : 2), t];
      q.setOptions(r, t, [for (final s in list) Option(s, shapeName(s), _icon(s, '#e9a94b'))]);
    case 'count':
      final n = r.range(1, g.max);
      q.prompt = 'Quantas maçãs você consegue contar?';
      q.hint = 'Toque em cada maçã e conte devagar.';
      q.scene = Scene('items', count: n);
      q.setOptions(r, '$n', [_num(n), for (final v in _take(r.shuffle(_range(1, g.max + 2, n)), level > 0 ? 3 : 2)) _num(v)]);
    case 'memory':
      q.memory = true;
      q.prompt = 'Encontre os pares que são iguais.';
      q.hint = 'Vire duas cartas. Lembre onde cada desenho está.';
      final kinds = _take(r.shuffle(['apple', 'bird', 'flower', 'sun']), g.pairs);
      q.cards = r.shuffle([for (var p = 0; p < kinds.length; p++) ...[Card(kinds[p], p), Card(kinds[p], p)]]);
    case 'pattern':
      final a = r.pick(shapes);
      final b = r.pick([for (final s in shapes) if (s != a) s]);
      final seq = level == 2
          ? r.pick([
              [a, a, b, a, a, b],
              [a, b, b, a, b, b],
              [a, b, a, b, a, b]
            ])
          : [a, b, a, b];
      q.prompt = 'Qual desenho completa a sequência?';
      q.hint = 'Diga os nomes das formas e procure a repetição.';
      q.scene = Scene('sequence', items: seq.sublist(0, seq.length - 1));
      q.setOptions(r, seq.last, [for (final s in shapes) Option(s, shapeName(s), _icon(s, '#8b7bbd'))]);
    case 'odd':
      final normal = r.pick(shapes);
      final other = r.pick([for (final s in shapes) if (s != normal) s]);
      final size = level > 0 ? 4 : 3;
      final index = r.range(0, size - 1);
      q.prompt = 'Qual desenho tem uma forma diferente?';
      q.hint = 'A cor é igual. Compare o contorno dos desenhos.';
      q.scene = Scene('look');
      q.options = [
        for (var i = 0; i < size; i++)
          Option('$i', 'Desenho ${i + 1}: ${shapeName(i == index ? other : normal)}', _icon(i == index ? other : normal, '#599b84'))
      ];
      q.answer = index;
    case 'sum':
      final hi = level == 2 ? 9 : 3;
      final a = r.range(1, hi), b = r.range(1, hi), n = a + b;
      q.prompt = 'Juntando os dois grupos, quantas maçãs temos?';
      q.hint = 'Conte as maçãs de um grupo e continue no outro.';
      q.scene = Scene('sum', a: a, b: b);
      q.setOptions(r, '$n', [_num(n), for (final v in _take(r.shuffle(_range(1, math.max(n + 3, 10), n)), 3)) _num(v)]);
    case 'order':
      final step = level == 2 ? r.pick([1, 2, 5]) : 1;
      final start = r.range(1, level == 2 ? 10 : 4);
      final index = r.range(1, 3);
      final seq = [for (var i = 0; i < 4; i++) start + i * step];
      final n = seq[index];
      q.prompt = 'Qual número falta na escada?';
      q.hint = step == 1 ? 'Conte os números em ordem.' : 'Conte de $step em $step.';
      q.scene = Scene('numbers', numbers: [for (var i = 0; i < 4; i++) i == index ? null : seq[i]]);
      q.setOptions(r, '$n', [_num(n), for (final v in _take(r.shuffle(_range(1, seq[3] + 3, n)), 3)) _num(v)]);
    default:
      if (!_toddler(q, game, level, seed) && !_extra(q, game, level, seed)) q.prompt = 'Atividade desconhecida';
  }
  return q;
}

Question _vary(Question q, int seed) {
  final r = Rng(seed ^ 0xa4b1c987);
  const palette = ['#e97e68', '#5799be', '#e0b849', '#68a585', '#9b86bd'];
  final p = q.presentation;
  p.surface = r.pick(['#fffaf0', '#f2f7ed', '#eff6f7', '#faf1ed', '#f5f0f8']);
  p.radius = 20 + r.below(10);
  p.itemColor = r.pick(['#e87961', '#83ad68', '#e6b85c']);
  p.itemGap = 4 + r.below(5);
  p.rotation = [for (var i = 0; i < 24; i++) r.below(17) - 8];
  if (q.game == 'colors') {
    for (final o in q.options) {
      o.visual.color = r.pick(switch (o.id) {
        'coral' => ['#e46b5b', '#df6357', '#e67a69'],
        'blue' => ['#548fc5', '#5b9cce', '#478bbb'],
        'yellow' => ['#edc14c', '#e6b73e', '#efc656'],
        _ => ['#60a277', '#6caa80', '#56976c'],
      });
    }
    q.scene?.color = q.options[q.answer].visual.color;
  }
  final shared = r.pick(palette);
  for (final o in q.options) {
    final v = o.visual;
    if (v.type == 'icon' && shapes.contains(v.kind)) {
      v.color = q.game == 'odd' ? shared : r.pick(palette);
    } else if (v.type == 'sized') {
      final base = v.number;
      v.number = base < 0.5 ? 0.4 + r.next() * 0.08 : base < 0.8 ? 0.63 + r.next() * 0.08 : 0.91 + r.next() * 0.08;
      v.color = shared;
    }
  }
  if (q.game != 'shadow' && q.scene?.type == 'shape') q.scene!.color = r.pick(palette);
  return q;
}

Question generate(String game, String group, int seed) => _vary(_base(game, group, seed), seed);

List<Game> catalog(int level, int seed) => Rng(seed).shuffle([for (final g in games) if (g.min <= level) g]);
