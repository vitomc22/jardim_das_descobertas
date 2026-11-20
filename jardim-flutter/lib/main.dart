// Jardim das Descobertas — versão Flutter.
import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math' as math;
import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_svg/flutter_svg.dart';

import 'art.dart' as art;
import 'curriculum.dart' as cur;

const _native = MethodChannel('jardim');
const goal = 5;

// ---------------------------------------------------------------- paleta (style.css)
const ink = Color(0xFF294B43), muted = Color(0xFF6D7D70), paper = Color(0xFFF8F6EE), green = Color(0xFF347B65);
const greenShadow = Color(0xFF26614F), line = Color(0xFFE2E6D9), quiet = Color(0xFFEEEEE4), quietBorder = Color(0xFFDEDFD2);
const eyebrowC = Color(0xFF7F8C73), em = Color(0xFF56856B), success = Color(0xFF387657), gold = Color(0xFFE9BD68);

Color hex(String s) => Color(int.parse('FF${s.replaceFirst('#', '')}', radix: 16));
Color cardBg(String c) => hex(const {'blue': '#deedf2', 'yellow': '#faedc8', 'green': '#e0eddc', 'purple': '#e8e2f1', 'peach': '#f6e7d5'}[c] ?? '#f7e1db');

TextStyle serif(double size, [Color c = ink]) => TextStyle(fontFamily: 'serif', fontWeight: FontWeight.bold, fontSize: size, color: c, height: 1.15);
TextStyle sans(double size, [Color c = ink]) => TextStyle(fontSize: size, color: c);
const eyebrowStyle = TextStyle(fontSize: 12, letterSpacing: 2, fontWeight: FontWeight.bold, color: eyebrowC);

// ---------------------------------------------------------------- estado

class Stat {
  int solved = 0, tries = 0, sets = 0, rounds = 0;
  Map<String, int> toJson() => {'solved': solved, 'tries': tries, 'sets': sets, 'rounds': rounds};
}

class Daily {
  int solved = 0;
  bool get completed => solved >= goal;
}

String today() {
  final n = DateTime.now();
  return '${n.year}-${n.month.toString().padLeft(2, '0')}-${n.day.toString().padLeft(2, '0')}';
}

class JardimState extends ChangeNotifier {
  String screen = 'home', group = 'little', category = 'all', date = today(), modal = '';
  bool voice = true;
  final stats = <String, Stat>{};
  final daily = <String, Daily>{};
  int entrySeed = cur.entrySeed();
  String? _path;

  // sessão
  String game = '';
  int round = 0, length = 0, wiggle = -1, wiggleSerial = 0, _flipToken = 0;
  cur.Question? q;
  bool correct = false, busy = false;
  String feedback = 'idle';
  List<int> revealed = [], matched = [];
  final counted = <int>{};

  String key(String g) => '$group:$g';
  Daily dailyOf(String g) => daily[key(g)] ?? Daily();
  Stat statOf(String g) => stats[key(g)] ?? Stat();
  int get total => stats.values.fold(0, (a, s) => a + s.solved);
  int get level => cur.groupIndex(group);
  List<cur.Game> get catalogAll => cur.catalog(level, cur.seedFor(date, group, 'catalog', 0, entrySeed));
  List<cur.Game> get cards => [
    for (final g in catalogAll)
      if (category == 'all' || g.category == category) g,
  ];

  Future<void> load() async {
    try {
      final dir = await _native.invokeMethod<String>('filesDir');
      _path = '$dir/jardim-v1.json';
      final f = File(_path!);
      if (await f.exists()) {
        final o = jsonDecode(await f.readAsString()) as Map<String, dynamic>;
        bool valid(String k) {
          final p = k.split(':');
          return p.length == 2 && cur.groups.any((g) => g.id == p[0]) && cur.games.any((g) => g.id == p[1]);
        }

        if (o['version'] == 1 && cur.groups.any((g) => g.id == o['group'])) {
          group = o['group'];
          voice = o['voice'] != false;
          (o['stats'] as Map<String, dynamic>? ?? {}).forEach((k, v) {
            if (valid(k)) {
              stats[k] = Stat()
                ..solved = v['solved'] ?? 0
                ..tries = v['tries'] ?? 0
                ..sets = v['sets'] ?? 0
                ..rounds = v['rounds'] ?? 0;
            }
          });
          final d = o['daily'] as Map<String, dynamic>? ?? {};
          if (d['date'] == date) {
            (d['activities'] as Map<String, dynamic>? ?? {}).forEach((k, v) {
              if (valid(k)) daily[k] = Daily()..solved = math.min(goal, (v['solved'] ?? 0) as int);
            });
          }
        }
      }
    } catch (_) {}
    notifyListeners();
  }

  Timer? _saveTimer;
  void save() {
    // Coalesce e grava de forma assíncrona (I/O fora da thread de UI).
    _saveTimer?.cancel();
    _saveTimer = Timer(const Duration(milliseconds: 50), () {
      if (_path == null) return;
      final o = {
        'version': 1,
        'group': group,
        'voice': voice,
        'stats': {for (final e in stats.entries) e.key: e.value.toJson()},
        'daily': {
          'date': date,
          'activities': {
            for (final e in daily.entries) e.key: {'solved': e.value.solved, 'completed': e.value.completed},
          },
        },
      };
      final tmp = File('$_path.tmp');
      tmp.writeAsString(jsonEncode(o)).then((_) => tmp.rename(_path!)).catchError((_) => tmp);
    });
  }

  bool ensureDay() {
    final t = today();
    if (t == date) return false;
    date = t;
    daily.clear();
    save();
    return true;
  }

  void speak(String text) {
    if (voice) _native.invokeMethod('speak', text);
  }

  void stopVoice() => _native.invokeMethod('stop');

  void home() {
    ensureDay();
    stopVoice();
    screen = 'home';
    modal = '';
    notifyListeners();
  }

  void setGroup(String g) {
    group = g;
    category = 'all';
    save();
    notifyListeners();
  }

  void setCategory(String c) {
    category = c;
    notifyListeners();
  }

  void start(String g) {
    ensureDay();
    game = g;
    final d = dailyOf(g);
    length = d.completed ? goal : goal - d.solved;
    round = 0;
    screen = 'game';
    _next();
  }

  void _next() {
    ensureDay();
    final s = stats.putIfAbsent(key(game), Stat.new);
    s.rounds++;
    save();
    q = cur.generate(game, group, cur.seedFor(date, group, game, s.rounds, entrySeed));
    correct = false;
    feedback = 'idle';
    revealed = [];
    matched = [];
    counted.clear();
    wiggle = -1;
    busy = false;
    _flipToken++;
    notifyListeners();
    speak('${q!.story.isEmpty ? '' : '${q!.story} '}${q!.prompt}');
  }

  void _recordTry() {
    stats.putIfAbsent(key(game), Stat.new).tries++;
    save();
  }

  void _correct() {
    if (correct) return;
    correct = true;
    feedback = 'success';
    ensureDay();
    final d = daily.putIfAbsent(key(game), Daily.new);
    d.solved = math.min(goal, d.solved + 1);
    stats.putIfAbsent(key(game), Stat.new).solved++;
    save();
    notifyListeners();
    speak('Você descobriu! Muito bem.');
  }

  void answer(int i) {
    if (correct) return;
    _recordTry();
    if (i == q!.answer) return _correct();
    wiggle = i;
    final serial = ++wiggleSerial;
    feedback = 'retry';
    notifyListeners();
    speak('Vamos tentar de novo? ${q!.hint}');
    Timer(const Duration(milliseconds: 500), () {
      if (serial == wiggleSerial) {
        wiggle = -1;
        notifyListeners();
      }
    });
  }

  void flip(int i) {
    if (busy || correct || revealed.contains(i) || matched.contains(i)) return;
    revealed = [...revealed, i];
    if (revealed.length < 2) return notifyListeners();
    _recordTry();
    final a = revealed[0], d = revealed[1];
    if (q!.cards[a].pair == q!.cards[d].pair) {
      matched = [...matched, a, d];
      revealed = [];
      if (matched.length == q!.cards.length) return _correct();
      notifyListeners();
      speak('Um par!');
    } else {
      busy = true;
      notifyListeners();
      final token = ++_flipToken;
      Timer(const Duration(milliseconds: 1200), () {
        if (token != _flipToken) return;
        revealed = [];
        busy = false;
        notifyListeners();
      });
    }
  }

  void next() {
    if (!correct) return;
    round++;
    if (round == length) {
      ensureDay();
      stopVoice();
      stats.putIfAbsent(key(game), Stat.new).sets++;
      save();
      screen = 'finish';
      notifyListeners();
      speak('${length == 1 ? 'Uma nova descoberta!' : '$length novas descobertas!'} Que tal fazer uma pausa?');
    } else {
      _next();
    }
  }

  void hint() {
    if (!correct) feedback = 'hint';
    notifyListeners();
    speak('${q!.story.isEmpty ? '' : '${q!.story} '}${q!.hint}');
  }

  void listen() => speak('${q!.story.isEmpty ? '' : '${q!.story} '}${q!.prompt}');

  void countItem(int i) {
    if (!counted.remove(i)) counted.add(i);
    notifyListeners();
  }

  void toggleVoice() {
    voice = !voice;
    save();
    if (!voice) {
      stopVoice();
    } else if (screen == 'game') {
      speak(q!.prompt);
    }
    notifyListeners();
  }

  void reset() {
    stats.clear();
    daily.clear();
    save();
    notifyListeners();
  }

  void setModal(String m) {
    if (m == 'gate') stopVoice();
    modal = m;
    notifyListeners();
  }

  void resume() {
    entrySeed = cur.entrySeed();
    ensureDay();
    notifyListeners();
  }
}

// ---------------------------------------------------------------- app

const _trace = bool.fromEnvironment('TRACE');

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  if (_trace) {
    // Diagnóstico: tempo de UI (build/layout) e de raster (GPU) de cada frame.
    WidgetsBinding.instance.addTimingsCallback((ts) {
      for (final t in ts) {
        debugPrint('FT ${t.buildDuration.inMicroseconds} ${t.rasterDuration.inMicroseconds} ${t.totalSpan.inMicroseconds}');
      }
    });
  }
  final state = JardimState()..load();
  runApp(JardimApp(state: state));
}

class JardimApp extends StatefulWidget {
  final JardimState state;
  const JardimApp({super.key, required this.state});
  @override
  State<JardimApp> createState() => _JardimAppState();
}

class _JardimAppState extends State<JardimApp> with WidgetsBindingObserver {
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    super.dispose();
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState s) {
    if (s == AppLifecycleState.resumed) {
      widget.state.resume();
    } else if (s == AppLifecycleState.paused) {
      widget.state.stopVoice();
    }
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Jardim das Descobertas',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        scaffoldBackgroundColor: paper,
        colorScheme: ColorScheme.fromSeed(seedColor: green),
        useMaterial3: true,
      ),
      // Mesmo tamanho de texto das outras versões, que ignoram a escala de
      // fonte do sistema (este tablet está em 115%).
      builder: (context, child) => MediaQuery(
        data: MediaQuery.of(context).copyWith(textScaler: TextScaler.noScaling),
        child: child!,
      ),
      home: ListenableBuilder(
        listenable: widget.state,
        builder: (context, _) => Root(s: widget.state),
      ),
    );
  }
}

