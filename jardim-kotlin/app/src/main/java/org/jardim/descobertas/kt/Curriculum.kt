// Port de curriculum.js + extra + toddler + variation.js para Kotlin.
// A ordem das chamadas ao RNG é a mesma do JS: a mesma semente gera a mesma
// pergunta (ParityTest confere 96/96 contra o JS original).
package org.jardim.descobertas.kt

import java.security.SecureRandom
import kotlin.math.max


private const val M32 = 0xFFFFFFFFL

/** mulberry32, igual a `Curriculum.random(seed)`; aritmética em Long mascarada para 32 bits. */
class Rng(seed: Long) {
    private var s = seed and M32
    fun next(): Double {
        s = (s + 0x6d2b79f5L) and M32
        val a = s
        var t = ((a xor (a ushr 15)) * (1L or a)) and M32
        t = ((t + (((t xor (t ushr 7)) * (61L or t)) and M32)) and M32) xor t
        return ((t xor (t ushr 14)) and M32).toDouble() / 4294967296.0
    }
    fun below(n: Int) = (next() * n).toInt()
    fun range(lo: Int, hi: Int) = lo + (next() * (hi - lo + 1)).toInt()
    fun <T> pick(xs: List<T>) = xs[below(xs.size)]
    fun <T> shuffle(xs: List<T>): List<T> {
        val a = xs.toMutableList()
        for (i in a.size - 1 downTo 1) {
            val j = (next() * (i + 1)).toInt()
            val t = a[i]; a[i] = a[j]; a[j] = t
        }
        return a
    }
}

/** FNV-1a sobre a 1ª unidade UTF-16 de cada code point (como o JS). */
fun hash(text: String): Long {
    var v = 2166136261L
    var i = 0
    while (i < text.length) {
        val c = text[i]
        v = v xor c.code.toLong()
        v = (v * 16777619L) and M32
        i += if (c.isHighSurrogate()) 2 else 1
    }
    return v
}

fun seedFor(day: String, group: String, game: String, round: Long, entry: Long) = hash("$day|$group|$game|$round|$entry")
fun entrySeed(): Long = SecureRandom().nextInt().toLong() and M32

data class Group(val id: String, val label: String, val name: String, val max: Int, val pairs: Int)
data class Game(val id: String, val name: String, val skill: String, val icon: String, val color: String, val min: Int, val category: String)

val GROUPS = listOf(
    Group("little", "2–3 anos", "Primeiros passos", 3, 2),
    Group("middle", "4–5 anos", "Mentes curiosas", 6, 3),
    Group("big", "6–8 anos", "Grandes ideias", 12, 4),
)
fun groupIndex(id: String) = max(0, GROUPS.indexOfFirst { it.id == id })

