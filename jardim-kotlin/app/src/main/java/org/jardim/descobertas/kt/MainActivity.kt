// Jardim das Descobertas — versão 100% Kotlin (Jetpack Compose), sem WebView.
package org.jardim.descobertas.kt

import android.graphics.Bitmap
import android.os.Bundle
import android.speech.tts.TextToSpeech
import androidx.activity.ComponentActivity
import androidx.activity.compose.BackHandler
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.animation.animateColorAsState
import androidx.compose.animation.core.Animatable
import androidx.compose.animation.core.Spring
import androidx.compose.animation.core.spring
import androidx.compose.animation.core.tween
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsPressedAsState
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.ui.layout.onGloballyPositioned
import androidx.compose.ui.layout.positionInParent
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicText
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.rotate
import androidx.compose.ui.draw.scale
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.Density
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.caverock.androidsvg.SVG
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import org.json.JSONObject
import java.io.File
import java.time.LocalDate
import java.util.Locale
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.Executors
import kotlin.math.ceil
import kotlin.math.floor
import kotlin.math.max
import kotlin.math.min
import kotlin.math.roundToInt

private const val GOAL = 5

// ---------------------------------------------------------------- paleta (style.css)
private fun hex(s: String) = Color(android.graphics.Color.parseColor(s))
private val Ink = hex("#294b43"); private val Muted = hex("#6d7d70"); private val Paper = hex("#f8f6ee")
private val Green = hex("#347b65"); private val GreenShadow = hex("#26614f"); private val Line = hex("#e2e6d9")
private val Quiet = hex("#eeeee4"); private val QuietBorder = hex("#dedfd2"); private val EyebrowC = hex("#7f8c73")
private val Em = hex("#56856b"); private val Success = hex("#387657"); private val Gold = hex("#e9bd68")
private fun cardBg(c: String) = hex(mapOf("blue" to "#deedf2", "yellow" to "#faedc8", "green" to "#e0eddc", "purple" to "#e8e2f1", "peach" to "#f6e7d5")[c] ?: "#f7e1db")
private fun serif(size: Int, c: Color = Ink) = TextStyle(fontFamily = FontFamily.Serif, fontWeight = FontWeight.Bold, fontSize = size.sp, color = c)
private fun sans(size: Int, c: Color = Ink) = TextStyle(fontSize = size.sp, color = c)
private val eyebrowStyle = TextStyle(fontSize = 12.sp, letterSpacing = 2.sp, fontWeight = FontWeight.Bold, color = EyebrowC)

@Composable private fun T(text: String, style: TextStyle, modifier: Modifier = Modifier, maxLines: Int = Int.MAX_VALUE) =
    BasicText(text, modifier, style, maxLines = maxLines, overflow = TextOverflow.Ellipsis)

// ---------------------------------------------------------------- estado

class Stat(var solved: Int = 0, var tries: Int = 0, var sets: Int = 0, var rounds: Int = 0)

class JardimState(private val dir: File, private val speakFn: (String) -> Unit, private val stopFn: () -> Unit) {
    var version by mutableIntStateOf(0); private set
    fun changed() { version++ }

    var screen = "home"
    var group = "little"; private set
    var category = "all"; private set
    var modal = ""; private set
    var voice = true
    var date = LocalDate.now().toString()
    val stats = HashMap<String, Stat>(); val daily = HashMap<String, Int>()
    var entry = entrySeed()

    var game = ""; var round = 0; var length = 0; var q: Question? = null; var correct = false; var feedback = "idle"
    var busy = false; var revealed = listOf<Int>(); var matched = listOf<Int>(); val counted = HashSet<Int>()
    var wiggle = -1; var wiggleSerial = 0; private var flipToken = 0

    private val io = Executors.newSingleThreadExecutor()
    private val file = File(dir, "jardim-v1.json")

    fun key(g: String) = "$group:$g"
    fun dailySolved(g: String) = daily[key(g)] ?: 0
    fun stat(g: String) = stats[key(g)] ?: Stat()
    val total get() = stats.values.sumOf { it.solved }
    val level get() = groupIndex(group)
    fun catalogAll() = catalog(level, seedFor(date, group, "catalog", 0, entry))

    fun load() {
        runCatching {
            val o = JSONObject(file.readText())
            val valid = { k: String -> k.split(":").let { p -> p.size == 2 && GROUPS.any { it.id == p[0] } && GAMES.any { it.id == p[1] } } }
            if (o.optInt("version") == 1 && GROUPS.any { it.id == o.optString("group") }) {
                group = o.getString("group"); voice = o.optBoolean("voice", true)
                o.optJSONObject("stats")?.let { st -> st.keys().forEach { k -> if (valid(k)) st.getJSONObject(k).let { v -> stats[k] = Stat(v.optInt("solved"), v.optInt("tries"), v.optInt("sets"), v.optInt("rounds")) } } }
                o.optJSONObject("daily")?.takeIf { it.optString("date") == date }?.optJSONObject("activities")?.let { a ->
                    a.keys().forEach { k -> if (valid(k)) daily[k] = min(GOAL, a.getJSONObject(k).optInt("solved")) }
                }
            }
        }
        changed()
    }

