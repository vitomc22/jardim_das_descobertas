/* Jardim das Descobertas — versão React Native.
 * Lógica e desenhos: os MESMOS arquivos JS do app original (src/legacy),
 * sem alteração (só um export no art.js). UI em componentes nativos do RN. */
import React, { memo, useEffect, useRef, useState, useSyncExternalStore } from 'react';
import {
  Animated, AppState, BackHandler, NativeModules, Pressable, ScrollView, StatusBar, StyleSheet, Text, TextProps, View,
  useWindowDimensions,
} from 'react-native';
import { SafeAreaProvider, useSafeAreaInsets } from 'react-native-safe-area-context';
import { SvgXml } from 'react-native-svg';

/* eslint-disable @typescript-eslint/no-var-requires */
const Curriculum = require('./src/legacy/curriculum.js');
const DailyProgress = require('./src/legacy/progress.js');
const Variation = require('./src/legacy/variation.js');
const Art = require('./src/legacy/art.js');
/* eslint-enable @typescript-eslint/no-var-requires */

const { GROUPS, GAMES, CATEGORIES, generate } = Curriculum;
const Native = NativeModules.JardimNative as {
  speak(t: string): void; stop(): void; load(): Promise<string | null>; save(json: string): void;
};

// ---------------------------------------------------------------- paleta
const C = {
  ink: '#294b43', muted: '#6d7d70', paper: '#f8f6ee', green: '#347b65', greenShadow: '#26614f', line: '#e2e6d9',
  quiet: '#eeeee4', quietBorder: '#dedfd2', eyebrow: '#7f8c73', em: '#56856b', success: '#387657', gold: '#e9bd68',
};
const cardBg = (c: string) =>
  (({ blue: '#deedf2', yellow: '#faedc8', green: '#e0eddc', purple: '#e8e2f1', peach: '#f6e7d5' }) as Record<string, string>)[c] ?? '#f7e1db';

// Mesmo tamanho de texto das outras versões (sem a escala de fonte do sistema).
const T = (p: TextProps) => <Text allowFontScaling={false} {...p} />;
const serif = (size: number, color = C.ink) => ({ fontFamily: 'serif', fontWeight: 'bold' as const, fontSize: size, color });
const sans = (size: number, color = C.ink) => ({ fontSize: size, color });

// ---------------------------------------------------------------- estado (mesmas regras do app.js)
const KEY_RE = /^(little|middle|big):[a-z]+$/;
type Stat = { solved: number; tries: number; sets: number; rounds: number };
const blank = (): Stat => ({ solved: 0, tries: 0, sets: 0, rounds: 0 });

const store = {
  v: 0,
  screen: 'home', group: 'little', category: 'all', modal: '', voice: true,
  stats: {} as Record<string, Stat>,
  daily: DailyProgress.normalise(null, new Date()),
  entry: Variation.entrySeed(),
  // sessão
  game: '', round: 0, length: 0, q: null as any, correct: false, feedback: 'idle', busy: false,
  revealed: [] as number[], matched: [] as number[], counted: [] as number[], wiggle: -1, wiggleSerial: 0, flipToken: 0,
};
type Store = typeof store;
const listeners = new Set<() => void>();
const emit = () => { store.v++; listeners.forEach(l => l()); };
const useStore = () => {
  useSyncExternalStore(cb => { listeners.add(cb); return () => listeners.delete(cb); }, () => store.v);
  return store;
};

const key = (g: string) => `${store.group}:${g}`;
const stat = (g: string): Stat => store.stats[key(g)] ?? blank();
const dailyOf = (g: string) => DailyProgress.status(store.daily, key(g));
const total = () => Object.values(store.stats).reduce((a, s) => a + (s.solved || 0), 0);
const level = () => GROUPS.findIndex((g: any) => g.id === store.group);
const catalog = () =>
  Curriculum.shuffle(
    GAMES.filter((g: any) => g.min <= level()),
    Curriculum.random(Variation.seedFor(store.daily.date, store.group, 'catalog', 0, store.entry)),
  );

let saveTimer: any;
function save() {
  clearTimeout(saveTimer);
  saveTimer = setTimeout(() => Native.save(JSON.stringify({
    version: 1, group: store.group, voice: store.voice, stats: store.stats, daily: store.daily,
  })), 50);
}
async function load() {
  try {
    const raw = await Native.load();
    const s = raw ? JSON.parse(raw) : null;
    if (s && s.version === 1 && GROUPS.some((g: any) => g.id === s.group)) {
      store.group = s.group;
      store.voice = s.voice !== false;
      for (const [k, v] of Object.entries(s.stats || {})) if (KEY_RE.test(k)) store.stats[k] = { ...blank(), ...(v as Stat) };
      store.daily = DailyProgress.normalise(s.daily, new Date());
    }
  } catch {}
  emit();
}
function ensureDay() {
  if (store.daily.date === DailyProgress.localDay()) return false;
  store.daily = DailyProgress.normalise(null, new Date());
  save();
  return true;
}
const speak = (t: string) => { if (store.voice) Native.speak(t); };