val GAMES = listOf(
    Game("colors", "Jardim das cores", "Reconhecer e associar", "flower", "pink", 0, "space"),
    Game("shapes", "Formas pelo caminho", "Percepção visual", "shapes", "blue", 0, "space"),
    Game("count", "Vamos contar?", "Números e quantidades", "count", "yellow", 0, "math"),
    Game("memory", "Pares da floresta", "Memória e atenção", "memory", "green", 0, "logic"),
    Game("pattern", "O que vem depois?", "Sequências e lógica", "pattern", "purple", 1, "logic"),
    Game("odd", "Um é diferente", "Observar e comparar", "odd", "peach", 0, "logic"),
    Game("sum", "Somar sorrisos", "Primeiras operações", "sum", "yellow", 1, "math"),
    Game("order", "Escada dos números", "Ordem e raciocínio", "order", "blue", 1, "math"),
    Game("compare", "Onde tem mais?", "Comparar quantidades", "count", "yellow", 0, "math"),
    Game("sizes", "Grande ou pequeno?", "Comparar tamanhos", "shapes", "blue", 0, "space"),
    Game("position", "Onde está a flor?", "Posição no espaço", "flower", "green", 0, "space"),
    Game("shadow", "De quem é a sombra?", "Contornos e associação", "shapes", "purple", 0, "space"),
    Game("sort", "Cada um no seu grupo", "Classificar e relacionar", "memory", "green", 0, "logic"),
    Game("letters", "Letras companheiras", "Maiúsculas e minúsculas", "letters", "pink", 1, "language"),
    Game("initial", "Começa com qual letra?", "Letra inicial das palavras", "letters", "peach", 1, "language"),
    Game("vowels", "Jardim das vogais", "Reconhecer as vogais", "letters", "pink", 1, "language"),
    Game("syllables", "Palavras em pedacinhos", "Contar sílabas", "letters", "purple", 2, "language"),
    Game("word", "Palavra e desenho", "Leitura de palavras", "letters", "peach", 2, "language"),
    Game("subtract", "Quantas ficaram?", "Subtração com imagens", "count", "yellow", 1, "math"),
    Game("missing", "O número escondido", "Completar uma adição", "sum", "yellow", 2, "math"),
    Game("tenframe", "Casinha dos dez", "Quantidades até dez", "count", "blue", 1, "math"),
    Game("tens", "Dez de cada vez", "Dezenas e unidades", "order", "yellow", 2, "math"),
    Game("clock", "Que horas são?", "Horas inteiras no relógio", "clock", "blue", 2, "math"),
    Game("story", "Histórias do jardim", "Escutar e compreender", "book", "green", 1, "language"),
    Game("twins", "Encontre o igual", "Associar objetos iguais", "memory", "green", 0, "logic"),
    Game("inside", "Dentro ou fora?", "Explorar posições", "container", "blue", 0, "space"),
    Game("fill", "Cheio ou vazio?", "Comparar recipientes", "cup", "yellow", 0, "space"),
    Game("tower", "Torres coloridas", "Copiar um modelo", "tower", "purple", 0, "logic"),
    Game("socks", "Meias companheiras", "Combinar cores e detalhes", "socks", "pink", 0, "logic"),
    Game("oneeach", "Um para cada amigo", "Relacionar um a um", "count", "peach", 0, "math"),
    Game("vocabulary", "Quem foi chamado?", "Ouvir e reconhecer", "book", "green", 0, "language"),
    Game("colorpath", "Caminho das cores", "Continuar uma repetição", "colorpath", "pink", 0, "logic"),
)
fun gameById(id: String) = GAMES.firstOrNull { it.id == id } ?: GAMES[0]

val CATEGORIES = listOf("all" to "Todas", "math" to "Matemática", "logic" to "Lógica", "space" to "Formas e espaço", "language" to "Letras e histórias")

private class Clr(val id: String, val name: String, val hex: String)
private val COLORS = listOf(Clr("coral", "vermelha", "#eb6d58"), Clr("blue", "azul", "#589cd4"), Clr("yellow", "amarela", "#efbd46"), Clr("green", "verde", "#60a786"))
val SHAPES = listOf("circle", "triangle", "square", "star")
fun shapeName(s: String) = when (s) { "circle" -> "círculo"; "triangle" -> "triângulo"; "square" -> "quadrado"; else -> "estrela" }

private class Word(val word: String, val icon: String, val syl: List<String>)
private val WORDS = listOf(
    Word("SOL", "sun", listOf("SOL")), Word("FLOR", "flower", listOf("FLOR")), Word("MAÇÃ", "apple", listOf("MA", "ÇÃ")),
    Word("GATO", "cat", listOf("GA", "TO")), Word("BOLA", "ball", listOf("BO", "LA")), Word("LUA", "moon", listOf("LU", "A")),
    Word("CASA", "house", listOf("CA", "SA")), Word("PEIXE", "fish", listOf("PEI", "XE")),
    Word("COELHO", "bunny", listOf("CO", "E", "LHO")), Word("PÁSSARO", "bird", listOf("PÁS", "SA", "RO")),
)