class Root extends StatelessWidget {
  final JardimState s;
  const Root({super.key, required this.s});

  @override
  Widget build(BuildContext context) {
    return PopScope(
      canPop: false,
      onPopInvokedWithResult: (didPop, _) {
        if (s.modal.isNotEmpty) {
          s.setModal('');
        } else if (s.screen != 'home') {
          s.home();
        } else {
          SystemNavigator.pop();
        }
      },
      child: Scaffold(
        body: SafeArea(
          child: Stack(
            children: [
              Padding(
                padding: const EdgeInsets.fromLTRB(24, 16, 24, 0),
                child: switch (s.screen) {
                  'game' => GameScreen(s: s),
                  'finish' => FinishScreen(s: s),
                  _ => HomeScreen(s: s),
                },
              ),
              if (s.modal.isNotEmpty) ModalLayer(s: s),
            ],
          ),
        ),
      ),
    );
  }
}

// ---------------------------------------------------------------- componentes

/// Desenho SVG rasterizado UMA vez no tamanho exato (pixels físicos) e
/// guardado como ui.Image. O Impeller não tem cache de raster: com
/// SvgPicture os caminhos vetoriais eram refeitos a cada frame da rolagem
/// (raster ≈ 16 ms/frame medido no tablet).
class ArtCache {
  static final _done = <String, ui.Image>{};
  static final _pending = <String, Future<ui.Image>>{};