const A = {
  home() { ensureDay(); Native.stop(); store.screen = 'home'; store.modal = ''; emit(); },
  setGroup(g: string) { store.group = g; store.category = 'all'; save(); emit(); },
  setCategory(c: string) { store.category = c; emit(); },
  setModal(m: string) { if (m === 'gate') Native.stop(); store.modal = m; emit(); },
  start(g: string) {
    ensureDay();
    store.game = g;
    const d = dailyOf(g);
    store.length = d.completed ? DailyProgress.GOAL : DailyProgress.GOAL - d.solved;
    store.round = 0;
    store.screen = 'game';
    A.nextQuestion();
  },
  nextQuestion() {
    ensureDay();
    const s = (store.stats[key(store.game)] ??= blank());
    s.rounds++;
    save();
    store.q = generate(store.game, store.group, Variation.seedFor(store.daily.date, store.group, store.game, s.rounds, store.entry));
    Object.assign(store, { correct: false, feedback: 'idle', revealed: [], matched: [], counted: [], wiggle: -1, busy: false });
    store.flipToken++;
    emit();
    speak((store.q.story ? store.q.story + ' ' : '') + store.q.prompt);
  },
  recordTry() { (store.stats[key(store.game)] ??= blank()).tries++; save(); },
  correct() {
    if (store.correct) return;
    store.correct = true;
    store.feedback = 'success';
    ensureDay();
    DailyProgress.record(store.daily, key(store.game));
    (store.stats[key(store.game)] ??= blank()).solved++;
    save();
    emit();
    speak('Você descobriu! Muito bem.');
  },
  answer(i: number) {
    if (store.correct) return;
    A.recordTry();
    if (i === store.q.answer) return A.correct();
    store.wiggle = i;
    const serial = ++store.wiggleSerial;
    store.feedback = 'retry';
    emit();
    speak('Vamos tentar de novo? ' + store.q.hint);
    setTimeout(() => { if (serial === store.wiggleSerial) { store.wiggle = -1; emit(); } }, 500);
  },
  flip(i: number) {
    if (store.busy || store.correct || store.revealed.includes(i) || store.matched.includes(i)) return;
    store.revealed = [...store.revealed, i];
    if (store.revealed.length < 2) return emit();
    A.recordTry();
    const [a, d] = store.revealed;
    if (store.q.cards[a].pair === store.q.cards[d].pair) {
      store.matched = [...store.matched, a, d];
      store.revealed = [];
      if (store.matched.length === store.q.cards.length) return A.correct();
      emit();
      speak('Um par!');
    } else {
      store.busy = true;
      emit();
      const token = ++store.flipToken;
      setTimeout(() => { if (token !== store.flipToken) return; store.revealed = []; store.busy = false; emit(); }, 1200);
    }
  },
  next() {
    if (!store.correct) return;
    store.round++;
    if (store.round === store.length) {
      ensureDay();
      Native.stop();
      (store.stats[key(store.game)] ??= blank()).sets++;
      save();
      store.screen = 'finish';
      emit();
      speak((store.length === 1 ? 'Uma nova descoberta!' : `${store.length} novas descobertas!`) + ' Que tal fazer uma pausa?');
    } else A.nextQuestion();
  },
  hint() {
    if (!store.correct) store.feedback = 'hint';
    emit();
    speak((store.q.story ? store.q.story + ' ' : '') + store.q.hint);
  },
  listen() { speak((store.q.story ? store.q.story + ' ' : '') + store.q.prompt); },
  countItem(i: number) {
    store.counted = store.counted.includes(i) ? store.counted.filter(x => x !== i) : [...store.counted, i];
    emit();
  },
  toggleVoice() {
    store.voice = !store.voice;
    save();
    if (!store.voice) Native.stop(); else if (store.screen === 'game') speak(store.q.prompt);
    emit();
  },
  reset() { store.stats = {}; store.daily = DailyProgress.normalise(null, new Date()); save(); emit(); },
  resume() { store.entry = Variation.entrySeed(); ensureDay(); emit(); },
};

// ---------------------------------------------------------------- componentes

// O art.js original posiciona a flor da caixa com um <svg x y width height>
// aninhado, que o react-native-svg não posiciona: vira um <g transform>.
const NESTED = /<svg x="([\d.]+)" y="([\d.]+)" width="([\d.]+)" height="([\d.]+)" viewBox="0 0 120 120">([\s\S]*?)<\/svg>/g;
const fixNested = (xml: string) =>
  xml.replace(NESTED, (_m, x, y, w, _h, body) => `<g transform="translate(${x} ${y}) scale(${Number(w) / 120})">${body}</g>`);
const Svg = memo(({ xml, w, h }: { xml: string; w: number; h: number }) => <SvgXml xml={fixNested(xml)} width={w} height={h} />);

function Btn({ text, kind = 'quiet', selected = false, onPress }: { text: string; kind?: string; selected?: boolean; onPress: () => void }) {
  const primary = kind === 'primary';
  const fill = primary ? C.green : kind === 'danger' ? '#f2e2d8' : kind === 'pill' ? (selected ? C.green : 'transparent')
    : kind === 'tab' ? (selected ? '#e0eada' : 'transparent') : C.quiet;
  const fg = primary ? '#fff' : kind === 'danger' ? '#9c513c' : kind === 'pill' ? (selected ? '#fff' : C.muted)
    : kind === 'tab' ? (selected ? '#346950' : C.muted) : C.ink;
  return (
    <Pressable onPress={onPress}>
      {({ pressed }) => (
        <View>
          {primary && <View style={[StyleSheet.absoluteFill, { top: 4, bottom: -4, backgroundColor: C.greenShadow, borderRadius: 16 }]} />}
          <View style={{
            height: primary ? 56 : kind === 'tab' ? 42 : 46, paddingHorizontal: primary ? 24 : 17, justifyContent: 'center',
            backgroundColor: !primary && kind === 'quiet' && pressed ? C.line : fill, borderRadius: primary ? 16 : kind === 'danger' ? 15 : 24,
            borderWidth: kind === 'quiet' || kind === 'tab' ? 1 : 0,
            borderColor: kind === 'tab' ? (selected ? '#9dba99' : C.line) : C.quietBorder,
            transform: [{ translateY: pressed && primary ? 3 : 0 }],
          }}>
            <T style={primary || kind === 'danger' ? serif(primary ? 18 : 16, fg) : sans(15, fg)}>{text}</T>
          </View>
        </View>
      )}
    </Pressable>
  );
}