    fun save() {
        // Serializa na thread de UI, grava numa thread própria (coalescendo).
        val st = JSONObject(); stats.forEach { (k, s) -> st.put(k, JSONObject().put("solved", s.solved).put("tries", s.tries).put("sets", s.sets).put("rounds", s.rounds)) }
        val acts = JSONObject(); daily.forEach { (k, v) -> acts.put(k, JSONObject().put("solved", v).put("completed", v >= GOAL)) }
        val json = JSONObject().put("version", 1).put("group", group).put("voice", voice).put("stats", st).put("daily", JSONObject().put("date", date).put("activities", acts)).toString()
        io.execute { runCatching { val tmp = File(dir, "jardim-v1.json.tmp"); tmp.writeText(json); tmp.renameTo(file) } }
    }

    private fun ensureDay(): Boolean {
        val t = LocalDate.now().toString()
        if (t == date) return false
        date = t; daily.clear(); save(); return true
    }
    fun speak(t: String) { if (voice) speakFn(t) }
    fun stopVoice() = stopFn()

    fun home() { ensureDay(); stopVoice(); screen = "home"; modal = ""; changed() }
    fun setGroup(g: String) { group = g; category = "all"; save(); changed() }
    fun setCategory(c: String) { category = c; changed() }
    fun setModal(m: String) { if (m == "gate") stopVoice(); modal = m; changed() }

    fun start(g: String) {
        ensureDay(); game = g
        val d = dailySolved(g)
        length = if (d >= GOAL) GOAL else GOAL - d
        round = 0; screen = "game"; nextQuestion()
    }
    private fun nextQuestion() {
        ensureDay()
        val s = stats.getOrPut(key(game)) { Stat() }
        s.rounds++; save()
        q = generate(game, group, seedFor(date, group, game, s.rounds.toLong(), entry))
        correct = false; feedback = "idle"; revealed = emptyList(); matched = emptyList(); counted.clear(); wiggle = -1; busy = false; flipToken++
        changed()
        q!!.let { speak((if (it.story.isEmpty()) "" else it.story + " ") + it.prompt) }
    }
    private fun recordTry() { stats.getOrPut(key(game)) { Stat() }.tries++; save() }
    private fun markCorrect() {
        if (correct) return
        correct = true; feedback = "success"; ensureDay()
        daily[key(game)] = min(GOAL, dailySolved(game) + 1)
        stats.getOrPut(key(game)) { Stat() }.solved++
        save(); changed(); speak("Você descobriu! Muito bem.")
    }
    suspend fun answer(i: Int) {
        if (correct) return
        recordTry()
        if (i == q!!.answer) return markCorrect()
        wiggle = i; val serial = ++wiggleSerial; feedback = "retry"; changed()
        speak("Vamos tentar de novo? ${q!!.hint}")
        delay(500)
        if (serial == wiggleSerial) { wiggle = -1; changed() }
    }
    suspend fun flip(i: Int) {
        if (busy || correct || i in revealed || i in matched) return
        revealed = revealed + i
        if (revealed.size < 2) return changed()
        recordTry()
        val (a, d) = revealed
        if (q!!.cards[a].pair == q!!.cards[d].pair) {
            matched = matched + a + d; revealed = emptyList()
            if (matched.size == q!!.cards.size) return markCorrect()
            changed(); speak("Um par!")
        } else {
            busy = true; changed()
            val token = ++flipToken
            delay(1200)
            if (token == flipToken) { revealed = emptyList(); busy = false; changed() }
        }
    }
    fun next() {
        if (!correct) return
        round++
        if (round == length) {
            ensureDay(); stopVoice(); stats.getOrPut(key(game)) { Stat() }.sets++; save()
            screen = "finish"; changed()
            speak((if (length == 1) "Uma nova descoberta!" else "$length novas descobertas!") + " Que tal fazer uma pausa?")
        } else nextQuestion()
    }
    fun hint() { if (!correct) feedback = "hint"; changed(); q!!.let { speak((if (it.story.isEmpty()) "" else it.story + " ") + it.hint) } }
    fun listen() = q!!.let { speak((if (it.story.isEmpty()) "" else it.story + " ") + it.prompt) }
    fun countItem(i: Int) { if (!counted.remove(i)) counted.add(i); changed() }
    fun toggleVoice() { voice = !voice; save(); if (!voice) stopVoice() else if (screen == "game") speak(q!!.prompt); changed() }
    fun reset() { stats.clear(); daily.clear(); save(); changed() }
    fun resume() { entry = entrySeed(); ensureDay(); changed() }
}

// ---------------------------------------------------------------- Activity

class MainActivity : ComponentActivity() {
    private var tts: TextToSpeech? = null
    private var ready = false
    private lateinit var state: JardimState

    override fun onCreate(savedInstanceState: Bundle?) {
        enableEdgeToEdge()
        super.onCreate(savedInstanceState)
        tts = TextToSpeech(this) { status ->
            if (status == TextToSpeech.SUCCESS) { tts?.language = Locale("pt", "BR"); tts?.setSpeechRate(0.88f); ready = true }
        }
        state = JardimState(filesDir, { if (ready) tts?.speak(it, TextToSpeech.QUEUE_FLUSH, null, "jardim") }, { tts?.stop() })
        state.load()
        setContent { Root(state) { finish() } }
    }
    override fun onResume() { super.onResume(); state.resume() }
    override fun onPause() { super.onPause(); state.stopVoice() }
    override fun onDestroy() { tts?.shutdown(); super.onDestroy() }
}

// ---------------------------------------------------------------- desenhos