private class Story(val text: String, val question: String, val icon: String, val answer: String, val choices: List<String>, val labels: List<String>)
private val STORIES = listOf(
    Story("Nina, a coelha, encontrou uma maçã no jardim. Ela dividiu a maçã com seu amigo passarinho.", "Qual fruta Nina dividiu?", "bunny", "apple", listOf("apple", "flower", "ball"), listOf("maçã", "flor", "bola")),
    Story("O passarinho Tito recolheu uma flor amarela. Depois, levou a flor para perto de sua casa.", "O que Tito recolheu?", "bird", "flower", listOf("flower", "apple", "fish"), listOf("flor", "maçã", "peixe")),
    Story("O gato Bento brincava com uma bola. Quando o sol apareceu, Bento levou a bola para o jardim.", "Com o que Bento brincava?", "cat", "ball", listOf("ball", "apple", "flower"), listOf("bola", "maçã", "flor")),
    Story("A lua apareceu no céu. Nina, a coelha, guardou seus brinquedos e foi descansar.", "O que apareceu no céu?", "bunny", "moon", listOf("moon", "sun", "flower"), listOf("lua", "sol", "flor")),
    Story("Tito viu um peixe nadando no lago. O peixe passou por uma pedra e mergulhou devagar.", "Qual animal estava no lago?", "bird", "fish", listOf("fish", "cat", "bunny"), listOf("peixe", "gato", "coelho")),
    Story("Bento encontrou Nina em frente à casa. O gato e a coelha entraram juntos para brincar.", "Onde os amigos entraram?", "cat", "house", listOf("house", "flower", "apple"), listOf("casa", "flor", "maçã")),
)

private val OBJECTS = listOf(
    "apple" to "a maçã", "flower" to "a flor", "bird" to "o passarinho", "bunny" to "o coelho", "cat" to "o gato",
    "fish" to "o peixe", "ball" to "a bola", "moon" to "a lua", "house" to "a casa", "sun" to "o sol",
)

// ---------------------------------------------------------------- modelo

class Visual(val type: String, var kind: String = "", var color: String = "", val colors: List<String> = emptyList(), var number: Double = 0.0, val text: String = "")
class Option(val id: String, val label: String, val visual: Visual)
class Scene(
    val type: String, var color: String = "", val shape: String = "", val icon: String = "", val text: String = "", val word: String = "",
    val pattern: String = "", val items: List<String> = emptyList(), val numbers: List<Int?> = emptyList(), val a: Int = 0, val b: Int = 0, val count: Int = 0,
)
class Card(val kind: String, val pair: Int)
class Presentation {
    var surface = "#fffaf0"; var itemColor = "#e87961"; var radius = 20; var itemGap = 4
    var rotation: List<Int> = List(24) { 0 }
}
class Question(val game: String) {
    var prompt = ""; var hint = ""; var story = ""; var memory = false
    var options: List<Option> = emptyList(); var answer = 0
    var cards: List<Card> = emptyList(); var scene: Scene? = null
    val presentation = Presentation()
    fun setOptions(r: Rng, correct: String, items: List<Option>) {
        options = r.shuffle(items)
        answer = max(0, options.indexOfFirst { it.id == correct })
    }
}

private fun icon(kind: String, color: String = "") = Visual("icon", kind = kind, color = color)
private fun num(n: Int) = Option("$n", "$n", Visual("number", number = n.toDouble()))
private fun rangeEx(lo: Int, hi: Int, except: Int) = (lo..hi).filter { it != except }
private fun <T> List<T>.takeN(n: Int) = take(n)

// ---------------------------------------------------------------- jogos