  static ui.Image? peek(String key) => _done[key];

  static Future<ui.Image> get(String svg, int pw, int ph) {
    final key = '$pw x $ph|$svg';
    return _pending[key] ??= () async {
      final info = await vg.loadPicture(SvgStringLoader(svg), null);
      final rec = ui.PictureRecorder();
      final canvas = Canvas(rec);
      canvas.scale(pw / info.size.width, ph / info.size.height);
      canvas.drawPicture(info.picture);
      info.picture.dispose();
      final img = await rec.endRecording().toImage(pw, ph);
      _done[key] = img;
      return img;
    }();
  }
}

class Art extends StatefulWidget {
  final String svg;
  final double width, height;
  const Art(this.svg, this.width, this.height, {super.key});
  @override
  State<Art> createState() => _ArtState();
}

class _ArtState extends State<Art> {
  ui.Image? img;
  String? key;

  void _resolve() {
    final dpr = MediaQuery.devicePixelRatioOf(context);
    final pw = (widget.width * dpr).ceil(), ph = (widget.height * dpr).ceil();
    final k = '$pw x $ph|${widget.svg}';
    if (k == key) return;
    key = k;
    img = ArtCache.peek(k);
    if (img == null) {
      ArtCache.get(widget.svg, pw, ph).then((i) {
        if (mounted && key == k) setState(() => img = i);
      });
    }
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _resolve();
  }

  @override
  void didUpdateWidget(Art old) {
    super.didUpdateWidget(old);
    _resolve();
  }

  @override
  Widget build(BuildContext context) => SizedBox(
    width: widget.width,
    height: widget.height,
    child: img == null ? null : RawImage(image: img, width: widget.width, height: widget.height, filterQuality: FilterQuality.medium),
  );
}

class Btn extends StatefulWidget {
  final String text, kind;
  final bool selected;
  final VoidCallback onTap;
  const Btn(this.text, {super.key, this.kind = 'quiet', this.selected = false, required this.onTap});
  @override
  State<Btn> createState() => _BtnState();
}

class _BtnState extends State<Btn> {
  bool down = false;
  @override
  Widget build(BuildContext context) {
    final k = widget.kind, sel = widget.selected;
    final primary = k == 'primary';
    final fill = primary
        ? green
        : k == 'danger'
        ? const Color(0xFFF2E2D8)
        : k == 'pill'
        ? (sel ? green : Colors.transparent)
        : k == 'tab'
        ? (sel ? const Color(0xFFE0EADA) : Colors.transparent)
        : (down ? line : quiet);
    final fg = primary
        ? Colors.white
        : k == 'danger'
        ? const Color(0xFF9C513C)
        : k == 'pill'
        ? (sel ? Colors.white : muted)
        : k == 'tab'
        ? (sel ? const Color(0xFF346950) : muted)
        : ink;
    final radius = BorderRadius.circular(
      primary
          ? 16
          : k == 'danger'
          ? 15
          : 24,
    );
    final face = AnimatedContainer(
      duration: const Duration(milliseconds: 60),
      transform: Matrix4.translationValues(0, down && primary ? 3 : 0, 0),
      height: primary
          ? 56
          : k == 'tab'
          ? 42
          : 46,
      padding: EdgeInsets.symmetric(horizontal: primary ? 24 : 17),
      decoration: BoxDecoration(
        color: fill,
        borderRadius: radius,
        border: k == 'quiet' || k == 'tab' ? Border.all(color: k == 'tab' ? (sel ? const Color(0xFF9DBA99) : line) : quietBorder) : null,
      ),
      child: Center(widthFactor: 1, child: Text(widget.text, style: primary || k == 'danger' ? serif(primary ? 18 : 16, fg) : sans(15, fg))),
    );
    return GestureDetector(
      onTapDown: (_) => setState(() => down = true),
      onTapCancel: () => setState(() => down = false),
      onTapUp: (_) => setState(() => down = false),
      onTap: widget.onTap,
      child: primary
          ? Stack(
              children: [
                Positioned.fill(
                  top: 4,
                  bottom: -4,
                  child: DecoratedBox(
                    decoration: BoxDecoration(color: greenShadow, borderRadius: radius),
                  ),
                ),
                face,
              ],
            )
          : face,
    );
  }
}

class Logo extends StatelessWidget {
  const Logo({super.key});
  @override
  Widget build(BuildContext context) => Row(
    mainAxisSize: MainAxisSize.min,
    children: [
      Art(art.icon('leaf'), 34, 34),
      const SizedBox(width: 8),
      Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          Text('jardim', style: serif(22, green)),
          Text('das descobertas', style: sans(11, muted)),
        ],
      ),
    ],
  );
}

class Header extends StatelessWidget {
  final JardimState s;
  const Header({super.key, required this.s});
  @override
  Widget build(BuildContext context) => SizedBox(
    height: 56,
    child: Row(
      children: [
        const Logo(),
        const Spacer(),
        Text('★ ${s.total} descobertas', style: sans(15, eyebrowC)),
        const SizedBox(width: 12),
        Btn('Área dos adultos', onTap: () => s.setModal('gate')),
      ],
    ),
  );
}

// ---------------------------------------------------------------- tela inicial

class HomeScreen extends StatefulWidget {
  final JardimState s;
  const HomeScreen({super.key, required this.s});
  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  final scroll = ScrollController();
  final activitiesKey = GlobalKey();