/** SVG rasterizado UMA vez (AndroidSVG, em paralelo nos núcleos) e guardado como ImageBitmap. */
object ArtCache {
    private val done = ConcurrentHashMap<String, ImageBitmap>()
    fun peek(k: String) = done[k]
    suspend fun get(svg: String, w: Int, h: Int): ImageBitmap {
        val k = "$w x $h|$svg"
        done[k]?.let { return it }
        return withContext(Dispatchers.Default) {
            val doc = SVG.getFromString(svg)
            doc.documentWidth = w.toFloat(); doc.documentHeight = h.toFloat()
            val bmp = Bitmap.createBitmap(w, h, Bitmap.Config.ARGB_8888)
            doc.renderToCanvas(android.graphics.Canvas(bmp))
            bmp.asImageBitmap().also { done[k] = it }
        }
    }
}

@Composable
private fun Art(svg: String, w: Dp, h: Dp, modifier: Modifier = Modifier) {
    val d = LocalDensity.current
    val pw = with(d) { w.roundToPx() }; val ph = with(d) { h.roundToPx() }
    val k = "$pw x $ph|$svg"
    val img by produceState(ArtCache.peek(k), k) { if (value == null) value = ArtCache.get(svg, pw, ph) }
    Box(modifier.size(w, h)) { img?.let { Image(it, null, Modifier.fillMaxSize()) } }
}

// ---------------------------------------------------------------- componentes

@Composable
private fun Btn(text: String, kind: String = "quiet", selected: Boolean = false, onClick: () -> Unit) {
    val src = remember { MutableInteractionSource() }
    val pressed by src.collectIsPressedAsState()
    val primary = kind == "primary"
    val fill = when (kind) {
        "primary" -> Green; "danger" -> hex("#f2e2d8")
        "pill" -> if (selected) Green else Color.Transparent
        "tab" -> if (selected) hex("#e0eada") else Color.Transparent
        else -> if (pressed) Line else Quiet
    }
    val fg = when (kind) {
        "primary" -> Color.White; "danger" -> hex("#9c513c")
        "pill" -> if (selected) Color.White else Muted
        "tab" -> if (selected) hex("#346950") else Muted
        else -> Ink
    }
    val shape = RoundedCornerShape(if (primary) 16.dp else if (kind == "danger") 15.dp else 24.dp)
    Box(Modifier.clickable(src, null, onClick = onClick)) {
        if (primary) Box(Modifier.matchParentSize().offset(y = 4.dp).background(GreenShadow, shape))
        Box(
            Modifier.offset { IntOffset(0, if (pressed && primary) 6 else 0) }
                .height(if (primary) 56.dp else if (kind == "tab") 42.dp else 46.dp)
                .background(fill, shape)
                .then(if (kind == "quiet" || kind == "tab") Modifier.border(1.dp, if (kind == "tab") (if (selected) hex("#9dba99") else Line) else QuietBorder, shape) else Modifier)
                .padding(horizontal = if (primary) 24.dp else 17.dp),
            contentAlignment = Alignment.Center,
        ) { T(text, if (primary || kind == "danger") serif(if (primary) 18 else 16, fg) else sans(15, fg)) }
    }
}

@Composable
private fun Logo() = Row(verticalAlignment = Alignment.CenterVertically) {
    Art(Art.icon("leaf"), 34.dp, 34.dp)
    Spacer(Modifier.width(8.dp))
    Column { T("jardim", serif(22, Green)); T("das descobertas", sans(11, Muted)) }
}

@Composable
private fun Header(s: JardimState) = Row(Modifier.fillMaxWidth().height(56.dp), verticalAlignment = Alignment.CenterVertically) {
    Logo()
    Spacer(Modifier.weight(1f))
    T("★ ${s.total} descobertas", sans(15, EyebrowC))
    Spacer(Modifier.width(12.dp))
    Btn("Área dos adultos") { s.setModal("gate") }
}

// ---------------------------------------------------------------- raiz

@Composable
fun Root(s: JardimState, exit: () -> Unit) {
    s.version // lê o estado: qualquer mudança recompõe
    val d = LocalDensity.current
    BackHandler {
        when {
            s.modal.isNotEmpty() -> s.setModal("")
            s.screen != "home" -> s.home()
            else -> exit()
        }
    }
    // Mesmo tamanho de texto das outras versões (sem a escala de fonte do sistema).
    CompositionLocalProvider(LocalDensity provides Density(d.density, 1f)) {
        Box(Modifier.fillMaxSize().background(Paper)) {
            Box(Modifier.safeDrawingPadding().padding(start = 24.dp, end = 24.dp, top = 16.dp)) {
                when (s.screen) {
                    "game" -> GameScreen(s)
                    "finish" -> FinishScreen(s)
                    else -> HomeScreen(s)
                }
            }
            if (s.modal.isNotEmpty()) ModalLayer(s)
        }
    }
}

// ---------------------------------------------------------------- tela inicial

