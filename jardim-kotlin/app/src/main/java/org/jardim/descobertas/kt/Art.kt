// Port de art.js: os mesmos desenhos vetoriais, gerados como SVG.
package org.jardim.descobertas.kt

import java.util.Locale
import kotlin.math.PI
import kotlin.math.cos
import kotlin.math.sin

object Art {
    private fun svg(body: String, vb: String = "0 0 120 120") = """<svg xmlns="http://www.w3.org/2000/svg" viewBox="$vb">$body</svg>"""
    private fun n(v: Double) = String.format(Locale.US, "%.2f", v)

    private const val EYES =
        """<circle cx="48" cy="59" r="3" fill="#283e3b"/><circle cx="72" cy="59" r="3" fill="#283e3b"/><path d="M53 71q7 7 14 0" fill="none" stroke="#283e3b" stroke-width="3" stroke-linecap="round"/>"""

    fun body(kind: String, color: String = ""): String {
        val c = color.ifEmpty { "#eb6d58" }
        return when (kind) {
            "circle" -> """<circle cx="60" cy="60" r="38" fill="$c"/>"""
            "square" -> """<rect x="23" y="23" width="74" height="74" rx="9" fill="$c"/>"""
            "triangle" -> """<path d="M60 18 105 96H15Z" fill="$c" stroke-linejoin="round"/>"""
            "star" -> """<path d="m60 12 14 30 33 5-24 24 6 34-29-16-29 16 6-34-24-24 33-5Z" fill="$c"/>"""
            "apple" -> """<path d="M60 28q-3-18 16-22-1 19-16 22" fill="#5e9d70"/><path d="M60 34C9 8 13 84 40 101q12 8 20 0 16 10 29-5 32-48 6-64-17-10-35 2" fill="$c"/><path d="M37 45q-9 8-8 20" stroke="#ffc4ad" stroke-width="7" stroke-linecap="round" fill="none"/>"""
            "flower" -> """<path d="M60 60v53m0-19q-32-21-32-3 14 20 32 9m0-2q29-27 32-7-12 19-32 12" fill="#659976" stroke="#659976" stroke-width="5"/><g fill="$c"><ellipse cx="60" cy="34" rx="17" ry="22"/><ellipse cx="80" cy="51" rx="22" ry="17"/><ellipse cx="72" cy="72" rx="17" ry="22"/><ellipse cx="47" cy="72" rx="17" ry="22"/><ellipse cx="37" cy="51" rx="22" ry="17"/></g><circle cx="60" cy="54" r="19" fill="#ffdf78"/><circle cx="54" cy="52" r="2" fill="#33493a"/><circle cx="66" cy="52" r="2" fill="#33493a"/><path d="M56 60q4 4 8 0" fill="none" stroke="#33493a" stroke-width="2"/>"""
            "bird" -> """<ellipse cx="57" cy="65" rx="37" ry="32" fill="#83b7cb"/><circle cx="70" cy="41" r="24" fill="#83b7cb"/><path d="m93 42 20 9-20 9" fill="#e9ad46"/><path d="M17 66 4 42 33 56" fill="#6093ae"/><path d="M31 66q24-20 36 3-17 28-36-3" fill="#aed6df"/><circle cx="77" cy="38" r="3" fill="#283e3b"/><path d="m46 95-4 15m26-15 4 15" stroke="#c68e49" stroke-width="5"/>"""
            "sun" -> {
                val rays = (0 until 8).joinToString("") { """<path d="M60 9v9" transform="rotate(${it * 45} 60 60)"/>""" }
                """<g stroke="#efbd46" stroke-width="6" stroke-linecap="round">$rays</g><circle cx="60" cy="60" r="32" fill="#f4ca61"/>$EYES"""
            }
            "bunny" -> """<ellipse cx="43" cy="29" rx="13" ry="26" fill="#f4e9d1" transform="rotate(-14 43 29)"/><ellipse cx="77" cy="29" rx="13" ry="26" fill="#f4e9d1" transform="rotate(14 77 29)"/><ellipse cx="43" cy="27" rx="6" ry="17" fill="#edb8a6"/><ellipse cx="77" cy="27" rx="6" ry="17" fill="#edb8a6"/><ellipse cx="60" cy="101" rx="34" ry="25" fill="#e2d4b8"/><ellipse cx="60" cy="67" rx="38" ry="34" fill="#f4e9d1"/>$EYES<ellipse cx="38" cy="70" rx="7" ry="4" fill="#edb8a6"/><ellipse cx="82" cy="70" rx="7" ry="4" fill="#edb8a6"/>"""
            "cat" -> """<path d="M27 46 20 14 49 32M71 32 100 14 94 49" fill="#d7a56e"/><ellipse cx="60" cy="65" rx="43" ry="37" fill="#e7bc87"/>$EYES<path d="m23 64 21 4m-22 9 22-3m53-10-21 4m22 9-22-3" stroke="#8d6c4f" stroke-width="2"/><path d="m55 63 5 5 5-5" fill="#ce8e88"/>"""
            "fish" -> """<path d="M82 60 112 31v58Z" fill="#d9a75d"/><ellipse cx="52" cy="60" rx="40" ry="28" fill="#efbe6d"/><path d="M53 37 64 17 79 43M54 82 69 102 80 76" fill="#d9a75d"/><circle cx="30" cy="54" r="4" fill="#294b43"/><path d="M16 66q8 6 14 0" stroke="#294b43" stroke-width="2" fill="none"/>"""
            "ball" -> """<circle cx="60" cy="60" r="42" fill="#7fb8ca"/><path d="M32 28q37 28 55 59M20 68q33-32 80-22" fill="none" stroke="#e6f1ed" stroke-width="9"/><path d="M60 18q-13 37-10 84" stroke="#d3e9de" stroke-width="5" fill="none"/>"""
            "moon" -> """<path d="M78 14a44 44 0 1 0 26 67Q45 92 78 14" fill="#edc976"/><circle cx="40" cy="74" r="4" fill="#d8b263"/><circle cx="45" cy="44" r="6" fill="#d8b263"/>"""
            "house" -> """<path d="M23 49h74v58H23Z" fill="#e9c69b"/><path d="M10 52 60 9l50 43Z" fill="#c77e69"/><rect x="46" y="70" width="26" height="37" rx="3" fill="#73a493"/><rect x="30" y="59" width="14" height="15" rx="2" fill="#faf4da"/><rect x="77" y="59" width="14" height="15" rx="2" fill="#faf4da"/>"""
            "book" -> """<path d="M60 34Q32 16 10 28v69q25-10 50 6 25-16 50-6V28Q87 16 60 34" fill="#f8e4b5" stroke="#bc9a65" stroke-width="4"/><path d="M60 34v69M24 43l23 5m-23 8 23 5m-23 8 23 5m27-26 23-5m-23 18 23-5m-23 18 23-5" fill="none" stroke="#bc9a65" stroke-width="3"/>"""
            "letters" -> """<text x="10" y="84" font-family="serif" font-weight="bold" font-size="75" fill="#8e729e">Aa</text>"""
            "clock" -> clockBody(3)
            "socks" -> sockBody("#83b7cb", "stripes")
            "cup" -> cupBody(1.0)
            "tower" -> towerBody(listOf("#5799be", "#68a585"))
            "container" -> containerBody("inside")
            "colorpath" -> """<circle cx="25" cy="60" r="20" fill="#e87961"/><circle cx="60" cy="60" r="20" fill="#5799be"/><circle cx="95" cy="60" r="20" fill="#e87961"/>"""
            "leaf" -> """<path d="M60 103V45" stroke="#347665" stroke-width="7" stroke-linecap="round"/><path d="M59 70Q8 72 15 25q46-8 44 45" fill="#7cb78f"/><path d="M60 51q-5-43 43-41 7 43-43 41" fill="#47977e"/>"""
            else -> body("star", "#efbd46")
        }
    }

