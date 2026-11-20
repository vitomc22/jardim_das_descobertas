package org.jardim.descobertas.kt

import org.junit.Assert.assertEquals
import org.junit.Test
import java.io.File

// Paridade com o JS original: js_reference.json foi gerado rodando o
// curriculum.js do APK antigo (o mesmo arquivo das outras versões).
class ParityTest {
    private fun field(json: String, key: String, name: String): String {
        val start = json.indexOf("\"$key\":{")
        val obj = json.substring(start, json.indexOf('}', start))
        val m = Regex("\"$name\":(\"([^\"]*)\"|([0-9]+))").find(obj)!!
        return m.groupValues[2].ifEmpty { m.groupValues[3] }
    }

    @Test fun rngAndHash() {
        val r = Rng(42)
        assertEquals(0.6011037519201636, r.next(), 0.0)
        assertEquals(0.44829055899754167, r.next(), 0.0)
        assertEquals(0.8524657934904099, r.next(), 0.0)
        assertEquals(692878806L, hash("a|b"))
        assertEquals(987461503L, seedFor("2026-11-19", "little", "colors", 1, 7))
    }

    @Test fun sameQuestionsAsJavascript() {
        val ref = File("../../jardim-rs/tests/js_reference.json").readText()
        var ok = 0
        for (g in GAMES) for (grp in listOf("little", "middle", "big")) {
            val k = "${g.id}:$grp"
            val q = generate(g.id, grp, 12345)
            assertEquals(k, field(ref, k, "prompt"), q.prompt)
            assertEquals(k, field(ref, k, "n"), (if (q.memory) q.cards.size else q.options.size).toString())
            if (!q.memory) assertEquals(k, field(ref, k, "answer"), q.answer.toString())
            assertEquals(k, field(ref, k, "surface"), q.presentation.surface)
            ok++
        }
        assertEquals(96, ok)
    }
}