private fun toddler(q: Question, game: String, level: Int, seed: Long): Boolean {
    val r = Rng(seed)
    val choices = if (level == 0) 2 else 3
    when (game) {
        "twins", "vocabulary" -> {
            val ti = r.below(OBJECTS.size)
            val (tid, tname) = OBJECTS[ti]
            val twins = game == "twins"
            q.prompt = if (twins) "Qual desenho é igual ao modelo?" else "Encontre $tname."
            q.hint = if (twins) "Olhe o desenho grande e procure outro igual." else "Vamos procurar $tname juntos."
            q.scene = if (twins) Scene("object-model", icon = tid) else Scene("look")
            val idx = listOf(ti) + r.shuffle(OBJECTS.indices.filter { it != ti }).takeN(choices - 1)
            q.setOptions(r, tid, idx.map { Option(OBJECTS[it].first, OBJECTS[it].second, icon(OBJECTS[it].first)) })
        }
        "inside" -> {
            val target = r.pick(listOf("inside", "outside"))
            q.prompt = if (target == "inside") "Em qual desenho a flor está dentro da caixa?" else "Em qual desenho a flor está fora da caixa?"
            q.hint = if (target == "inside") "Procure a flor no espaço de dentro da caixa." else "Procure a flor ao lado de fora da caixa."
            q.scene = Scene("look")
            q.setOptions(r, target, listOf("inside", "outside").map { Option(it, if (it == "inside") "Flor dentro da caixa" else "Flor fora da caixa", Visual("container", kind = it)) })
        }
        "fill" -> {
            val target = r.pick(listOf(0, 1))
            q.prompt = if (target == 1) "Qual copo está cheio?" else "Qual copo está vazio?"
            q.hint = if (target == 1) "Procure o copo com água até perto da borda." else "Procure o copo sem água."
            q.scene = Scene("look")
            val amounts = if (level == 0) listOf(0.0, 1.0) else listOf(0.0, 0.5, 1.0)
            q.setOptions(r, "$target", amounts.map {
                Option(if (it == 0.5) "0.5" else "${it.toInt()}", if (it == 1.0) "Copo cheio" else if (it == 0.0) "Copo vazio" else "Copo pela metade", Visual("cup", number = it))
            })
        }
        "tower" -> {
            val palette = r.shuffle(listOf(0, 1, 2, 3)).takeN(if (level == 0) 2 else 3).map { COLORS[it].hex }
            q.prompt = "Qual torre é igual à torre do modelo?"
            q.hint = "Olhe a cor dos blocos de baixo para cima."
            q.scene = Scene("tower", items = palette)
            val variants = mutableListOf(palette, palette.reversed())
            if (choices == 3) variants += listOf(palette[1], palette[0], palette[2])
            q.setOptions(r, palette.joinToString(":"), variants.mapIndexed { i, c -> Option(c.joinToString(":"), "Torre ${i + 1}", Visual("tower", colors = c)) })
        }
        "socks" -> {
            val color = COLORS[r.below(4)].hex
            val other = r.pick(COLORS.map { it.hex }.filter { it != color })
            val pattern = r.pick(listOf("dots", "stripes"))
            q.prompt = "Qual meia faz par com esta?"
            q.hint = "Procure a mesma cor e o mesmo desenho na meia."
            q.scene = Scene("sock", color = color, pattern = pattern)
            val variants = mutableListOf(color to pattern, other to pattern)
            if (choices == 3) variants += color to (if (pattern == "dots") "stripes" else "dots")
            q.setOptions(r, "$color$pattern", variants.mapIndexed { i, (c, p) -> Option("$c$p", "Meia ${i + 1}", Visual("sock", color = c, kind = p)) })
        }
        "oneeach" -> {
            val n = 1 + r.below(if (level == 0) 3 else 5)
            q.prompt = "Qual grupo tem uma maçã para cada coelho?"
            q.hint = "Dê uma maçã para um coelho. Depois, uma para o próximo."
            q.scene = Scene("friends", count = n)
            val ns = listOf(n) + r.shuffle(rangeEx(1, if (level == 0) 3 else 6, n)).takeN(choices - 1)
            q.setOptions(r, "$n", ns.map { Option("$it", "Grupo com $it maçãs", Visual("quantity", number = it.toDouble())) })
        }
        "colorpath" -> {
            val ab = r.shuffle(listOf(0, 1, 2, 3)).takeN(2)
            val a = COLORS[ab[0]]; val b = COLORS[ab[1]]
            q.prompt = "Qual cor continua o caminho?"
            q.hint = "As cores aparecem uma de cada vez e depois se repetem."
            q.scene = Scene("color-sequence", items = listOf(a.hex, b.hex, a.hex))
            q.setOptions(r, b.id, listOf(a, b).map { Option(it.id, "Cor ${it.name}", Visual("swatch", color = it.hex)) })
        }
        else -> return false
    }
    return true
}