const Logo = () => (
  <View style={{ flexDirection: 'row', alignItems: 'center', gap: 8 }}>
    <Svg xml={Art.icon('leaf')} w={34} h={34} />
    <View>
      <T style={serif(22, C.green)}>jardim</T>
      <T style={sans(11, C.muted)}>das descobertas</T>
    </View>
  </View>
);

const Eyebrow = ({ children }: { children: string }) => (
  <T style={{ fontSize: 12, letterSpacing: 2, fontWeight: 'bold', color: C.eyebrow }}>{children}</T>
);

function Header({ s }: { s: Store }) {
  return (
    <View style={{ height: 56, flexDirection: 'row', alignItems: 'center' }}>
      <Logo />
      <View style={{ flex: 1 }} />
      <T style={[sans(15, C.eyebrow), { marginRight: 12 }]}>★ {total()} descobertas</T>
      <Btn text="Área dos adultos" onPress={() => A.setModal('gate')} />
    </View>
  );
}

// ---------------------------------------------------------------- tela inicial

const GameCard = memo(({ g, w, solved, completed }: { g: any; w: number; solved: number; completed: boolean }) => {
  const wide = !['flower', 'letters', 'book', 'clock', 'socks', 'cup', 'tower', 'container', 'colorpath'].includes(g.icon);
  const tileXml = Art.tile(g.icon);
  return (
    <Pressable onPress={() => A.start(g.id)} style={{ width: w, height: 294 }}>
      <View style={{ position: 'absolute', left: 0, right: 0, top: 4, height: 290, backgroundColor: '#e8e9de', borderRadius: 22 }} />
      <View style={{ height: 290, backgroundColor: '#fff', borderRadius: 22, borderWidth: 1, borderColor: '#e6e6d9', overflow: 'hidden' }}>
        <View style={{ height: 140, backgroundColor: cardBg(g.color), alignItems: 'center', justifyContent: 'center' }}>
          {wide ? <WideTile g={g} /> : <Svg xml={tileXml} w={110} h={110} />}
          <T style={[sans(18), { position: 'absolute', right: 16, top: 12, transform: [{ rotate: '-45deg' }] }]}>→</T>
        </View>
        <View style={{ padding: 16, paddingTop: 12 }}>
          <T style={serif(19)} numberOfLines={1} adjustsFontSizeToFit>{g.name}</T>
          <T style={sans(14, C.muted)}>{g.skill}</T>
          <View style={{ marginTop: 8, padding: 6, borderRadius: 9, backgroundColor: completed ? '#f1f7e9' : undefined }}>
            <T style={serif(14, completed ? C.success : C.ink)}>{completed ? '✓ Feita hoje' : solved ? 'Em andamento hoje' : 'Para descobrir hoje'}</T>
            <T style={sans(13, C.muted)}>{solved} de 5 descobertas</T>
            <View style={{ flexDirection: 'row', gap: 6, marginTop: 4 }}>
              {[0, 1, 2, 3, 4].map(i => (
                <View key={i} style={{
                  width: 10, height: 10, borderRadius: 5, backgroundColor: i < solved ? C.green : undefined,
                  borderWidth: i < solved ? 0 : 1.5, borderColor: '#c9d3bf',
                }} />
              ))}
            </View>
          </View>
        </View>
      </View>
    </Pressable>
  );
});

// art.tile() do original devolve dois <svg> dentro de um <div>: desenhamos os dois.
const WideTile = ({ g }: { g: any }) => {
  const map: Record<string, [string, string]> = {
    shapes: ['circle', 'triangle'], count: ['apple', 'apple'], memory: ['bird', 'bird'], pattern: ['circle', 'star'],
    odd: ['circle', 'square'], sum: ['apple', 'star'], order: ['triangle', 'star'],
  };
  const [a, b] = map[g.icon] ?? ['leaf', 'sun'];
  const ca = g.icon === 'pattern' ? '#8c7db3' : '#edab69', cb = g.icon === 'pattern' ? '#8c7db3' : '#62a58e';
  return (
    <View style={{ flexDirection: 'row' }}>
      <Svg xml={Art.icon(a, ca)} w={90} h={90} />
      <Svg xml={Art.icon(b, cb)} w={90} h={90} />
    </View>
  );
};