    fun icon(kind: String, color: String = "") = svg(body(kind, color))

    private fun sockBody(color: String, pattern: String): String {
        val marks = if (pattern == "dots") """<circle cx="61" cy="52" r="4"/><circle cx="77" cy="66" r="4"/><circle cx="61" cy="81" r="4"/>"""
        else """<path d="M48 48h37M48 64h37M48 80h32" stroke="#fff7de" stroke-width="6"/>"""
        return """<path d="M45 15h42v64q0 28-30 29H30q-23 0-15-19 3-8 17-10l13-2Z" fill="$color" stroke="#607f80" stroke-width="2"/><path d="M45 15h42v14H45Z" fill="#fff7de"/><g fill="#fff7de">$marks</g>"""
    }
    fun sock(color: String, pattern: String) = svg(sockBody(color, pattern))

    private fun cupBody(amount: Double): String {
        val top = 91.0 - amount * 64.0
        val water = if (amount > 0) """<path d="M${n(31 + (top - 17) * 7 / 84)} ${n(top)}h${n(58 - (top - 17) * 14 / 84)}L80 98H40Z" fill="#8fc5db"/>""" else ""
        return """<path d="M31 17h58l-7 84H38Z" fill="#f3f7f2" stroke="#709a97" stroke-width="4"/>$water<path d="M46 31v25" stroke="#fff" stroke-width="5" stroke-linecap="round"/>"""
    }
    fun cup(amount: Double) = svg(cupBody(amount))