private fun extra(q: Question, game: String, level: Int, seed: Long): Boolean {
    val r = Rng(seed)
    val tk = if (level > 0) 3 else 2
    fun numbers(answer: Int, lo: Int, hi: Int) = q.setOptions(r, "$answer", listOf(num(answer)) + r.shuffle(rangeEx(lo, hi, answer)).takeN(tk).map { num(it) })
    when (game) {
        "compare" -> {
            val counts = r.shuffle(rangeEx(1, listOf(3, 6, 10)[level], -1)).takeN(3)
            val want = r.pick(listOf("more", "less"))
            val target = if (want == "more") counts.max() else counts.min()
            q.prompt = if (want == "more") "Qual grupo tem mais maçãs?" else "Qual grupo tem menos maçãs?"
            q.hint = "Conte as maçãs de cada grupo e compare."
            q.scene = Scene("look")
            q.setOptions(r, "$target", counts.map { Option("$it", "Grupo com $it maçãs", Visual("quantity", number = it.toDouble())) })
        }
        "sizes" -> {
            val shape = r.pick(SHAPES)
            val want = r.pick(listOf("big", "small"))
            val sizes = r.shuffle(listOf(0.46, 0.7, 1.0))
            q.prompt = if (want == "big") "Qual desenho é o maior?" else "Qual desenho é o menor?"
            q.hint = "As formas são iguais. Compare o espaço que cada uma ocupa."
            q.scene = Scene("look")
            q.setOptions(r, if (want == "big") "1" else "0.46", sizes.map {
                Option(if (it == 1.0) "1" else "$it", if (it == 1.0) "Desenho grande" else if (it == 0.46) "Desenho pequeno" else "Desenho médio",
                    Visual("sized", kind = shape, number = it, color = "#7aaa8b"))
            })
        }
        "position" -> {
            val dirs = if (level == 0) listOf("above", "below") else listOf("above", "below", "left", "right")
            val target = r.pick(dirs)
            fun name(d: String) = when (d) { "above" -> "acima"; "below" -> "abaixo"; "left" -> "à esquerda"; else -> "à direita" }
            q.prompt = "Em qual desenho a flor está ${name(target)} da caixa?"
            q.hint = when (target) {
                "left" -> "A esquerda fica deste lado: ←."; "right" -> "A direita fica deste lado: →."
                "above" -> "Procure a flor na parte de cima da caixa."; else -> "Procure a flor na parte de baixo da caixa."
            }
            q.scene = Scene("look")
            q.setOptions(r, target, dirs.map { Option(it, "Flor ${name(it)} da caixa", Visual("position", kind = it)) })
        }
        "shadow" -> {
            val pool = SHAPES.take(if (level > 0) 4 else 3)
            val target = r.pick(pool)
            q.prompt = "Qual desenho combina com esta sombra?"
            q.hint = "Observe o contorno, sem se preocupar com a cor."
            q.scene = Scene("shape", shape = target, color = "#344c47")
            q.setOptions(r, target, pool.mapIndexed { i, s -> Option(s, shapeName(s), icon(s, COLORS[i].hex)) })
        }
        "sort" -> {
            val animals = listOf("bird", "bunny", "cat", "fish"); val plants = listOf("apple", "flower", "leaf")
            val isAnimals = r.below(2) == 0
            val target = r.pick(if (isAnimals) animals else plants)
            val label = mapOf("bird" to "passarinho", "bunny" to "coelho", "cat" to "gato", "fish" to "peixe", "apple" to "maçã", "flower" to "flor", "leaf" to "folha")
            q.prompt = "Qual destes desenhos é ${if (isAnimals) "um animal" else "uma planta ou parte de uma planta"}?"
            q.hint = if (isAnimals) "Procure um ser vivo que é um animal." else "Folhas, flores e frutos fazem parte de plantas."
            q.scene = Scene("look")
            q.setOptions(r, target, (listOf(target) + r.shuffle(if (isAnimals) plants else animals).takeN(2)).map { Option(it, label.getValue(it), icon(it)) })
        }
        "letters" -> {
            val alphabet = "ABCDEFGHJKLMNPRSTUVZ".map { "$it" }
            val t = r.pick(alphabet); val lo = t.lowercase()
            q.prompt = "Qual é a letra minúscula que combina com $t?"
            q.hint = "A letra $t também pode ser escrita assim: $lo."
            q.scene = Scene("letter", text = t)
            val cs = listOf(t) + r.shuffle(alphabet.filter { it != t }).takeN(3)
            q.setOptions(r, t, cs.map { Option(it, "Letra ${it.lowercase()}", Visual("text", text = it.lowercase())) })
        }
        "initial" -> {
            val w = WORDS[r.below(WORDS.size)]
            val first = w.word.substring(0, 1)
            q.prompt = "Qual é a primeira letra de ${w.word.lowercase()}?"
            q.hint = "Observe a primeira letra: $first."
            q.scene = Scene("word-picture", word = w.word, icon = w.icon)
            val cs = listOf(first) + r.shuffle("ABCDEFGHILMNOPRSTUV".map { "$it" }.filter { it != first }).takeN(3)
            q.setOptions(r, first, cs.map { Option(it, "Letra $it", Visual("text", text = it)) })
        }
        "vowels" -> {
            val v = r.pick("AEIOU".map { "$it" })
            val others = r.shuffle("BCDFGLMNPRSTV".map { "$it" }).takeN(3)
            q.prompt = "Qual destas letras é uma vogal?"
            q.hint = "As vogais são A, E, I, O e U."
            q.scene = Scene("letter", text = "A E I O U")
            q.setOptions(r, v, (listOf(v) + others).map { Option(it, "Letra $it", Visual("text", text = it)) })
        }
        "syllables" -> {
            val w = WORDS[r.below(WORDS.size)]
            q.prompt = "Quantos pedacinhos tem a palavra ${w.word.lowercase()}?"
            q.hint = "Fale devagar: ${w.syl.joinToString(", ")}. Conte uma batida para cada pedacinho."
            q.scene = Scene("word-picture", word = w.word, icon = w.icon)
            numbers(w.syl.size, 1, 5)
        }
        "word" -> {
            val ti = r.below(WORDS.size); val w = WORDS[ti]
            q.prompt = "Qual palavra combina com o desenho?"
            q.hint = "O desenho é ${w.word.lowercase()}. Procure essa palavra."
            q.scene = Scene("word-picture", icon = w.icon)
            val idx = listOf(ti) + r.shuffle(WORDS.indices.filter { it != ti }).takeN(3)
            q.setOptions(r, w.word, idx.map { Option(WORDS[it].word, WORDS[it].word, Visual("text", text = WORDS[it].word)) })
        }
        "subtract" -> {
            val a = r.range(2, if (level == 2) 12 else 5); val b = r.range(1, a)
            q.prompt = "Havia $a maçãs. Tiramos $b. Quantas ficaram?"
            q.hint = "As maçãs riscadas saíram do grupo. Conte as que ficaram."
            q.scene = Scene("subtract", a = a, b = b)
            numbers(a - b, 0, if (level == 2) 15 else 7)
        }
        "missing" -> {
            val hi = if (level == 2) 9 else 3
            val a = r.range(1, hi); val b = r.range(1, hi)
            q.prompt = "Qual número está escondido na adição?"
            q.hint = "Quanto falta para ir de $a até ${a + b}?"
            q.scene = Scene("equation", a = a, count = a + b)
            numbers(b, 0, if (level == 2) 12 else 6)
        }
        "tenframe" -> {
            val n = r.range(1, 10)
            q.prompt = "Quantos lugares estão ocupados na casinha?"
            q.hint = "A primeira linha tem cinco lugares. Conte os círculos preenchidos."
            q.scene = Scene("tenframe", count = n)
            numbers(n, 0, 12)
        }
        "tens" -> {
            val tens = r.range(1, if (level == 2) 4 else 2); val units = r.range(0, 9)
            q.prompt = "Qual número estas dezenas e unidades formam?"
            q.hint = "Cada barra tem dez quadradinhos. Há $tens dezenas e $units unidades."
            q.scene = Scene("tens", a = tens, b = units)
            numbers(tens * 10 + units, 10, 59)
        }
        "clock" -> {
            val hour = r.range(1, 12)
            q.prompt = "Que horas o relógio está marcando?"
            q.hint = "O ponteiro grande está no doze. O ponteiro pequeno mostra a hora."
            q.scene = Scene("clock", count = hour)
            numbers(hour, 1, 12)
        }
        "story" -> {
            val s = STORIES[r.below(STORIES.size)]
            q.prompt = s.question; q.story = s.text
            q.hint = "Ouça a história outra vez e observe o que aconteceu."
            q.scene = Scene("story", text = s.text, icon = s.icon)
            q.setOptions(r, s.answer, (0 until 3).map { Option(s.choices[it], s.labels[it], icon(s.choices[it])) })
        }
        else -> return false
    }
    return true
}

