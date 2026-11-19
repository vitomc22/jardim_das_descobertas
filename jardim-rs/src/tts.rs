//! Narração pt-BR pelo TextToSpeech do Android, chamado direto do Rust via JNI
//! (sem nenhuma classe Java no APK).

#[cfg(target_os = "android")]
mod imp {
    use jni::objects::{GlobalRef, JObject, JValue};
    use jni::JavaVM;

    pub struct Tts {
        vm: JavaVM,
        tts: GlobalRef,
        configured: bool,
    }

    fn clear(env: &mut jni::JNIEnv) {
        if env.exception_check().unwrap_or(false) {
            let _ = env.exception_clear();
        }
    }

    impl Tts {
        pub fn new(app: &winit::platform::android::activity::AndroidApp) -> Option<Self> {
            let vm = unsafe { JavaVM::from_raw(app.vm_as_ptr().cast()) }.ok()?;
            let tts = {
                let mut env = vm.attach_current_thread_permanently().ok()?;
                let activity = unsafe { JObject::from_raw(app.activity_as_ptr().cast()) };
                let obj = env.new_object(
                    "android/speech/tts/TextToSpeech",
                    "(Landroid/content/Context;Landroid/speech/tts/TextToSpeech$OnInitListener;)V",
                    &[JValue::Object(&activity), JValue::Object(&JObject::null())],
                );
                clear(&mut env);
                let obj = obj.ok()?;
                env.new_global_ref(obj).ok()?
            };
            Some(Tts { vm, tts, configured: false })
        }

        /// O motor liga de forma assíncrona; configuramos na primeira fala em
        /// que ele já responde.
        fn configure(tts: &GlobalRef, env: &mut jni::JNIEnv) -> bool {
            let Ok(pt) = env.new_string("pt") else { return false };
            let Ok(br) = env.new_string("BR") else { return false };
            let locale = env.new_object("java/util/Locale", "(Ljava/lang/String;Ljava/lang/String;)V", &[(&pt).into(), (&br).into()]);
            clear(env);
            let Ok(locale) = locale else { return false };
            let r = env.call_method(tts.as_obj(), "setLanguage", "(Ljava/util/Locale;)I", &[(&locale).into()]);
            clear(env);
            if matches!(r.and_then(|v| v.i()), Ok(code) if code >= 0) {
                let _ = env.call_method(tts.as_obj(), "setSpeechRate", "(F)I", &[JValue::Float(0.88)]);
                clear(env);
                return true;
            }
            false
        }

        pub fn speak(&mut self, text: &str) {
            let Ok(mut env) = self.vm.attach_current_thread_permanently() else { return };
            if !self.configured {
                self.configured = Self::configure(&self.tts, &mut env);
            }
            let (Ok(t), Ok(id)) = (env.new_string(text), env.new_string("jardim")) else { return };
            let _ = env.call_method(
                self.tts.as_obj(),
                "speak",
                "(Ljava/lang/CharSequence;ILandroid/os/Bundle;Ljava/lang/String;)I",
                &[(&t).into(), JValue::Int(0), JValue::Object(&JObject::null()), (&id).into()],
            );
            clear(&mut env);
        }

        pub fn stop(&mut self) {
            let Ok(mut env) = self.vm.attach_current_thread_permanently() else { return };
            let _ = env.call_method(self.tts.as_obj(), "stop", "()I", &[]);
            clear(&mut env);
        }
    }

    impl Drop for Tts {
        fn drop(&mut self) {
            if let Ok(mut env) = self.vm.attach_current_thread_permanently() {
                let _ = env.call_method(self.tts.as_obj(), "shutdown", "()V", &[]);
                clear(&mut env);
            }
        }
    }
}

#[cfg(not(target_os = "android"))]
mod imp {
    /// No desktop (testes), a narração só vai para o log.
    pub struct Tts;
    impl Tts {
        pub fn speak(&mut self, text: &str) {
            log::info!("[voz] {text}");
        }
        pub fn stop(&mut self) {}
    }
}

pub use imp::Tts;