function Home({ s }: { s: Store }) {
  const { width } = useWindowDimensions();
  const insets = useSafeAreaInsets();
  const scroll = useRef<any>(null);
  const [actY, setActY] = useState(0);
  const w = width - 48 - insets.left - insets.right;
  const all = catalog();
  const cards = all.filter((g: any) => s.category === 'all' || g.category === s.category);
  const done = all.filter((g: any) => dailyOf(g.id).completed).length;
  const pct = all.length ? Math.round((100 * done) / all.length) : 0;
  const cols = Math.max(1, Math.floor((w + 16) / (250 + 16)));
  const cardW = (w - 16 * (cols - 1)) / cols;
  const gardenW = Math.min(460, w * 0.45);
  return (
    <ScrollView ref={scroll} showsVerticalScrollIndicator={false}>
      <Header s={s} />
      <View style={{ height: 18 }} />
      <View style={{ flexDirection: 'row', alignItems: 'center' }}>
        <View style={{ flex: 1, gap: 6 }}>
          <Eyebrow>BRINCAR • PENSAR • DESCOBRIR</Eyebrow>
          <View>
            <T style={serif(40)}>Pequenas descobertas.</T>
            <T style={serif(40, C.em)}>Grandes sorrisos.</T>
          </View>
          <T style={sans(17, C.muted)}>Um jardim de ideias para aprender brincando, um passo de cada vez.</T>
          <View style={{ height: 10 }} />
          <View style={{ alignSelf: 'flex-start' }}>
            <Btn text="Vamos explorar →" kind="primary" onPress={() => {
              speak('Escolha uma atividade para começar.');
              scroll.current?.scrollTo({ y: actY, animated: true });
            }} />
          </View>
        </View>
        <View style={{ width: 20 }} />
        <Svg xml={Art.garden()} w={gardenW} h={(gardenW * 300) / 520} />
      </View>
      <View style={{ height: 28 }} />
      <View onLayout={e => setActY(e.nativeEvent.layout.y)} style={{ flexDirection: 'row', flexWrap: 'wrap', alignItems: 'center', gap: 16 }}>
        <View>
          <Eyebrow>CADA IDADE, UMA DESCOBERTA</Eyebrow>
          <T style={serif(30)}>O que vamos brincar hoje?</T>
        </View>
        <View style={{ flexDirection: 'row', gap: 4 }}>
          {GROUPS.map((g: any) => <Btn key={g.id} text={g.label} kind="pill" selected={g.id === s.group} onPress={() => A.setGroup(g.id)} />)}
        </View>
      </View>
      <T style={[sans(15, C.muted), { marginTop: 6 }]}>{GROUPS[level()].name} • {all.length} atividades gratuitas</T>
      <View style={{
        marginTop: 14, paddingHorizontal: 22, paddingVertical: 20, borderRadius: 22, backgroundColor: '#eaf0e2', borderWidth: 1,
        borderColor: '#d4dfc7', flexDirection: 'row', alignItems: 'center',
      }}>
        <View style={{ flex: 1 }}>
          <Eyebrow>SEU JARDIM DE HOJE</Eyebrow>
          <T style={serif(21)}>{done} de {all.length} atividades feitas hoje</T>
          <T style={sans(14, C.muted)}>Novos desafios a cada dia. As marcações começam de novo diariamente.</T>
        </View>
        <View style={{ width: 180, height: 12, borderRadius: 6, backgroundColor: '#fff', overflow: 'hidden' }}>
          <View style={{ width: `${pct}%`, height: 12, backgroundColor: C.green }} />
        </View>
        <T style={[sans(15), { marginLeft: 10 }]}>{pct}% hoje</T>
      </View>
      <View style={{ flexDirection: 'row', flexWrap: 'wrap', gap: 8, marginTop: 16 }}>
        {CATEGORIES.filter((c: any) => c.id === 'all' || all.some((g: any) => g.category === c.id)).map((c: any) => (
          <Btn key={c.id} text={c.label} kind="tab" selected={c.id === s.category} onPress={() => A.setCategory(c.id)} />
        ))}
      </View>
      <View style={{ flexDirection: 'row', flexWrap: 'wrap', gap: 16, marginTop: 16 }}>
        {cards.map((g: any) => {
          const d = dailyOf(g.id);
          return <GameCard key={g.id} g={g} w={cardW} solved={d.solved} completed={d.completed} />;
        })}
      </View>
      <View style={{ height: 1, backgroundColor: C.line, marginTop: 20 }} />
      <View style={{ height: 48, flexDirection: 'row', alignItems: 'center', justifyContent: 'space-between' }}>
        <T style={sans(14, C.muted)}>Feito para a curiosidade florescer.</T>
        <T style={sans(14, C.muted)}>Gratuito · Offline · Sem anúncios</T>
      </View>
    </ScrollView>
  );
}

// ---------------------------------------------------------------- tela do jogo

function visualXml(v: any, q: any): string | null {
  switch (v.type) {
    case 'sock': return Art.sock(v.color, v.pattern);
    case 'cup': return Art.cup(v.amount);
    case 'container': return Art.container(v.place);
    case 'tower': return Art.tower(v.colors);
    case 'position': return Art.position(v.direction);
    case 'sized': return Art.icon(v.shape, v.color);
    case 'swatch': case 'text': case 'number': case 'quantity': return null;
    default: return Art.icon(v.shape || v.type, v.color);
  }
}

function Visual({ v, q }: { v: any; q: any }) {
  const p = q.presentation;
  if (v.type === 'number') return <T style={serif(52)}>{v.value}</T>;
  if (v.type === 'text') return <T style={serif(String(v.value).length > 3 ? 28 : 46)}>{v.value}</T>;
  if (v.type === 'swatch') return <View style={{ width: 80, height: 80, borderRadius: 18, backgroundColor: v.color }} />;
  if (v.type === 'quantity') {
    const n = v.count, cols = Math.max(1, Math.min(n, n <= 4 ? 2 : n <= 9 ? 3 : 4)), cell = Math.min(124 / cols, 114 / Math.ceil(n / cols));
    return (
      <View style={{ width: cell * cols, flexDirection: 'row', flexWrap: 'wrap' }}>
        {Array.from({ length: n }, (_, i) => (
          <View key={i} style={{ transform: [{ rotate: `${p.rotation[i % 24]}deg` }] }}><Svg xml={Art.icon('apple', p.itemColor)} w={cell} h={cell} /></View>
        ))}
      </View>
    );
  }
  const size = v.type === 'sized' ? 114 * v.size : 114;
  return <Svg xml={visualXml(v, q)!} w={size} h={size} />;
}