@Composable
private fun HomeScreen(s: JardimState) {
    s.version
    val all = s.catalogAll()
    val cards = all.filter { s.category == "all" || it.category == s.category }
    val done = all.count { s.dailySolved(it.id) >= GOAL }
    val pct = if (all.isEmpty()) 0 else (100.0 * done / all.size).roundToInt()
    val scroll = rememberScrollState()
    var catalogY by remember { mutableIntStateOf(0) }
    val scope = rememberCoroutineScope()
    BoxWithConstraints {
        val w = maxWidth
        val cols = max(1, floor((w.value + 16) / (250 + 16)).toInt())
        val cardW = (w - 16.dp * (cols - 1)) / cols
        val gardenW = minOf(460.dp, w * 0.45f)
        // Column + verticalScroll: só 18-32 cartões — compõe uma vez e rola sem recompor.
        Column(Modifier.verticalScroll(scroll)) {
            Header(s)
            Row(Modifier.padding(top = 18.dp), verticalAlignment = Alignment.CenterVertically) {
                Column(Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(6.dp)) {
                    T("BRINCAR • PENSAR • DESCOBRIR", eyebrowStyle)
                    Column { T("Pequenas descobertas.", serif(40)); T("Grandes sorrisos.", serif(40, Em)) }
                    T("Um jardim de ideias para aprender brincando, um passo de cada vez.", sans(17, Muted))
                    Spacer(Modifier.height(10.dp))
                    Btn("Vamos explorar →", "primary") { s.speak("Escolha uma atividade para começar."); scope.launch { scroll.animateScrollTo(catalogY) } }
                }
                Spacer(Modifier.width(20.dp))
                Art(Art.garden(), gardenW, gardenW * 300f / 520f)
            }
            Column(Modifier.padding(top = 28.dp).onGloballyPositioned { catalogY = it.positionInParent().y.roundToInt() }) {
                FlowRow(horizontalArrangement = Arrangement.spacedBy(16.dp), verticalArrangement = Arrangement.Center) {
                    Column { T("CADA IDADE, UMA DESCOBERTA", eyebrowStyle); T("O que vamos brincar hoje?", serif(30)) }
                    Row(Modifier.align(Alignment.CenterVertically), horizontalArrangement = Arrangement.spacedBy(4.dp)) {
                        GROUPS.forEach { g -> Btn(g.label, "pill", g.id == s.group) { s.setGroup(g.id) } }
                    }
                }
                T("${GROUPS[s.level].name} • ${all.size} atividades gratuitas", sans(15, Muted), Modifier.padding(top = 6.dp))
                Row(
                    Modifier.padding(top = 14.dp).fillMaxWidth().background(hex("#eaf0e2"), RoundedCornerShape(22.dp))
                        .border(1.dp, hex("#d4dfc7"), RoundedCornerShape(22.dp)).padding(horizontal = 22.dp, vertical = 20.dp),
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    Column(Modifier.weight(1f)) {
                        T("SEU JARDIM DE HOJE", eyebrowStyle)
                        T("$done de ${all.size} atividades feitas hoje", serif(21))
                        T("Novos desafios a cada dia. As marcações começam de novo diariamente.", sans(14, Muted))
                    }
                    Box(Modifier.size(180.dp, 12.dp).background(Color.White, RoundedCornerShape(6.dp))) {
                        Box(Modifier.fillMaxHeight().fillMaxWidth(pct / 100f).background(Green, RoundedCornerShape(6.dp)))
                    }
                    T("$pct% hoje", sans(15), Modifier.padding(start = 10.dp))
                }
                FlowRow(Modifier.padding(top = 16.dp, bottom = 16.dp), horizontalArrangement = Arrangement.spacedBy(8.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
                    CATEGORIES.filter { (id, _) -> id == "all" || all.any { it.category == id } }.forEach { (id, label) ->
                        Btn(label, "tab", id == s.category) { s.setCategory(id) }
                    }
                }
            }
            cards.chunked(cols).forEach { row ->
                Row(Modifier.padding(bottom = 16.dp).graphicsLayer(), horizontalArrangement = Arrangement.spacedBy(16.dp)) {
                    row.forEach { g -> GameCard(g, cardW, s.dailySolved(g.id)) { s.start(g.id) } }
                }
            }
            Column {
                Box(Modifier.padding(top = 4.dp).fillMaxWidth().height(1.dp).background(Line))
                Row(Modifier.fillMaxWidth().height(48.dp), verticalAlignment = Alignment.CenterVertically) {
                    T("Feito para a curiosidade florescer.", sans(14, Muted))
                    Spacer(Modifier.weight(1f))
                    T("Gratuito · Offline · Sem anúncios", sans(14, Muted))
                }
            }
        }
    }
}

@Composable
private fun GameCard(g: Game, w: Dp, solved: Int, onClick: () -> Unit) {
    val completed = solved >= GOAL
    val wide = Art.tileWide(g.icon)
    val shape = RoundedCornerShape(22.dp)
    Box(Modifier.size(w, 294.dp).clickable(onClick = onClick)) {
        Box(Modifier.fillMaxWidth().height(290.dp).offset(y = 4.dp).background(hex("#e8e9de"), shape))
        Column(Modifier.fillMaxWidth().height(290.dp).background(Color.White, shape).border(1.dp, hex("#e6e6d9"), shape)) {
            Box(Modifier.fillMaxWidth().height(140.dp).background(cardBg(g.color), RoundedCornerShape(topStart = 21.dp, topEnd = 21.dp)), contentAlignment = Alignment.Center) {
                Art(Art.tile(g.icon), if (wide) 180.dp else 110.dp, if (wide) 90.dp else 110.dp)
                T("→", sans(18), Modifier.align(Alignment.TopEnd).padding(top = 12.dp, end = 16.dp).rotate(-45f))
            }
            Column(Modifier.padding(start = 16.dp, end = 16.dp, top = 12.dp)) {
                T(g.name, serif(19), maxLines = 1)
                T(g.skill, sans(14, Muted))
                Column(Modifier.padding(top = 8.dp).background(if (completed) hex("#f1f7e9") else Color.Transparent, RoundedCornerShape(9.dp)).padding(6.dp)) {
                    T(if (completed) "✓ Feita hoje" else if (solved > 0) "Em andamento hoje" else "Para descobrir hoje", serif(14, if (completed) Success else Ink))
                    T("$solved de 5 descobertas", sans(13, Muted))
                    Row(Modifier.padding(top = 4.dp), horizontalArrangement = Arrangement.spacedBy(6.dp)) {
                        repeat(GOAL) { i ->
                            Box(Modifier.size(10.dp).then(if (i < solved) Modifier.background(Green, CircleShape) else Modifier.border(1.5.dp, hex("#c9d3bf"), CircleShape)))
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------- tela do jogo

private fun visualSvg(v: Visual) = when (v.type) {
    "icon", "sized" -> Art.icon(v.kind, v.color)
    "position" -> Art.position(v.kind)
    "sock" -> Art.sock(v.color, v.kind)
    "cup" -> Art.cup(v.number)
    "container" -> Art.container(v.kind)
    "tower" -> Art.tower(v.colors)
    else -> ""
}

@Composable
private fun GameScreen(s: JardimState) {
    s.version
    val q = s.q ?: return
    val p = q.presentation
    val scope = rememberCoroutineScope()
    val surface by animateColorAsState(hex(p.surface), tween(250), label = "surface")
    Column(Modifier.verticalScroll(rememberScrollState())) {
        Row(Modifier.fillMaxWidth().height(56.dp), verticalAlignment = Alignment.CenterVertically) {
            Btn("← Jardim") { s.home() }
            Spacer(Modifier.width(16.dp))
            Logo()
            Spacer(Modifier.weight(1f))
            Btn(if (s.voice) "♪ Som ligado" else "♪ Som desligado") { s.toggleVoice() }
        }
        Column(
            Modifier.padding(top = 12.dp).fillMaxWidth().background(surface, RoundedCornerShape((p.radius + 8).dp)).padding(20.dp),
            horizontalAlignment = Alignment.CenterHorizontally, verticalArrangement = Arrangement.spacedBy(10.dp),
        ) {
            Row(Modifier.fillMaxWidth()) {
                T(gameById(s.game).name, sans(15, Muted)); Spacer(Modifier.weight(1f)); T("Descoberta ${s.round + 1} de ${s.length}", sans(15, Muted))
            }
            Row(horizontalArrangement = Arrangement.spacedBy(10.dp), verticalAlignment = Alignment.CenterVertically) {
                repeat(s.length) { i ->
                    val sz = if (i == s.round) 14.dp else 12.dp
                    Box(Modifier.size(sz).then(when {
                        i < s.round -> Modifier.background(Green, CircleShape)
                        i == s.round -> Modifier.background(Gold, CircleShape)
                        else -> Modifier.border(1.5.dp, hex("#c9d3bf"), CircleShape)
                    }))
                }
            }
            Row(verticalAlignment = Alignment.CenterVertically, horizontalArrangement = Arrangement.spacedBy(12.dp)) {
                T(q.prompt, serif(30).copy(textAlign = TextAlign.Center), Modifier.weight(1f, fill = false))
                Box(Modifier.size(52.dp).background(Color.White, CircleShape).border(1.dp, Line, CircleShape).clickable { s.listen() }, contentAlignment = Alignment.Center) {
                    T("🔊", sans(22, Green))
                }
            }
            T(if (q.memory) "Toque nas cartas para descobrir os desenhos." else "Toque na resposta que você escolheu.", sans(15, Muted))
            if (!q.memory) {
                q.scene?.let { SceneView(s, it) }
                FlowRow(Modifier.padding(top = 8.dp), horizontalArrangement = Arrangement.spacedBy(18.dp, Alignment.CenterHorizontally), verticalArrangement = Arrangement.spacedBy(18.dp)) {
                    q.options.forEachIndexed { i, o -> Choice(s, q, i, o.visual) { scope.launch { s.answer(i) } } }
                }
            } else {
                FlowRow(Modifier.width(((if (q.cards.size <= 4) 2 else 4) * 156).dp), horizontalArrangement = Arrangement.spacedBy(16.dp, Alignment.CenterHorizontally), verticalArrangement = Arrangement.spacedBy(16.dp)) {
                    q.cards.forEachIndexed { i, c ->
                        val isMatched = i in s.matched; val open = isMatched || i in s.revealed
                        val sh = RoundedCornerShape(20.dp)
                        Box(Modifier.size(140.dp, 145.dp).clickable(enabled = !isMatched) { scope.launch { s.flip(i) } }) {
                            Box(Modifier.fillMaxWidth().height(140.dp).offset(y = 5.dp).background(if (isMatched) hex("#95b681") else hex("#c3d2b4"), sh))
                            Box(
                                Modifier.size(140.dp).background(if (isMatched) hex("#eff5e9") else if (open) hex("#fffdf7") else hex("#e2ead6"), sh)
                                    .border(2.dp, if (isMatched) hex("#95b681") else if (open) hex("#d8cba5") else hex("#c5d6b7"), sh).padding(22.dp),
                            ) { Art(Art.icon(if (open) c.kind else "leaf"), 96.dp, 96.dp) }
                        }
                    }
                }
            }
            Box(Modifier.padding(top = 8.dp).fillMaxWidth().height(1.dp).background(Line))
            Row(Modifier.fillMaxWidth().heightIn(min = 64.dp), verticalAlignment = Alignment.CenterVertically) {
                val fb = s.feedback
                T(
                    when (fb) { "success" -> "★ Você descobriu! Muito bem."; "hint" -> q.hint; "retry" -> "Vamos tentar de novo? ${q.hint}"; else -> "Vamos descobrir juntos?" },
                    if (fb == "success") serif(18, Success) else sans(17), Modifier.weight(1f),
                )
                if (s.correct) Btn(if (s.round + 1 == s.length) "Ver meu jardim →" else "Próxima descoberta →", "primary") { s.next() }
                else Btn(if (fb == "idle") "Uma dica" else "Ouvir dica") { s.hint() }
            }
        }
        Spacer(Modifier.height(20.dp))
    }
}

@Composable
private fun Choice(s: JardimState, q: Question, i: Int, v: Visual, onClick: () -> Unit) {
    val right = s.correct && i == q.answer
    val retry = s.wiggle == i
    val x = remember { Animatable(0f) }
    LaunchedEffect(s.wiggleSerial, retry) {
        if (retry) for (to in listOf(-6f, 6f, -4f, 4f, 0f)) x.animateTo(to, tween(80))
    }
    val sh = RoundedCornerShape(min(q.presentation.radius, 40).dp)
    val bg by animateColorAsState(if (right) hex("#e1eed8") else if (retry) hex("#f8ebdb") else Color.White, tween(150), label = "bg")
    Box(Modifier.size(150.dp, 146.dp).graphicsLayer { translationX = x.value * density }.clickable(enabled = !s.correct, onClick = onClick)) {
        Box(Modifier.fillMaxWidth().height(140.dp).offset(y = 6.dp).background(if (right) hex("#78a36d") else hex("#e3e6d6"), sh))
        Box(Modifier.size(150.dp, 140.dp).background(bg, sh).border(2.dp, if (right) hex("#78a36d") else hex("#e5e7d9"), sh).padding(13.dp), contentAlignment = Alignment.Center) {
            VisualView(v, q)
        }
    }
}

@Composable
private fun VisualView(v: Visual, q: Question) {
    val p = q.presentation
    when (v.type) {
        "number" -> T("${v.number.toInt()}", serif(52))
        "text" -> T(v.text, serif(if (v.text.length > 3) 28 else 46))
        "swatch" -> Box(Modifier.size(80.dp).background(hex(v.color), RoundedCornerShape(18.dp)))
        "quantity" -> {
            val n = v.number.toInt()
            val cols = max(1, min(n, if (n <= 4) 2 else if (n <= 9) 3 else 4))
            val cell = min(124f / cols, 114f / ceil(n / cols.toFloat())).dp
            FlowRow(Modifier.width(cell * cols)) {
                repeat(n) { k -> Art(Art.icon("apple", p.itemColor), cell, cell, Modifier.rotate(p.rotation[k % 24].toFloat())) }
            }
        }
        "sized" -> { val sz = (114 * v.number).dp; Art(visualSvg(v), sz, sz) }
        else -> Art(visualSvg(v), 114.dp, 114.dp)
    }
}

@Composable
private fun Missing(w: Dp, h: Dp) = Box(
    Modifier.size(w, h).background(hex("#fffdf8"), RoundedCornerShape(14.dp)).border(2.5.dp, Gold, RoundedCornerShape(14.dp)),
    contentAlignment = Alignment.Center,
) { T("?", serif(34, hex("#c8963c"))) }

@Composable
private fun AppleItem(s: JardimState, idx: Int, size: Dp) {
    val p = s.q!!.presentation
    val on = idx in s.counted
    Box(
        Modifier.size(size).then(if (on) Modifier.background(hex("#e0eddc"), CircleShape).border(2.dp, hex("#78a36d"), CircleShape) else Modifier)
            .clickable { s.countItem(idx) }.padding(4.dp),
    ) { Art(Art.icon("apple", p.itemColor), size - 8.dp, size - 8.dp, Modifier.rotate(p.rotation[idx % 24].toFloat())) }
}

@Composable
private fun SceneView(s: JardimState, sc: Scene) {
    val q = s.q!!
    val center = Arrangement.spacedBy(6.dp, Alignment.CenterHorizontally)
    when (sc.type) {
        "color" -> Box(Modifier.size(120.dp).background(hex(sc.color), CircleShape))
        "shape" -> Art(Art.icon(sc.shape, sc.color), 140.dp, 140.dp)
        "sock" -> Art(Art.sock(sc.color, sc.pattern), 150.dp, 150.dp)
        "tower" -> Art(Art.tower(sc.items), 150.dp, 150.dp)
        "clock" -> Art(Art.clock(sc.count), 170.dp, 170.dp)
        "object-model" -> Box(Modifier.size(150.dp).background(Color.White, RoundedCornerShape(24.dp)).padding(14.dp)) { Art(Art.icon(sc.icon), 122.dp, 122.dp) }
        "look" -> T("Observe com atenção. Você consegue!", sans(16, Muted))
        "items" -> FlowRow(horizontalArrangement = Arrangement.spacedBy(q.presentation.itemGap.dp, Alignment.CenterHorizontally), verticalArrangement = Arrangement.spacedBy(6.dp)) {
            repeat(sc.count) { AppleItem(s, it, 64.dp) }
        }
        "sum" -> Row(verticalAlignment = Alignment.CenterVertically) {
            FlowRow(Modifier.width((min(sc.a, 5) * 52).dp), horizontalArrangement = Arrangement.spacedBy(6.dp), verticalArrangement = Arrangement.spacedBy(6.dp)) { repeat(sc.a) { AppleItem(s, it, 46.dp) } }
            T("+", serif(40), Modifier.padding(horizontal = 16.dp))
            FlowRow(Modifier.width((min(sc.b, 5) * 52).dp), horizontalArrangement = Arrangement.spacedBy(6.dp), verticalArrangement = Arrangement.spacedBy(6.dp)) { repeat(sc.b) { AppleItem(s, sc.a + it, 46.dp) } }
        }
        "sequence" -> Row(horizontalArrangement = center, verticalAlignment = Alignment.CenterVertically) {
            sc.items.forEach { Art(Art.icon(it, "#8b7bbd"), 84.dp, 84.dp) }; Missing(72.dp, 72.dp)
        }
        "numbers" -> Row(Modifier.height(120.dp), horizontalArrangement = Arrangement.spacedBy(12.dp), verticalAlignment = Alignment.Bottom) {
            sc.numbers.forEachIndexed { i, nn ->
                Box(Modifier.padding(bottom = (i * 12).dp)) {
                    if (nn == null) Missing(80.dp, 70.dp)
                    else Box(Modifier.size(80.dp, 70.dp).background(Color.White, RoundedCornerShape(14.dp)).border(2.dp, Line, RoundedCornerShape(14.dp)), contentAlignment = Alignment.Center) { T("$nn", serif(34)) }
                }
            }
        }
        "friends" -> Row { repeat(sc.count) { Art(Art.icon("bunny"), 80.dp, 78.dp) } }
        "color-sequence" -> Row(horizontalArrangement = Arrangement.spacedBy(16.dp), verticalAlignment = Alignment.CenterVertically) {
            sc.items.forEach { Box(Modifier.size(68.dp).background(hex(it), CircleShape)) }; Missing(68.dp, 68.dp)
        }
        "letter" -> Box(Modifier.size(260.dp, 120.dp).background(Color.White, RoundedCornerShape(24.dp)), contentAlignment = Alignment.Center) { T(sc.text, serif(64)) }
        "word-picture" -> Row(horizontalArrangement = Arrangement.spacedBy(20.dp), verticalAlignment = Alignment.CenterVertically) {
            Art(Art.icon(sc.icon), 130.dp, 130.dp); if (sc.word.isNotEmpty()) T(sc.word, serif(46))
        }
        "subtract" -> Row(horizontalArrangement = Arrangement.spacedBy(6.dp)) {
            repeat(sc.a) { i ->
                val gone = i >= sc.a - sc.b
                Box(Modifier.size(52.dp), contentAlignment = Alignment.Center) {
                    Art(Art.icon("apple"), 52.dp, 52.dp, Modifier.graphicsLayer { alpha = if (gone) 0.4f else 1f })
                    if (gone) Box(Modifier.size(58.dp, 4.dp).rotate(45f).background(hex("#c95a46"), RoundedCornerShape(2.dp)))
                }
            }
        }
        "equation" -> Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            listOf("${sc.a}", "+", null, "=", "${sc.count}").forEach { t ->
                if (t == null) Missing(72.dp, 72.dp) else Box(Modifier.size(72.dp), contentAlignment = Alignment.Center) { T(t, serif(40)) }
            }
        }
        "tenframe" -> Box(Modifier.background(Color.White, RoundedCornerShape(12.dp)).border(2.dp, hex("#92b4ad"), RoundedCornerShape(12.dp)).padding(6.dp)) {
            FlowRow(Modifier.width(280.dp)) {
                repeat(10) { i ->
                    Box(Modifier.size(56.dp).border(1.dp, Line), contentAlignment = Alignment.Center) {
                        if (i < sc.count) Box(Modifier.size(38.dp).background(hex("#e87961"), CircleShape))
                    }
                }
            }
        }
        "tens" -> Row(horizontalArrangement = Arrangement.spacedBy(14.dp), verticalAlignment = Alignment.Bottom) {
            repeat(sc.a) { Column { repeat(10) { Box(Modifier.size(13.dp).background(hex("#5799be"), RoundedCornerShape(2.dp)).border(0.5.dp, Color.White, RoundedCornerShape(2.dp))) } } }
            Spacer(Modifier.width(16.dp))
            Row(horizontalArrangement = Arrangement.spacedBy(4.dp)) { repeat(sc.b) { Box(Modifier.size(13.dp).background(hex("#e0b849"), RoundedCornerShape(2.dp))) } }
        }
        "story" -> Row(Modifier.widthIn(max = 720.dp), verticalAlignment = Alignment.CenterVertically) {
            Art(Art.icon(sc.icon), 96.dp, 96.dp); Spacer(Modifier.width(12.dp)); T(sc.text, sans(19), Modifier.weight(1f))
        }
    }
}

// ---------------------------------------------------------------- final e diálogos

@Composable
private fun FinishScreen(s: JardimState) {
    s.version
    val d = s.dailySolved(s.game)
    val scale = remember { Animatable(0.6f) }
    LaunchedEffect(Unit) { scale.animateTo(1f, spring(dampingRatio = Spring.DampingRatioMediumBouncy)) }
    Column(Modifier.fillMaxWidth().verticalScroll(rememberScrollState()), horizontalAlignment = Alignment.CenterHorizontally) {
        Header(s)
        Spacer(Modifier.height(30.dp))
        Art(Art.icon("flower"), 130.dp, 130.dp, Modifier.scale(scale.value))
        T("A CURIOSIDADE FLORESCEU!", eyebrowStyle)
        T(if (s.length == 1) "Uma nova descoberta!" else "${s.length} novas descobertas!", serif(40))
        T(if (d >= GOAL) "✓ Atividade feita hoje!" else "$d de 5 descobertas hoje", serif(18, Success))
        Spacer(Modifier.height(8.dp))
        T("Você observou, tentou e aprendeu.", sans(17, Muted))
        T("Que tal alongar o corpo e olhar o mundo lá fora?", sans(17, Muted))
        T("★ ".repeat(s.length), sans(22, Gold))
        Spacer(Modifier.height(14.dp))
        Btn("Voltar para o jardim", "primary") { s.home() }
        Spacer(Modifier.height(10.dp))
        T("Cada tentativa faz parte de aprender.", sans(14, Muted))
    }
}

@Composable
private fun ModalLayer(s: JardimState) {
    val gate = remember { listOf(6 + (0..3).random(), 6 + (0..3).random()) }
    var input by remember { mutableStateOf("") }
    var error by remember { mutableStateOf(false) }
    // Fundo escurecido simples (sem blur); consome os toques para não vazar à tela de trás.
    Box(Modifier.fillMaxSize().background(Color(0xA8233B36)).clickable(remember { MutableInteractionSource() }, null) {}, contentAlignment = Alignment.Center) {
        BoxWithConstraints {
            Column(
                Modifier.width(560.dp).heightIn(max = maxHeight * 0.9f).background(Paper, RoundedCornerShape(27.dp)).padding(30.dp).verticalScroll(rememberScrollState()),
                verticalArrangement = Arrangement.spacedBy(8.dp),
            ) {
                when (s.modal) {
                    "gate" -> {
                        T("UM MOMENTO PARA OS ADULTOS", eyebrowStyle)
                        T("Vamos cuidar do jardim?", serif(26))
                        T("Para abrir as configurações, resolva:", sans(16, Muted))
                        Row(verticalAlignment = Alignment.CenterVertically, horizontalArrangement = Arrangement.spacedBy(12.dp)) {
                            T("Quanto é ${gate[0]} × ${gate[1]}?", serif(22))
                            Box(Modifier.size(110.dp, 50.dp).background(Color.White, RoundedCornerShape(12.dp)).border(2.dp, Line, RoundedCornerShape(12.dp)), contentAlignment = Alignment.Center) { T(input, serif(26)) }
                        }
                        if (error) T("Tente mais uma vez.", sans(15, hex("#9c513c")))
                        FlowRow(Modifier.width(240.dp), horizontalArrangement = Arrangement.spacedBy(8.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
                            listOf("1", "2", "3", "4", "5", "6", "7", "8", "9", "⌫", "0").forEach { k ->
                                Box(Modifier.size(72.dp, 52.dp).background(Quiet, RoundedCornerShape(12.dp)).clickable {
                                    error = false
                                    if (k == "⌫") input = input.dropLast(1) else if (input.length < 3) input += k
                                }, contentAlignment = Alignment.Center) { T(k, serif(22)) }
                            }
                        }
                        Row(horizontalArrangement = Arrangement.spacedBy(10.dp)) {
                            Btn("Voltar") { s.setModal("") }
                            Btn("Entrar", "primary") { if (input.toIntOrNull() == gate[0] * gate[1]) s.setModal("parents") else error = true }
                        }
                    }
                    "reset" -> {
                        T("Apagar todas as descobertas?", serif(26))
                        T("Isso apaga o progresso das três faixas etárias neste dispositivo.", sans(16, Muted))
                        Row(horizontalArrangement = Arrangement.spacedBy(10.dp)) {
                            Btn("Cancelar") { s.setModal("parents") }
                            Btn("Sim, apagar", "danger") { s.reset(); s.setModal("parents") }
                        }
                    }
                    else -> {
                        T("ÁREA DOS ADULTOS", eyebrowStyle)
                        T("Aprender no próprio ritmo", serif(26))
                        T("As faixas são sugestões. Escolha a que combina com a criança e acompanhe as atividades, especialmente aos 2–3 anos.", sans(15, Muted))
                        T("Faixa etária", serif(18))
                        Row(horizontalArrangement = Arrangement.spacedBy(4.dp)) { GROUPS.forEach { g -> Btn(g.label, "pill", g.id == s.group) { s.setGroup(g.id) } } }
                        T("Narração em português", serif(18))
                        Btn(if (s.voice) "Ligada — toque para desligar" else "Desligada — toque para ligar") { s.toggleVoice() }
                        T("Usa a voz do Android. O primeiro uso pode exigir uma voz pt-BR já instalada. As atividades funcionam sem áudio.", sans(13, Muted))
                        T("Descobertas por atividade · ${GROUPS[s.level].label}", serif(18))
                        GAMES.forEach { g ->
                            Row(Modifier.fillMaxWidth()) { T(g.name, sans(14)); Spacer(Modifier.weight(1f)); T("${s.dailySolved(g.id)}/5 hoje · ${s.stat(g.id).solved} no total", sans(14, Muted)) }
                        }
                        T("Os dados ficam somente neste dispositivo. Sem conta, anúncios, compras ou acesso à internet. A pergunta de entrada é uma barreira simples para crianças pequenas.", sans(13, Muted))
                        Row(horizontalArrangement = Arrangement.spacedBy(10.dp)) {
                            Btn("Apagar progresso", "danger") { s.setModal("reset") }
                            Btn("Pronto", "primary") { s.home() }
                        }
                    }
                }
            }
        }
    }
}
