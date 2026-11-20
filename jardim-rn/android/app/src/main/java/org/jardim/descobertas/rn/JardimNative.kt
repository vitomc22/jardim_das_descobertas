package org.jardim.descobertas.rn

import android.speech.tts.TextToSpeech
import com.facebook.react.ReactPackage
import com.facebook.react.bridge.NativeModule
import com.facebook.react.bridge.Promise
import com.facebook.react.bridge.ReactApplicationContext
import com.facebook.react.bridge.ReactContextBaseJavaModule
import com.facebook.react.bridge.ReactMethod
import com.facebook.react.uimanager.ViewManager
import java.io.File
import java.util.Locale
import java.util.concurrent.Executors

// Narração pt-BR (TextToSpeech do Android) e progresso em arquivo, gravado
// numa thread própria. Nenhuma biblioteca de terceiros.
class JardimNative(private val ctx: ReactApplicationContext) : ReactContextBaseJavaModule(ctx) {
    private val io = Executors.newSingleThreadExecutor()
    private var ready = false
    private val tts: TextToSpeech = TextToSpeech(ctx) { status ->
        if (status == TextToSpeech.SUCCESS) {
            tts.language = Locale("pt", "BR")
            tts.setSpeechRate(0.88f)
            ready = true
        }
    }
    private val file get() = File(ctx.filesDir, "jardim-v1.json")

    override fun getName() = "JardimNative"

    @ReactMethod fun speak(text: String) { if (ready) tts.speak(text, TextToSpeech.QUEUE_FLUSH, null, "jardim") }
    @ReactMethod fun stop() { tts.stop() }

    @ReactMethod fun load(promise: Promise) {
        io.execute { promise.resolve(runCatching { file.readText() }.getOrNull()) }
    }

    @ReactMethod fun save(json: String) {
        io.execute {
            val tmp = File(ctx.filesDir, "jardim-v1.json.tmp")
            runCatching { tmp.writeText(json); tmp.renameTo(file) }
        }
    }

    override fun invalidate() {
        tts.shutdown()
        io.shutdown()
        super.invalidate()
    }
}

class JardimPackage : ReactPackage {
    override fun createNativeModules(ctx: ReactApplicationContext): List<NativeModule> = listOf(JardimNative(ctx))
    override fun createViewManagers(ctx: ReactApplicationContext): List<ViewManager<*, *>> = emptyList()
}