function Wiggle({ serial, active, children }: { serial: number; active: boolean; children: React.ReactNode }) {
  const x = useRef(new Animated.Value(0)).current;
  useEffect(() => {
    if (!active) return;
    Animated.sequence([-6, 6, -4, 4, 0].map(to => Animated.timing(x, { toValue: to, duration: 80, useNativeDriver: true }))).start();
  }, [serial, active, x]);
  return <Animated.View style={{ transform: [{ translateX: x }] }}>{children}</Animated.View>;
}

const Missing = ({ w, h }: { w: number; h: number }) => (
  <View style={{ width: w, height: h, borderRadius: 14, backgroundColor: '#fffdf8', borderWidth: 2.5, borderColor: C.gold, alignItems: 'center', justifyContent: 'center' }}>
    <T style={serif(34, '#c8963c')}>?</T>
  </View>
);

function Apple({ s, idx, size }: { s: Store; idx: number; size: number }) {
  const p = s.q.presentation, on = s.counted.includes(idx);
  return (
    <Pressable onPress={() => A.countItem(idx)} style={{
      width: size, height: size, padding: 4, borderRadius: size / 2,
      backgroundColor: on ? '#e0eddc' : undefined, borderWidth: on ? 2 : 0, borderColor: '#78a36d',
    }}>
      <View style={{ transform: [{ rotate: `${p.rotation[idx % 24]}deg` }] }}><Svg xml={Art.icon('apple', p.itemColor)} w={size - 8} h={size - 8} /></View>
    </Pressable>
  );
}

const row = { flexDirection: 'row' as const, justifyContent: 'center' as const, alignItems: 'center' as const };

function Scene({ s }: { s: Store }) {
  const sc = s.q.scene, q = s.q;
  if (!sc) return null;
  switch (sc.type) {
    case 'color': return <View style={{ width: 120, height: 120, borderRadius: 60, backgroundColor: sc.color, alignSelf: 'center' }} />;
    case 'shape': return <View style={row}><Svg xml={Art.icon(sc.shape, sc.color)} w={140} h={140} /></View>;
    case 'sock': return <View style={row}><Svg xml={Art.sock(sc.color, sc.pattern)} w={150} h={150} /></View>;
    case 'tower': return <View style={row}><Svg xml={Art.tower(sc.colors)} w={150} h={150} /></View>;
    case 'clock': return <View style={row}><Svg xml={Art.clock(sc.hour)} w={170} h={170} /></View>;
    case 'object-model':
      return <View style={row}><View style={{ width: 150, height: 150, padding: 14, borderRadius: 24, backgroundColor: '#fff' }}><Svg xml={Art.icon(sc.icon)} w={122} h={122} /></View></View>;
    case 'look': return <T style={[sans(16, C.muted), { textAlign: 'center' }]}>Observe com atenção. Você consegue!</T>;
    case 'items':
      return <View style={[row, { flexWrap: 'wrap', gap: q.presentation.itemGap }]}>{Array.from({ length: sc.count }, (_, i) => <Apple key={i} s={s} idx={i} size={64} />)}</View>;
    case 'sum':
      return (
        <View style={row}>
          <View style={{ width: Math.min(sc.a, 5) * 52, flexDirection: 'row', flexWrap: 'wrap', gap: 6 }}>{Array.from({ length: sc.a }, (_, i) => <Apple key={i} s={s} idx={i} size={46} />)}</View>
          <T style={[serif(40), { marginHorizontal: 16 }]}>+</T>
          <View style={{ width: Math.min(sc.b, 5) * 52, flexDirection: 'row', flexWrap: 'wrap', gap: 6 }}>{Array.from({ length: sc.b }, (_, i) => <Apple key={i} s={s} idx={sc.a + i} size={46} />)}</View>
        </View>
      );
    case 'sequence':
      return <View style={[row, { gap: 6 }]}>{sc.items.map((it: string, i: number) => <Svg key={i} xml={Art.icon(it, '#8b7bbd')} w={84} h={84} />)}<Missing w={72} h={72} /></View>;
    case 'numbers':
      return (
        <View style={[row, { alignItems: 'flex-end', height: 120, gap: 12 }]}>
          {sc.items.map((n: number | null, i: number) => (
            <View key={i} style={{ marginBottom: i * 12 }}>
              {n === null ? <Missing w={80} h={70} /> : (
                <View style={{ width: 80, height: 70, borderRadius: 14, backgroundColor: '#fff', borderWidth: 2, borderColor: C.line, alignItems: 'center', justifyContent: 'center' }}>
                  <T style={serif(34)}>{n}</T>
                </View>
              )}
            </View>
          ))}
        </View>
      );
    case 'friends': return <View style={row}>{Array.from({ length: sc.count }, (_, i) => <Svg key={i} xml={Art.icon('bunny')} w={80} h={78} />)}</View>;
    case 'color-sequence':
      return <View style={[row, { gap: 16 }]}>{sc.colors.map((c: string, i: number) => <View key={i} style={{ width: 68, height: 68, borderRadius: 34, backgroundColor: c }} />)}<Missing w={68} h={68} /></View>;
    case 'letter':
      return <View style={row}><View style={{ width: 260, height: 120, borderRadius: 24, backgroundColor: '#fff', alignItems: 'center', justifyContent: 'center' }}><T style={serif(64)}>{sc.value}</T></View></View>;
    case 'word-picture':
      return <View style={[row, { gap: 20 }]}><Svg xml={Art.icon(sc.icon)} w={130} h={130} />{!!sc.word && <T style={serif(46)}>{sc.word}</T>}</View>;
    case 'subtract':
      return (
        <View style={[row, { gap: 6 }]}>
          {Array.from({ length: sc.a }, (_, i) => {
            const gone = i >= sc.a - sc.b;
            return (
              <View key={i} style={{ width: 52, height: 52, alignItems: 'center', justifyContent: 'center' }}>
                <View style={{ opacity: gone ? 0.4 : 1 }}><Svg xml={Art.icon('apple')} w={52} h={52} /></View>
                {gone && <View style={{ position: 'absolute', width: 58, height: 4, borderRadius: 2, backgroundColor: '#c95a46', transform: [{ rotate: '45deg' }] }} />}
              </View>
            );
          })}
        </View>
      );
    case 'equation':
      return (
        <View style={[row, { gap: 8 }]}>
          {[String(sc.a), '+', null, '=', String(sc.total)].map((t, i) => t === null ? <Missing key={i} w={72} h={72} /> : (
            <View key={i} style={{ width: 72, height: 72, alignItems: 'center', justifyContent: 'center' }}><T style={serif(40)}>{t}</T></View>
          ))}
        </View>
      );
    case 'tenframe':
      return (
        <View style={row}>
          <View style={{ padding: 6, borderRadius: 12, backgroundColor: '#fff', borderWidth: 2, borderColor: '#92b4ad', width: 5 * 56 + 16, flexDirection: 'row', flexWrap: 'wrap' }}>
            {Array.from({ length: 10 }, (_, i) => (
              <View key={i} style={{ width: 56, height: 56, borderWidth: 1, borderColor: C.line, alignItems: 'center', justifyContent: 'center' }}>
                {i < sc.count && <View style={{ width: 38, height: 38, borderRadius: 19, backgroundColor: '#e87961' }} />}
              </View>
            ))}
          </View>
        </View>
      );
    case 'tens':
      return (
        <View style={[row, { alignItems: 'flex-end', gap: 14 }]}>
          {Array.from({ length: sc.tens }, (_, t) => (
            <View key={t}>{Array.from({ length: 10 }, (_, k) => <View key={k} style={{ width: 13, height: 13, borderRadius: 2, backgroundColor: '#5799be', borderWidth: 0.5, borderColor: '#fff' }} />)}</View>
          ))}
          <View style={{ width: 16 }} />
          {Array.from({ length: sc.units }, (_, u) => <View key={u} style={{ width: 13, height: 13, borderRadius: 2, backgroundColor: '#e0b849' }} />)}
        </View>
      );
    case 'story':
      return (
        <View style={[row, { gap: 12, maxWidth: 720, alignSelf: 'center' }]}>
          <Svg xml={Art.icon(sc.icon)} w={96} h={96} />
          <T style={[sans(19), { flex: 1 }]}>{sc.text}</T>
        </View>
      );
    default: return null;
  }
}