  @override
  Widget build(BuildContext context) {
    final s = widget.s;
    final all = s.catalogAll;
    final done = all.where((g) => s.dailyOf(g.id).completed).length;
    final pct = all.isEmpty ? 0 : (100 * done / all.length).round();
    return LayoutBuilder(
      builder: (context, box) {
        final w = box.maxWidth;
        final cols = math.max(1, ((w + 16) / (250 + 16)).floor());
        final cardW = (w - 16 * (cols - 1)) / cols;
        final gardenW = math.min(460.0, w * 0.45);
        return SingleChildScrollView(
          controller: scroll,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Header(s: s),
              const SizedBox(height: 18),
              Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text('BRINCAR • PENSAR • DESCOBRIR', style: eyebrowStyle),
                        const SizedBox(height: 6),
                        Text('Pequenas descobertas.', style: serif(40)),
                        Text('Grandes sorrisos.', style: serif(40, em)),
                        const SizedBox(height: 8),
                        Text('Um jardim de ideias para aprender brincando, um passo de cada vez.', style: sans(17, muted)),
                        const SizedBox(height: 16),
                        Btn(
                          'Vamos explorar →',
                          kind: 'primary',
                          onTap: () {
                            s.speak('Escolha uma atividade para começar.');
                            final ctx = activitiesKey.currentContext;
                            if (ctx != null) Scrollable.ensureVisible(ctx, duration: const Duration(milliseconds: 400));
                          },
                        ),
                      ],
                    ),
                  ),
                  const SizedBox(width: 20),
                  RepaintBoundary(child: Art(art.garden(), gardenW, gardenW * 300 / 520)),
                ],
              ),
              const SizedBox(height: 28),
              Wrap(
                key: activitiesKey,
                spacing: 16,
                runSpacing: 8,
                crossAxisAlignment: WrapCrossAlignment.center,
                children: [
                  Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      const Text('CADA IDADE, UMA DESCOBERTA', style: eyebrowStyle),
                      Text('O que vamos brincar hoje?', style: serif(30)),
                    ],
                  ),
                  for (final g in cur.groups) Btn(g.label, kind: 'pill', selected: g.id == s.group, onTap: () => s.setGroup(g.id)),
                ],
              ),
              const SizedBox(height: 6),
              Text('${cur.groups[s.level].name} • ${all.length} atividades gratuitas', style: sans(15, muted)),
              const SizedBox(height: 14),
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 22, vertical: 20),
                decoration: BoxDecoration(
                  color: const Color(0xFFEAF0E2),
                  border: Border.all(color: const Color(0xFFD4DFC7)),
                  borderRadius: BorderRadius.circular(22),
                ),
                child: Row(
                  children: [
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          const Text('SEU JARDIM DE HOJE', style: eyebrowStyle),
                          Text('$done de ${all.length} atividades feitas hoje', style: serif(21)),
                          Text('Novos desafios a cada dia. As marcações começam de novo diariamente.', style: sans(14, muted)),
                        ],
                      ),
                    ),
                    SizedBox(
                      width: 180,
                      height: 12,
                      child: ClipRRect(
                        borderRadius: BorderRadius.circular(6),
                        child: LinearProgressIndicator(value: pct / 100, backgroundColor: Colors.white, color: green),
                      ),
                    ),
                    const SizedBox(width: 10),
                    Text('$pct% hoje', style: sans(15)),
                  ],
                ),
              ),
              const SizedBox(height: 16),
              Wrap(
                spacing: 8,
                runSpacing: 8,
                children: [
                  for (final (id, label) in cur.categories)
                    if (id == 'all' || all.any((g) => g.category == id)) Btn(label, kind: 'tab', selected: id == s.category, onTap: () => s.setCategory(id)),
                ],
              ),
              const SizedBox(height: 16),
              Wrap(
                spacing: 16,
                runSpacing: 16,
                children: [
                  for (final g in s.cards)
                    RepaintBoundary(
                      child: GameCard(s: s, g: g, width: cardW),
                    ),
                ],
              ),
              const SizedBox(height: 20),
              const Divider(color: line),
              SizedBox(
                height: 48,
                child: Row(
                  children: [
                    Text('Feito para a curiosidade florescer.', style: sans(14, muted)),
                    const Spacer(),
                    Text('Gratuito · Offline · Sem anúncios', style: sans(14, muted)),
                  ],
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}

class GameCard extends StatelessWidget {
  final JardimState s;
  final cur.Game g;
  final double width;
  const GameCard({super.key, required this.s, required this.g, required this.width});

  @override
  Widget build(BuildContext context) {
    final d = s.dailyOf(g.id);
    final wide = art.tileWide(g.icon);
    // Sem Clip.antiAlias nem BoxShadow: no Impeller (GLES/Mali) ambos custam
    // passadas extras por cartão. A "sombra" é um retângulo sólido atrás.
    return GestureDetector(
      onTap: () => s.start(g.id),
      child: SizedBox(
        width: width,
        height: 294,
        child: Stack(
          children: [
            Positioned(
              left: 0,
              right: 0,
              top: 4,
              height: 290,
              child: DecoratedBox(
                decoration: BoxDecoration(color: const Color(0xFFE8E9DE), borderRadius: BorderRadius.circular(22)),
              ),
            ),
            Container(
              width: width,
              height: 290,
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(22),
                border: Border.all(color: const Color(0xFFE6E6D9)),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Container(
                    height: 140,
                    decoration: BoxDecoration(
                      color: cardBg(g.color),
                      borderRadius: const BorderRadius.vertical(top: Radius.circular(21)),
                    ),
                    child: Stack(
                      children: [
                        Center(child: Art(art.tile(g.icon), wide ? 180 : 110, wide ? 90 : 110)),
                        Positioned(
                          right: 16,
                          top: 12,
                          child: Transform.rotate(
                            angle: -math.pi / 4,
                            child: Text('→', style: sans(18)),
                          ),
                        ),
                      ],
                    ),
                  ),
                  Padding(
                    padding: const EdgeInsets.fromLTRB(16, 12, 16, 0),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        FittedBox(
                          fit: BoxFit.scaleDown,
                          alignment: Alignment.centerLeft,
                          child: Text(g.name, style: serif(19), maxLines: 1),
                        ),
                        Text(g.skill, style: sans(14, muted)),
                        const SizedBox(height: 8),
                        Container(
                          padding: const EdgeInsets.all(6),
                          decoration: BoxDecoration(color: d.completed ? const Color(0xFFF1F7E9) : null, borderRadius: BorderRadius.circular(9)),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                d.completed
                                    ? '✓ Feita hoje'
                                    : d.solved > 0
                                    ? 'Em andamento hoje'
                                    : 'Para descobrir hoje',
                                style: serif(14, d.completed ? success : ink),
                              ),
                              Text('${d.solved} de 5 descobertas', style: sans(13, muted)),
                              const SizedBox(height: 4),
                              Row(
                                children: [
                                  for (var i = 0; i < goal; i++)
                                    Container(
                                      width: 10,
                                      height: 10,
                                      margin: const EdgeInsets.only(right: 6),
                                      decoration: BoxDecoration(
                                        shape: BoxShape.circle,
                                        color: i < d.solved ? green : null,
                                        border: i < d.solved ? null : Border.all(color: const Color(0xFFC9D3BF), width: 1.5),
                                      ),
                                    ),
                                ],
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

// ---------------------------------------------------------------- tela do jogo

String visualSvg(cur.Visual v) => switch (v.type) {
  'icon' || 'sized' => art.icon(v.kind, v.color),
  'position' => art.position(v.kind),
  'sock' => art.sock(v.color, v.kind),
  'cup' => art.cup(v.number),
  'container' => art.container(v.kind),
  'tower' => art.tower(v.colors),
  _ => '',
};

class Wiggle extends StatefulWidget {
  final int serial;
  final bool active;
  final Widget child;
  const Wiggle({super.key, required this.serial, required this.active, required this.child});
  @override
  State<Wiggle> createState() => _WiggleState();
}

class _WiggleState extends State<Wiggle> with SingleTickerProviderStateMixin {
  late final c = AnimationController(vsync: this, duration: const Duration(milliseconds: 400));
  @override
  void didUpdateWidget(Wiggle old) {
    super.didUpdateWidget(old);
    if (widget.active && widget.serial != old.serial) c.forward(from: 0);
  }

  @override
  void dispose() {
    c.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => AnimatedBuilder(
    animation: c,
    builder: (_, child) => Transform.translate(offset: Offset(math.sin(c.value * math.pi * 5) * 6 * (1 - c.value), 0), child: child),
    child: widget.child,
  );
}

class GameScreen extends StatelessWidget {
  final JardimState s;
  const GameScreen({super.key, required this.s});

  @override
  Widget build(BuildContext context) {
    final q = s.q!;
    final g = cur.gameById(s.game);
    return SingleChildScrollView(
      child: Column(
        children: [
          SizedBox(
            height: 56,
            child: Row(
              children: [
                Btn('← Jardim', onTap: s.home),
                const SizedBox(width: 16),
                const Logo(),
                const Spacer(),
                Btn(s.voice ? '♪ Som ligado' : '♪ Som desligado', onTap: s.toggleVoice),
              ],
            ),
          ),
          const SizedBox(height: 12),
          AnimatedContainer(
            duration: const Duration(milliseconds: 250),
            padding: const EdgeInsets.all(20),
            decoration: BoxDecoration(color: hex(q.presentation.surface), borderRadius: BorderRadius.circular(q.presentation.radius + 8.0)),
            child: Column(
              children: [
                Row(
                  children: [
                    Text(g.name, style: sans(15, muted)),
                    const Spacer(),
                    Text('Descoberta ${s.round + 1} de ${s.length}', style: sans(15, muted)),
                  ],
                ),
                const SizedBox(height: 8),
                Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    for (var i = 0; i < s.length; i++)
                      Container(
                        width: i == s.round ? 14 : 12,
                        height: i == s.round ? 14 : 12,
                        margin: const EdgeInsets.symmetric(horizontal: 5),
                        decoration: BoxDecoration(
                          shape: BoxShape.circle,
                          color: i < s.round
                              ? green
                              : i == s.round
                              ? gold
                              : null,
                          border: i > s.round ? Border.all(color: const Color(0xFFC9D3BF), width: 1.5) : null,
                        ),
                      ),
                  ],
                ),
                const SizedBox(height: 10),
                Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Flexible(
                      child: Text(q.prompt, style: serif(30), textAlign: TextAlign.center),
                    ),
                    const SizedBox(width: 12),
                    GestureDetector(
                      onTap: s.listen,
                      child: Container(
                        width: 52,
                        height: 52,
                        decoration: BoxDecoration(
                          color: Colors.white,
                          shape: BoxShape.circle,
                          border: Border.all(color: line),
                        ),
                        alignment: Alignment.center,
                        child: const Icon(Icons.volume_up_rounded, color: green),
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 6),
                Text(q.memory ? 'Toque nas cartas para descobrir os desenhos.' : 'Toque na resposta que você escolheu.', style: sans(15, muted)),
                const SizedBox(height: 12),
                if (!q.memory && q.scene != null) SceneView(s: s, scene: q.scene!),
                if (!q.memory) const SizedBox(height: 18),
                if (!q.memory)
                  Wrap(alignment: WrapAlignment.center, spacing: 18, runSpacing: 18, children: [for (var i = 0; i < q.options.length; i++) _choice(i, q)]),
                if (q.memory)
                  SizedBox(
                    width: (q.cards.length <= 4 ? 2 : 4) * 156.0,
                    child: Wrap(
                      alignment: WrapAlignment.center,
                      spacing: 16,
                      runSpacing: 16,
                      children: [for (var i = 0; i < q.cards.length; i++) _memoryCard(i, q)],
                    ),
                  ),
                const SizedBox(height: 18),
                const Divider(color: line),
                SizedBox(
                  height: 64,
                  child: Row(
                    children: [
                      Expanded(
                        child: Text(switch (s.feedback) {
                          'success' => '★ Você descobriu! Muito bem.',
                          'hint' => q.hint,
                          'retry' => 'Vamos tentar de novo? ${q.hint}',
                          _ => 'Vamos descobrir juntos?',
                        }, style: s.feedback == 'success' ? serif(18, success) : sans(17)),
                      ),
                      s.correct
                          ? Btn(s.round + 1 == s.length ? 'Ver meu jardim →' : 'Próxima descoberta →', kind: 'primary', onTap: s.next)
                          : Btn(s.feedback == 'idle' ? 'Uma dica' : 'Ouvir dica', onTap: s.hint),
                    ],
                  ),
                ),
              ],
            ),
          ),
          const SizedBox(height: 20),
        ],
      ),
    );
  }

  Widget _choice(int i, cur.Question q) {
    final right = s.correct && i == q.answer;
    final retry = s.wiggle == i;
    final r = BorderRadius.circular(math.min(q.presentation.radius, 40).toDouble());
    return Wiggle(
      serial: s.wiggleSerial,
      active: retry,
      child: GestureDetector(
        onTap: s.correct ? null : () => s.answer(i),
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 150),
          width: 150,
          height: 140,
          padding: const EdgeInsets.all(13),
          decoration: BoxDecoration(
            color: right
                ? const Color(0xFFE1EED8)
                : retry
                ? const Color(0xFFF8EBDB)
                : Colors.white,
            borderRadius: r,
            border: Border.all(color: right ? const Color(0xFF78A36D) : const Color(0xFFE5E7D9), width: 2),
            boxShadow: [BoxShadow(color: right ? const Color(0xFF78A36D) : const Color(0xFFE3E6D6), offset: const Offset(0, 6))],
          ),
          child: VisualView(v: q.options[i].visual, q: q),
        ),
      ),
    );
  }

  Widget _memoryCard(int i, cur.Question q) {
    final isMatched = s.matched.contains(i);
    final open = isMatched || s.revealed.contains(i);
    return GestureDetector(
      onTap: isMatched ? null : () => s.flip(i),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 150),
        width: 140,
        height: 140,
        padding: const EdgeInsets.all(22),
        decoration: BoxDecoration(
          color: isMatched
              ? const Color(0xFFEFF5E9)
              : open
              ? const Color(0xFFFFFDF7)
              : const Color(0xFFE2EAD6),
          borderRadius: BorderRadius.circular(20),
          border: Border.all(
            color: isMatched
                ? const Color(0xFF95B681)
                : open
                ? const Color(0xFFD8CBA5)
                : const Color(0xFFC5D6B7),
            width: 2,
          ),
          boxShadow: [BoxShadow(color: isMatched ? const Color(0xFF95B681) : const Color(0xFFC3D2B4), offset: const Offset(0, 5))],
        ),
        child: Art(art.icon(open ? q.cards[i].kind : 'leaf'), 96, 96),
      ),
    );
  }
}

class VisualView extends StatelessWidget {
  final cur.Visual v;
  final cur.Question q;
  const VisualView({super.key, required this.v, required this.q});

  @override
  Widget build(BuildContext context) {
    switch (v.type) {
      case 'number':
        return Center(child: Text('${v.number.toInt()}', style: serif(52)));
      case 'text':
        return Center(child: Text(v.text, style: serif(v.text.length > 3 ? 28 : 46)));
      case 'swatch':
        return Center(
          child: Container(
            width: 80,
            height: 80,
            decoration: BoxDecoration(color: hex(v.color), borderRadius: BorderRadius.circular(18)),
          ),
        );
      case 'quantity':
        final n = v.number.toInt();
        final cols = math.max(
          1,
          math.min(
            n,
            n <= 4
                ? 2
                : n <= 9
                ? 3
                : 4,
          ),
        );
        final rows = (n / cols).ceil();
        final cell = math.min(124 / cols, 114 / rows);
        final apple = art.icon('apple', q.presentation.itemColor);
        return Center(
          child: SizedBox(
            width: cell * cols,
            child: Wrap(
              children: [for (var i = 0; i < n; i++) Transform.rotate(angle: q.presentation.rotation[i % 24] * math.pi / 180, child: Art(apple, cell, cell))],
            ),
          ),
        );
      case 'sized':
        final sz = 114 * v.number;
        return Center(child: Art(visualSvg(v), sz, sz));
      default:
        return Center(child: Art(visualSvg(v), 114, 114));
    }
  }
}

class SceneView extends StatelessWidget {
  final JardimState s;
  final cur.Scene scene;
  const SceneView({super.key, required this.s, required this.scene});

  Widget _missing(double w, double h) => Container(
    width: w,
    height: h,
    alignment: Alignment.center,
    decoration: BoxDecoration(
      color: const Color(0xFFFFFDF8),
      borderRadius: BorderRadius.circular(14),
      border: Border.all(color: gold, width: 2.5),
    ),
    child: Text('?', style: serif(34, const Color(0xFFC8963C))),
  );

  Widget _apple(int idx, double size) {
    final on = s.counted.contains(idx);
    final q = s.q!;
    return GestureDetector(
      onTap: () => s.countItem(idx),
      child: Container(
        width: size,
        height: size,
        padding: const EdgeInsets.all(4),
        decoration: on
            ? BoxDecoration(
                color: const Color(0xFFE0EDDC),
                shape: BoxShape.circle,
                border: Border.all(color: const Color(0xFF78A36D), width: 2),
              )
            : null,
        child: Transform.rotate(
          angle: q.presentation.rotation[idx % 24] * math.pi / 180,
          child: Art(art.icon('apple', q.presentation.itemColor), size - 8, size - 8),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final sc = scene;
    final q = s.q!;
    switch (sc.type) {
      case 'color':
        return Container(
          width: 120,
          height: 120,
          decoration: BoxDecoration(color: hex(sc.color), shape: BoxShape.circle),
        );
      case 'shape':
        return Art(art.icon(sc.shape, sc.color), 140, 140);
      case 'sock':
        return Art(art.sock(sc.color, sc.pattern), 150, 150);
      case 'tower':
        return Art(art.tower(sc.items), 150, 150);
      case 'clock':
        return Art(art.clock(sc.count), 170, 170);
      case 'object-model':
        return Container(
          width: 150,
          height: 150,
          padding: const EdgeInsets.all(14),
          decoration: BoxDecoration(color: Colors.white, borderRadius: BorderRadius.circular(24)),
          child: Art(art.icon(sc.icon), 122, 122),
        );
      case 'look':
        return Text('Observe com atenção. Você consegue!', style: sans(16, muted));
      case 'items':
        return Wrap(
          alignment: WrapAlignment.center,
          spacing: q.presentation.itemGap.toDouble(),
          runSpacing: 6,
          children: [for (var i = 0; i < sc.count; i++) _apple(i, 64)],
        );
      case 'sum':
        return Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            SizedBox(
              width: math.min(sc.a, 5) * 52.0,
              child: Wrap(spacing: 6, runSpacing: 6, children: [for (var i = 0; i < sc.a; i++) _apple(i, 46)]),
            ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              child: Text('+', style: serif(40)),
            ),
            SizedBox(
              width: math.min(sc.b, 5) * 52.0,
              child: Wrap(spacing: 6, runSpacing: 6, children: [for (var i = 0; i < sc.b; i++) _apple(sc.a + i, 46)]),
            ),
          ],
        );
      case 'sequence':
        return Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            for (final it in sc.items) Padding(padding: const EdgeInsets.all(3), child: Art(art.icon(it, '#8b7bbd'), 84, 84)),
            _missing(72, 72),
          ],
        );
      case 'numbers':
        return SizedBox(
          height: 120,
          child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.end,
            children: [
              for (var i = 0; i < sc.numbers.length; i++)
                Padding(
                  padding: EdgeInsets.only(left: 6, right: 6, bottom: i * 12.0),
                  child: sc.numbers[i] == null
                      ? _missing(80, 70)
                      : Container(
                          width: 80,
                          height: 70,
                          alignment: Alignment.center,
                          decoration: BoxDecoration(
                            color: Colors.white,
                            borderRadius: BorderRadius.circular(14),
                            border: Border.all(color: line, width: 2),
                          ),
                          child: Text('${sc.numbers[i]}', style: serif(34)),
                        ),
                ),
            ],
          ),
        );
      case 'friends':
        return Row(mainAxisAlignment: MainAxisAlignment.center, children: [for (var i = 0; i < sc.count; i++) Art(art.icon('bunny'), 80, 78)]);
      case 'color-sequence':
        return Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            for (final c in sc.items)
              Container(
                width: 68,
                height: 68,
                margin: const EdgeInsets.all(8),
                decoration: BoxDecoration(color: hex(c), shape: BoxShape.circle),
              ),
            _missing(68, 68),
          ],
        );
      case 'letter':
        return Container(
          width: 260,
          height: 120,
          alignment: Alignment.center,
          decoration: BoxDecoration(color: Colors.white, borderRadius: BorderRadius.circular(24)),
          child: Text(sc.text, style: serif(64)),
        );
      case 'word-picture':
        return Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Art(art.icon(sc.icon), 130, 130),
            if (sc.word.isNotEmpty) ...[const SizedBox(width: 20), Text(sc.word, style: serif(46))],
          ],
        );
      case 'subtract':
        return Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            for (var i = 0; i < sc.a; i++)
              Padding(
                padding: const EdgeInsets.all(3),
                child: Stack(
                  alignment: Alignment.center,
                  children: [
                    Opacity(opacity: i >= sc.a - sc.b ? 0.4 : 1, child: Art(art.icon('apple'), 52, 52)),
                    if (i >= sc.a - sc.b)
                      Transform.rotate(
                        angle: math.pi / 4,
                        child: Container(width: 58, height: 4, color: const Color(0xFFC95A46)),
                      ),
                  ],
                ),
              ),
          ],
        );
      case 'equation':
        return Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            for (final t in ['${sc.a}', '+', null, '=', '${sc.count}'])
              Padding(
                padding: const EdgeInsets.all(4),
                child: t == null
                    ? _missing(72, 72)
                    : SizedBox(
                        width: 72,
                        height: 72,
                        child: Center(child: Text(t, style: serif(40))),
                      ),
              ),
          ],
        );
      case 'tenframe':
        return Container(
          padding: const EdgeInsets.all(6),
          decoration: BoxDecoration(
            color: Colors.white,
            borderRadius: BorderRadius.circular(12),
            border: Border.all(color: const Color(0xFF92B4AD), width: 2),
          ),
          child: SizedBox(
            width: 280,
            child: Wrap(
              children: [
                for (var i = 0; i < 10; i++)
                  Container(
                    width: 56,
                    height: 56,
                    decoration: BoxDecoration(border: Border.all(color: line)),
                    alignment: Alignment.center,
                    child: i < sc.count
                        ? Container(
                            width: 38,
                            height: 38,
                            decoration: const BoxDecoration(color: Color(0xFFE87961), shape: BoxShape.circle),
                          )
                        : null,
                  ),
              ],
            ),
          ),
        );
      case 'tens':
        return Row(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.end,
          children: [
            for (var t = 0; t < sc.a; t++)
              Padding(
                padding: const EdgeInsets.only(right: 14),
                child: Column(
                  children: [
                    for (var k = 0; k < 10; k++)
                      Container(
                        width: 13,
                        height: 13,
                        decoration: BoxDecoration(
                          color: const Color(0xFF5799BE),
                          borderRadius: BorderRadius.circular(2),
                          border: Border.all(color: Colors.white, width: 0.5),
                        ),
                      ),
                  ],
                ),
              ),
            const SizedBox(width: 16),
            for (var u = 0; u < sc.b; u++)
              Container(
                width: 13,
                height: 13,
                margin: const EdgeInsets.only(right: 4),
                decoration: BoxDecoration(color: const Color(0xFFE0B849), borderRadius: BorderRadius.circular(2)),
              ),
          ],
        );
      case 'story':
        return ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 720),
          child: Row(
            children: [
              Art(art.icon(sc.icon), 96, 96),
              const SizedBox(width: 12),
              Expanded(child: Text(sc.text, style: sans(19))),
            ],
          ),
        );
      default:
        return const SizedBox.shrink();
    }
  }
}