private fun base(game: String, group: String, seed: Long): Question {
    val r = Rng(seed)
    val level = groupIndex(group)
    val g = GROUPS[level]
    val q = Question(game)
    when (game) {
        "colors" -> {
            val ti = r.below(4); val t = COLORS[ti]
            q.prompt = "Encontre a flor de cor ${t.name}."
            q.hint = "Observe a cor das pétalas."
            q.scene = Scene("color", color = t.hex)
            val idx = listOf(ti) + r.shuffle((0 until 4).filter { it != ti }).takeN(if (level > 0) 3 else 2)
            q.setOptions(r, t.id, idx.map { Option(COLORS[it].id, COLORS[it].name, icon("flower", COLORS[it].hex)) })
        }
        "shapes" -> {
            val t = r.pick(SHAPES.take(if (level > 0) 4 else 3))
            q.prompt = "Qual forma é igual à do desenho?"
            q.hint = "Procure o ${shapeName(t)}."
            q.scene = Scene("shape", shape = t, color = "#559d8b")
            val list = r.shuffle(SHAPES.filter { it != t }).takeN(if (level > 0) 3 else 2) + t
            q.setOptions(r, t, list.map { Option(it, shapeName(it), icon(it, "#e9a94b")) })
        }
        "count" -> {
            val n = r.range(1, g.max)
            q.prompt = "Quantas maçãs você consegue contar?"
            q.hint = "Toque em cada maçã e conte devagar."
            q.scene = Scene("items", count = n)
            q.setOptions(r, "$n", listOf(num(n)) + r.shuffle(rangeEx(1, g.max + 2, n)).takeN(if (level > 0) 3 else 2).map { num(it) })
        }
        "memory" -> {
            q.memory = true
            q.prompt = "Encontre os pares que são iguais."
            q.hint = "Vire duas cartas. Lembre onde cada desenho está."
            val kinds = r.shuffle(listOf("apple", "bird", "flower", "sun")).takeN(g.pairs)
            q.cards = r.shuffle(kinds.flatMapIndexed { p, k -> listOf(Card(k, p), Card(k, p)) })
        }
        "pattern" -> {
            val a = r.pick(SHAPES); val b = r.pick(SHAPES.filter { it != a })
            val seq = if (level == 2) r.pick(listOf(listOf(a, a, b, a, a, b), listOf(a, b, b, a, b, b), listOf(a, b, a, b, a, b))) else listOf(a, b, a, b)
            q.prompt = "Qual desenho completa a sequência?"
            q.hint = "Diga os nomes das formas e procure a repetição."
            q.scene = Scene("sequence", items = seq.dropLast(1))
            q.setOptions(r, seq.last(), SHAPES.map { Option(it, shapeName(it), icon(it, "#8b7bbd")) })
        }
        "odd" -> {
            val normal = r.pick(SHAPES); val other = r.pick(SHAPES.filter { it != normal })
            val size = if (level > 0) 4 else 3
            val index = r.range(0, size - 1)
            q.prompt = "Qual desenho tem uma forma diferente?"
            q.hint = "A cor é igual. Compare o contorno dos desenhos."
            q.scene = Scene("look")
            q.options = (0 until size).map { val s = if (it == index) other else normal; Option("$it", "Desenho ${it + 1}: ${shapeName(s)}", icon(s, "#599b84")) }
            q.answer = index
        }
        "sum" -> {
            val hi = if (level == 2) 9 else 3
            val a = r.range(1, hi); val b = r.range(1, hi); val n = a + b
            q.prompt = "Juntando os dois grupos, quantas maçãs temos?"
            q.hint = "Conte as maçãs de um grupo e continue no outro."
            q.scene = Scene("sum", a = a, b = b)
            q.setOptions(r, "$n", listOf(num(n)) + r.shuffle(rangeEx(1, max(n + 3, 10), n)).takeN(3).map { num(it) })
        }
        "order" -> {
            val step = if (level == 2) r.pick(listOf(1, 2, 5)) else 1
            val start = r.range(1, if (level == 2) 10 else 4)
            val index = r.range(1, 3)
            val seq = (0 until 4).map { start + it * step }
            val n = seq[index]
            q.prompt = "Qual número falta na escada?"
            q.hint = if (step == 1) "Conte os números em ordem." else "Conte de $step em $step."
            q.scene = Scene("numbers", numbers = seq.mapIndexed { i, v -> if (i == index) null else v })
            q.setOptions(r, "$n", listOf(num(n)) + r.shuffle(rangeEx(1, seq[3] + 3, n)).takeN(3).map { num(it) })
        }
        else -> if (!toddler(q, game, level, seed) && !extra(q, game, level, seed)) q.prompt = "Atividade desconhecida"
    }
    return q
}