function Game({ s }: { s: Store }) {
  const q = s.q, p = q.presentation;
  const g = GAMES.find((x: any) => x.id === s.game);
  const fb = s.feedback;
  return (
    <ScrollView showsVerticalScrollIndicator={false}>
      <View style={{ height: 56, flexDirection: 'row', alignItems: 'center', gap: 16 }}>
        <Btn text="← Jardim" onPress={A.home} />
        <Logo />
        <View style={{ flex: 1 }} />
        <Btn text={s.voice ? '♪ Som ligado' : '♪ Som desligado'} onPress={A.toggleVoice} />
      </View>
      <View style={{ marginTop: 12, padding: 20, borderRadius: p.radius + 8, backgroundColor: p.surface, gap: 10 }}>
        <View style={{ flexDirection: 'row', justifyContent: 'space-between' }}>
          <T style={sans(15, C.muted)}>{g.name}</T>
          <T style={sans(15, C.muted)}>Descoberta {s.round + 1} de {s.length}</T>
        </View>
        <View style={[row, { gap: 10 }]}>
          {Array.from({ length: s.length }, (_, i) => (
            <View key={i} style={{
              width: i === s.round ? 14 : 12, height: i === s.round ? 14 : 12, borderRadius: 7,
              backgroundColor: i < s.round ? C.green : i === s.round ? C.gold : undefined,
              borderWidth: i > s.round ? 1.5 : 0, borderColor: '#c9d3bf',
            }} />
          ))}
        </View>
        <View style={[row, { gap: 12 }]}>
          <T style={[serif(30), { textAlign: 'center', flexShrink: 1 }]}>{q.prompt}</T>
          <Pressable onPress={A.listen} style={{ width: 52, height: 52, borderRadius: 26, backgroundColor: '#fff', borderWidth: 1, borderColor: C.line, alignItems: 'center', justifyContent: 'center' }}>
            <T style={sans(22, C.green)}>🔊</T>
          </Pressable>
        </View>
        <T style={[sans(15, C.muted), { textAlign: 'center' }]}>{q.mode === 'memory' ? 'Toque nas cartas para descobrir os desenhos.' : 'Toque na resposta que você escolheu.'}</T>
        {q.mode !== 'memory' && <Scene s={s} />}
        {q.mode !== 'memory' && (
          <View style={[row, { flexWrap: 'wrap', gap: 18, marginTop: 8 }]}>
            {q.options.map((v: any, i: number) => {
              const right = s.correct && i === q.answer, retry = s.wiggle === i;
              return (
                <Wiggle key={i} serial={s.wiggleSerial} active={retry}>
                  <Pressable disabled={s.correct} onPress={() => A.answer(i)} style={{ width: 150, height: 146 }}>
                    <View style={{ position: 'absolute', left: 0, right: 0, top: 6, height: 140, borderRadius: Math.min(p.radius, 40), backgroundColor: right ? '#78a36d' : '#e3e6d6' }} />
                    <View style={{
                      width: 150, height: 140, padding: 13, borderRadius: Math.min(p.radius, 40), borderWidth: 2, alignItems: 'center', justifyContent: 'center',
                      backgroundColor: right ? '#e1eed8' : retry ? '#f8ebdb' : '#fff', borderColor: right ? '#78a36d' : '#e5e7d9',
                    }}>
                      <Visual v={v} q={q} />
                    </View>
                  </Pressable>
                </Wiggle>
              );
            })}
          </View>
        )}
        {q.mode === 'memory' && (
          <View style={[row, { flexWrap: 'wrap', gap: 16, width: (q.cards.length <= 4 ? 2 : 4) * 156, alignSelf: 'center' }]}>
            {q.cards.map((c: any, i: number) => {
              const matched = s.matched.includes(i), open = matched || s.revealed.includes(i);
              return (
                <Pressable key={i} disabled={matched} onPress={() => A.flip(i)} style={{ width: 140, height: 145 }}>
                  <View style={{ position: 'absolute', left: 0, right: 0, top: 5, height: 140, borderRadius: 20, backgroundColor: matched ? '#95b681' : '#c3d2b4' }} />
                  <View style={{
                    width: 140, height: 140, padding: 22, borderRadius: 20, borderWidth: 2,
                    backgroundColor: matched ? '#eff5e9' : open ? '#fffdf7' : '#e2ead6', borderColor: matched ? '#95b681' : open ? '#d8cba5' : '#c5d6b7',
                  }}>
                    <Svg xml={Art.icon(open ? c.kind : 'leaf')} w={92} h={92} />
                  </View>
                </Pressable>
              );
            })}
          </View>
        )}
        <View style={{ height: 1, backgroundColor: C.line, marginTop: 8 }} />
        <View style={{ minHeight: 64, flexDirection: 'row', alignItems: 'center', gap: 12 }}>
          <T style={[fb === 'success' ? serif(18, C.success) : sans(17), { flex: 1 }]}>
            {fb === 'success' ? '★ Você descobriu! Muito bem.' : fb === 'hint' ? q.hint : fb === 'retry' ? `Vamos tentar de novo? ${q.hint}` : 'Vamos descobrir juntos?'}
          </T>
          {s.correct
            ? <Btn kind="primary" text={s.round + 1 === s.length ? 'Ver meu jardim →' : 'Próxima descoberta →'} onPress={A.next} />
            : <Btn text={fb === 'idle' ? 'Uma dica' : 'Ouvir dica'} onPress={A.hint} />}
        </View>
      </View>
      <View style={{ height: 20 }} />
    </ScrollView>
  );
}