    private fun miniFlower(x: Double, y: Double, size: Double) = """<g transform="translate(${n(x)} ${n(y)}) scale(${n(size / 120)})">${body("flower")}</g>"""

    private fun containerBody(place: String): String {
        val inside = place == "inside"
        return """<rect x="9" y="32" width="73" height="72" rx="8" fill="#f1dfbd" stroke="#bb9a6b" stroke-width="3"/><path d="M14 47h63" stroke="#d3b78c" stroke-width="2"/>""" +
            miniFlower(if (inside) 43.0 else 89.0, if (inside) 47.0 else 49.0, 29.0)
    }
    fun container(place: String) = svg(containerBody(place))

    private fun towerBody(colors: List<String>): String {
        val cnt = colors.size.toDouble()
        return colors.mapIndexed { i, c ->
            """<rect x="${n(20 + i * 4.0)}" y="${n((120 + (cnt - 2) * 27) / 2 - i * 27)}" width="${n(80 - i * 8.0)}" height="24" rx="5" fill="$c" stroke="#fff7e6" stroke-width="2"/>"""
        }.joinToString("")
    }
    fun tower(colors: List<String>) = svg(towerBody(colors))

    fun position(dir: String): String {
        val (x, y) = when (dir) { "above" -> 46.0 to 0.0; "below" -> 46.0 to 92.0; "left" -> 0.0 to 46.0; else -> 92.0 to 46.0 }
        return svg("""<rect x="40" y="40" width="40" height="40" rx="5" fill="#debd8c"/><path d="M40 50h40m-20-10v40" stroke="#c4a474" stroke-width="2"/>""" + miniFlower(x, y, 28.0))
    }

    private fun clockBody(hour: Int): String {
        val angle = hour * PI / 6
        val nums = (0 until 12).joinToString("") {
            val a = (it + 1) * PI / 6
            """<text x="${n(60 + 43 * sin(a))}" y="${n(64 - 43 * cos(a))}" text-anchor="middle" font-size="11" font-family="sans-serif" fill="#46625c">${it + 1}</text>"""
        }
        return """<circle cx="60" cy="60" r="56" fill="#fffaf0" stroke="#92b4ad" stroke-width="5"/>$nums<path d="M60 60V27" stroke="#659a8c" stroke-width="4" stroke-linecap="round"/><path d="M60 60L${n(60 + 25 * sin(angle))} ${n(60 - 25 * cos(angle))}" stroke="#294b43" stroke-width="6" stroke-linecap="round"/><circle cx="60" cy="60" r="4" fill="#294b43"/>"""
    }
    fun clock(hour: Int) = svg(clockBody(hour))

    fun garden() = svg(
        """<circle cx="397" cy="68" r="42" fill="#f8d47c"/><path d="M57 63q10-26 31-10 20-34 40 0 23-8 28 14H57" fill="#fff" opacity=".8"/><path d="M7 256Q148 136 285 239q144-100 235-22v83H7" fill="#b7d4b1"/><path d="M2 290q142-89 278-19 147-62 240-13v42H2" fill="#8fba96"/><path d="M332 248v-98" stroke="#8c9a73" stroke-width="14"/><ellipse cx="332" cy="119" rx="61" ry="80" fill="#6d9d7b"/><ellipse cx="300" cy="125" rx="22" ry="40" fill="#7aab86"/>""" +
            """<g transform="translate(160 130) scale(1.25)">${body("bunny")}</g><g transform="translate(63 161) scale(.7)">${body("flower")}</g><g transform="translate(396 179) scale(.65)">${body("bird")}</g><g transform="translate(352 239) scale(.45)">${body("flower", "#f4cd64")}</g>""",
        "0 0 520 300",
    )

    private val singleTiles = setOf("flower", "letters", "book", "clock", "socks", "cup", "tower", "container", "colorpath")
    fun tileWide(kind: String) = kind !in singleTiles

    fun tile(kind: String): String {
        if (kind in singleTiles) return icon(kind)
        val (a, b) = when (kind) {
            "shapes" -> "circle" to "triangle"; "count" -> "apple" to "apple"; "memory" -> "bird" to "bird"
            "pattern" -> "circle" to "star"; "odd" -> "circle" to "square"; "sum" -> "apple" to "star"
            "order" -> "triangle" to "star"; else -> "leaf" to "sun"
        }
        val ca = if (kind == "pattern") "#8c7db3" else "#edab69"
        val cb = if (kind == "pattern") "#8c7db3" else "#62a58e"
        return svg(body(a, ca) + """<g transform="translate(120 0)">${body(b, cb)}</g>""", "0 0 240 120")
    }
}
