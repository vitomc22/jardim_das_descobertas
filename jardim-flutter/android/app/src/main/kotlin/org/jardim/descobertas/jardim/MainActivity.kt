package org.jardim.descobertas.jardim

import android.speech.tts.TextToSpeech
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodChannel
import java.util.Locale

// Narração pt-BR (TextToSpeech do Android) e caminho de gravação, expostos ao
// Dart por um MethodChannel. Nenhum plugin de terceiros.
class MainActivity : FlutterActivity() {
    private var tts: TextToSpeech? = null
    private var ready = false

    override fun configureFlutterEngine(engine: FlutterEngine) {
        super.configureFlutterEngine(engine)
        tts = TextToSpeech(this) { status ->
            if (status == TextToSpeech.SUCCESS) {
                tts?.language = Locale("pt", "BR")
                tts?.setSpeechRate(0.88f)
                ready = true
            }
        }
        MethodChannel(engine.dartExecutor.binaryMessenger, "jardim").setMethodCallHandler { call, result ->
            when (call.method) {
                "speak" -> { if (ready) tts?.speak(call.arguments as String, TextToSpeech.QUEUE_FLUSH, null, "jardim"); result.success(null) }
                "stop" -> { tts?.stop(); result.success(null) }
                "filesDir" -> result.success(filesDir.absolutePath)
                "exit" -> { finish(); result.success(null) }
                else -> result.notImplemented()
            }
        }
    }

    override fun onDestroy() {
        tts?.shutdown()
        super.onDestroy()
    }
}