// ---------------------------------------------------------------- final e diálogos

function Finish({ s }: { s: Store }) {
  const d = dailyOf(s.game);
  const scale = useRef(new Animated.Value(0.6)).current;
  useEffect(() => { Animated.spring(scale, { toValue: 1, useNativeDriver: true, friction: 5 }).start(); }, [scale]);
  return (
    <ScrollView contentContainerStyle={{ alignItems: 'center' }}>
      <View style={{ alignSelf: 'stretch' }}><Header s={s} /></View>
      <View style={{ height: 30 }} />
      <Animated.View style={{ transform: [{ scale }] }}><Svg xml={Art.icon('flower')} w={130} h={130} /></Animated.View>
      <Eyebrow>A CURIOSIDADE FLORESCEU!</Eyebrow>
      <T style={serif(40)}>{s.length === 1 ? 'Uma nova descoberta!' : `${s.length} novas descobertas!`}</T>
      <T style={serif(18, C.success)}>{d.completed ? '✓ Atividade feita hoje!' : `${d.solved} de 5 descobertas hoje`}</T>
      <View style={{ height: 8 }} />
      <T style={sans(17, C.muted)}>Você observou, tentou e aprendeu.</T>
      <T style={sans(17, C.muted)}>Que tal alongar o corpo e olhar o mundo lá fora?</T>
      <T style={sans(22, C.gold)}>{'★ '.repeat(s.length)}</T>
      <View style={{ height: 14 }} />
      <Btn kind="primary" text="Voltar para o jardim" onPress={A.home} />
      <View style={{ height: 10 }} />
      <T style={sans(14, C.muted)}>Cada tentativa faz parte de aprender.</T>
    </ScrollView>
  );
}