// ---------------------------------------------------------------- final e diálogos

class FinishScreen extends StatelessWidget {
  final JardimState s;
  const FinishScreen({super.key, required this.s});
  @override
  Widget build(BuildContext context) {
    final d = s.dailyOf(s.game);
    return SingleChildScrollView(
      child: Column(
        children: [
          Header(s: s),
          const SizedBox(height: 30),
          TweenAnimationBuilder<double>(
            tween: Tween(begin: 0.6, end: 1),
            duration: const Duration(milliseconds: 450),
            curve: Curves.easeOutBack,
            builder: (_, v, child) => Transform.scale(scale: v, child: child),
            child: Art(art.icon('flower'), 130, 130),
          ),
          const Text('A CURIOSIDADE FLORESCEU!', style: eyebrowStyle),
          Text(s.length == 1 ? 'Uma nova descoberta!' : '${s.length} novas descobertas!', style: serif(40)),
          Text(d.completed ? '✓ Atividade feita hoje!' : '${d.solved} de 5 descobertas hoje', style: serif(18, success)),
          const SizedBox(height: 8),
          Text('Você observou, tentou e aprendeu.', style: sans(17, muted)),
          Text('Que tal alongar o corpo e olhar o mundo lá fora?', style: sans(17, muted)),
          Text('★ ' * s.length, style: sans(22, gold)),
          const SizedBox(height: 14),
          Btn('Voltar para o jardim', kind: 'primary', onTap: s.home),
          const SizedBox(height: 10),
          Text('Cada tentativa faz parte de aprender.', style: sans(14, muted)),
        ],
      ),
    );
  }
}