private fun vary(q: Question, seed: Long): Question {
    val r = Rng(seed xor 0xa4b1c987L)
    val palette = listOf("#e97e68", "#5799be", "#e0b849", "#68a585", "#9b86bd")
    val p = q.presentation
    p.surface = r.pick(listOf("#fffaf0", "#f2f7ed", "#eff6f7", "#faf1ed", "#f5f0f8"))
    p.radius = 20 + r.below(10)
    p.itemColor = r.pick(listOf("#e87961", "#83ad68", "#e6b85c"))
    p.itemGap = 4 + r.below(5)
    p.rotation = List(24) { r.below(17) - 8 }
    if (q.game == "colors") {
        for (o in q.options) o.visual.color = r.pick(when (o.id) {
            "coral" -> listOf("#e46b5b", "#df6357", "#e67a69"); "blue" -> listOf("#548fc5", "#5b9cce", "#478bbb")
            "yellow" -> listOf("#edc14c", "#e6b73e", "#efc656"); else -> listOf("#60a277", "#6caa80", "#56976c")
        })
        q.scene?.color = q.options[q.answer].visual.color
    }
    val shared = r.pick(palette)
    for (o in q.options) {
        val v = o.visual
        if (v.type == "icon" && v.kind in SHAPES) v.color = if (q.game == "odd") shared else r.pick(palette)
        else if (v.type == "sized") {
            val b = v.number
            v.number = if (b < 0.5) 0.4 + r.next() * 0.08 else if (b < 0.8) 0.63 + r.next() * 0.08 else 0.91 + r.next() * 0.08
            v.color = shared
        }
    }
    if (q.game != "shadow" && q.scene?.type == "shape") q.scene!!.color = r.pick(palette)
    return q
}

fun generate(game: String, group: String, seed: Long) = vary(base(game, group, seed), seed)
fun catalog(level: Int, seed: Long) = Rng(seed).shuffle(GAMES.filter { it.min <= level })