function Modal({ s }: { s: Store }) {
  const [gate] = useState(() => [6 + Math.floor(Math.random() * 4), 6 + Math.floor(Math.random() * 4)]);
  const [input, setInput] = useState('');
  const [error, setError] = useState(false);
  const { height } = useWindowDimensions();
  const H2 = ({ t }: { t: string }) => <T style={serif(26)}>{t}</T>;
  const H3 = ({ t }: { t: string }) => <T style={[serif(18), { marginTop: 10 }]}>{t}</T>;
  const P = ({ t, size = 15 }: { t: string; size?: number }) => <T style={sans(size, C.muted)}>{t}</T>;
  return (
    <View style={[StyleSheet.absoluteFill, { backgroundColor: '#233b36a8', alignItems: 'center', justifyContent: 'center' }]}>
      <View style={{ width: 560, maxHeight: height * 0.9, borderRadius: 27, backgroundColor: C.paper, padding: 30 }}>
        <ScrollView>
          {s.modal === 'gate' && (
            <View style={{ gap: 8 }}>
              <Eyebrow>UM MOMENTO PARA OS ADULTOS</Eyebrow>
              <H2 t="Vamos cuidar do jardim?" />
              <P t="Para abrir as configurações, resolva:" size={16} />
              <View style={{ flexDirection: 'row', alignItems: 'center', gap: 12 }}>
                <T style={serif(22)}>Quanto é {gate[0]} × {gate[1]}?</T>
                <View style={{ width: 110, height: 50, borderRadius: 12, backgroundColor: '#fff', borderWidth: 2, borderColor: C.line, alignItems: 'center', justifyContent: 'center' }}>
                  <T style={serif(26)}>{input}</T>
                </View>
              </View>
              {error && <T style={sans(15, '#9c513c')}>Tente mais uma vez.</T>}
              <View style={{ width: 3 * 80, flexDirection: 'row', flexWrap: 'wrap', gap: 8 }}>
                {['1', '2', '3', '4', '5', '6', '7', '8', '9', '⌫', '0'].map(k => (
                  <Pressable key={k} onPress={() => {
                    setError(false);
                    if (k === '⌫') setInput(input.slice(0, -1)); else if (input.length < 3) setInput(input + k);
                  }} style={({ pressed }) => ({ width: 72, height: 52, borderRadius: 12, backgroundColor: pressed ? C.line : C.quiet, alignItems: 'center', justifyContent: 'center' })}>
                    <T style={serif(22)}>{k}</T>
                  </Pressable>
                ))}
              </View>
              <View style={{ flexDirection: 'row', gap: 10, marginTop: 6 }}>
                <Btn text="Voltar" onPress={() => A.setModal('')} />
                <Btn text="Entrar" kind="primary" onPress={() => (parseInt(input, 10) === gate[0] * gate[1] ? A.setModal('parents') : setError(true))} />
              </View>
            </View>
          )}
          {s.modal === 'parents' && (
            <View style={{ gap: 6 }}>
              <Eyebrow>ÁREA DOS ADULTOS</Eyebrow>
              <H2 t="Aprender no próprio ritmo" />
              <P t="As faixas são sugestões. Escolha a que combina com a criança e acompanhe as atividades, especialmente aos 2–3 anos." />
              <H3 t="Faixa etária" />
              <View style={{ flexDirection: 'row', gap: 4 }}>
                {GROUPS.map((g: any) => <Btn key={g.id} text={g.label} kind="pill" selected={g.id === s.group} onPress={() => A.setGroup(g.id)} />)}
              </View>
              <H3 t="Narração em português" />
              <View style={{ alignSelf: 'flex-start' }}><Btn text={s.voice ? 'Ligada — toque para desligar' : 'Desligada — toque para ligar'} onPress={A.toggleVoice} /></View>
              <P size={13} t="Usa a voz do Android. O primeiro uso pode exigir uma voz pt-BR já instalada. As atividades funcionam sem áudio." />
              <H3 t={`Descobertas por atividade · ${GROUPS[level()].label}`} />
              {GAMES.map((g: any) => (
                <View key={g.id} style={{ flexDirection: 'row', justifyContent: 'space-between' }}>
                  <T style={sans(14)}>{g.name}</T>
                  <T style={sans(14, C.muted)}>{dailyOf(g.id).solved}/5 hoje · {stat(g.id).solved} no total</T>
                </View>
              ))}
              <P size={13} t="Os dados ficam somente neste dispositivo. Sem conta, anúncios, compras ou acesso à internet. A pergunta de entrada é uma barreira simples para crianças pequenas." />
              <View style={{ flexDirection: 'row', gap: 10, marginTop: 8 }}>
                <Btn kind="danger" text="Apagar progresso" onPress={() => A.setModal('reset')} />
                <Btn kind="primary" text="Pronto" onPress={A.home} />
              </View>
            </View>
          )}
          {s.modal === 'reset' && (
            <View style={{ gap: 10 }}>
              <H2 t="Apagar todas as descobertas?" />
              <P t="Isso apaga o progresso das três faixas etárias neste dispositivo." size={16} />
              <View style={{ flexDirection: 'row', gap: 10, marginTop: 6 }}>
                <Btn text="Cancelar" onPress={() => A.setModal('parents')} />
                <Btn kind="danger" text="Sim, apagar" onPress={() => { A.reset(); A.setModal('parents'); }} />
              </View>
            </View>
          )}
        </ScrollView>
      </View>
    </View>
  );
}

// ---------------------------------------------------------------- raiz

function Root() {
  const s = useStore();
  const insets = useSafeAreaInsets();
  useEffect(() => {
    load();
    const back = BackHandler.addEventListener('hardwareBackPress', () => {
      if (store.modal) { A.setModal(''); return true; }
      if (store.screen !== 'home') { A.home(); return true; }
      return false;
    });
    const app = AppState.addEventListener('change', st => { if (st === 'active') A.resume(); else Native.stop(); });
    return () => { back.remove(); app.remove(); };
  }, []);
  return (
    <View style={{ flex: 1, backgroundColor: C.paper }}>
      <StatusBar barStyle="dark-content" />
      <View style={{ flex: 1, paddingTop: insets.top + 16, paddingBottom: insets.bottom, paddingLeft: insets.left + 24, paddingRight: insets.right + 24 }}>
        {s.screen === 'game' && s.q ? <Game s={s} /> : s.screen === 'finish' ? <Finish s={s} /> : <Home s={s} />}
      </View>
      {!!s.modal && <Modal s={s} />}
    </View>
  );
}

export default function App() {
  return <SafeAreaProvider><Root /></SafeAreaProvider>;
}