class ModalLayer extends StatefulWidget {
  final JardimState s;
  const ModalLayer({super.key, required this.s});
  @override
  State<ModalLayer> createState() => _ModalLayerState();
}

class _ModalLayerState extends State<ModalLayer> {
  final rnd = math.Random();
  late int a = 6 + rnd.nextInt(4), b = 6 + rnd.nextInt(4);
  String input = '';
  bool error = false;

  @override
  Widget build(BuildContext context) {
    final s = widget.s;
    final h = MediaQuery.of(context).size.height;
    return Positioned.fill(
      child: Container(
        color: const Color(0xA8233B36), // fundo escurecido simples, sem blur
        alignment: Alignment.center,
        child: Container(
          width: 560,
          constraints: BoxConstraints(maxHeight: h * 0.9),
          padding: const EdgeInsets.all(30),
          decoration: BoxDecoration(color: paper, borderRadius: BorderRadius.circular(27)),
          child: SingleChildScrollView(
            child: switch (s.modal) {
              'gate' => _gate(s),
              'reset' => _reset(s),
              _ => _parents(s),
            },
          ),
        ),
      ),
    );
  }

  Widget _gate(JardimState s) => Column(
    crossAxisAlignment: CrossAxisAlignment.start,
    children: [
      const Text('UM MOMENTO PARA OS ADULTOS', style: eyebrowStyle),
      Text('Vamos cuidar do jardim?', style: serif(26)),
      Text('Para abrir as configurações, resolva:', style: sans(16, muted)),
      const SizedBox(height: 8),
      Row(
        children: [
          Text('Quanto é $a × $b?', style: serif(22)),
          const SizedBox(width: 12),
          Container(
            width: 110,
            height: 50,
            alignment: Alignment.center,
            decoration: BoxDecoration(
              color: Colors.white,
              borderRadius: BorderRadius.circular(12),
              border: Border.all(color: line, width: 2),
            ),
            child: Text(input, style: serif(26)),
          ),
        ],
      ),
      if (error) Text('Tente mais uma vez.', style: sans(15, const Color(0xFF9C513C))),
      const SizedBox(height: 10),
      SizedBox(
        width: 3 * 80.0,
        child: Wrap(
          spacing: 8,
          runSpacing: 8,
          children: [
            for (final k in ['1', '2', '3', '4', '5', '6', '7', '8', '9', '⌫', '0'])
              GestureDetector(
                onTap: () => setState(() {
                  error = false;
                  if (k == '⌫') {
                    if (input.isNotEmpty) input = input.substring(0, input.length - 1);
                  } else if (input.length < 3) {
                    input += k;
                  }
                }),
                child: Container(
                  width: 72,
                  height: 52,
                  alignment: Alignment.center,
                  decoration: BoxDecoration(color: quiet, borderRadius: BorderRadius.circular(12)),
                  child: Text(k, style: serif(22)),
                ),
              ),
          ],
        ),
      ),
      const SizedBox(height: 14),
      Row(
        children: [
          Btn('Voltar', onTap: () => s.setModal('')),
          const SizedBox(width: 10),
          Btn(
            'Entrar',
            kind: 'primary',
            onTap: () {
              if (int.tryParse(input) == a * b) {
                s.setModal('parents');
              } else {
                setState(() => error = true);
              }
            },
          ),
        ],
      ),
    ],
  );

  Widget _parents(JardimState s) => Column(
    crossAxisAlignment: CrossAxisAlignment.start,
    children: [
      const Text('ÁREA DOS ADULTOS', style: eyebrowStyle),
      Text('Aprender no próprio ritmo', style: serif(26)),
      Text('As faixas são sugestões. Escolha a que combina com a criança e acompanhe as atividades, especialmente aos 2–3 anos.', style: sans(15, muted)),
      const SizedBox(height: 10),
      Text('Faixa etária', style: serif(18)),
      Wrap(
        spacing: 4,
        children: [for (final g in cur.groups) Btn(g.label, kind: 'pill', selected: g.id == s.group, onTap: () => s.setGroup(g.id))],
      ),
      const SizedBox(height: 10),
      Text('Narração em português', style: serif(18)),
      Btn(s.voice ? 'Ligada — toque para desligar' : 'Desligada — toque para ligar', onTap: s.toggleVoice),
      Text('Usa a voz do Android. O primeiro uso pode exigir uma voz pt-BR já instalada. As atividades funcionam sem áudio.', style: sans(13, muted)),
      const SizedBox(height: 10),
      Text('Descobertas por atividade · ${cur.groups[s.level].label}', style: serif(18)),
      for (final g in cur.games)
        Row(
          children: [
            Text(g.name, style: sans(14)),
            const Spacer(),
            Text('${s.dailyOf(g.id).solved}/5 hoje · ${s.statOf(g.id).solved} no total', style: sans(14, muted)),
          ],
        ),
      const SizedBox(height: 8),
      Text(
        'Os dados ficam somente neste dispositivo. Sem conta, anúncios, compras ou acesso à internet. A pergunta de entrada é uma barreira simples para crianças pequenas.',
        style: sans(13, muted),
      ),
      const SizedBox(height: 14),
      Row(
        children: [
          Btn('Apagar progresso', kind: 'danger', onTap: () => s.setModal('reset')),
          const SizedBox(width: 10),
          Btn('Pronto', kind: 'primary', onTap: s.home),
        ],
      ),
    ],
  );

  Widget _reset(JardimState s) => Column(
    crossAxisAlignment: CrossAxisAlignment.start,
    children: [
      Text('Apagar todas as descobertas?', style: serif(26)),
      Text('Isso apaga o progresso das três faixas etárias neste dispositivo.', style: sans(16, muted)),
      const SizedBox(height: 16),
      Row(
        children: [
          Btn('Cancelar', onTap: () => s.setModal('parents')),
          const SizedBox(width: 10),
          Btn(
            'Sim, apagar',
            kind: 'danger',
            onTap: () {
              s.reset();
              s.setModal('parents');
            },
          ),
        ],
      ),
    ],
  );
}
