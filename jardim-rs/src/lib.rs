//! Jardim das Descobertas — versão nativa (Rust + NDK, sem WebView).

pub mod art;
pub mod curriculum;
pub mod raster;
pub mod rng;
pub mod store;
pub mod tts;

#[cfg(target_os = "android")]
mod app;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(android_app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::activity::WindowManagerFlags;
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info).with_tag("jardim"));
    android_app.set_window_flags(WindowManagerFlags::KEEP_SCREEN_ON, WindowManagerFlags::empty());

    let store = store::Store::open(android_app.internal_data_path());
    let tts = tts::Tts::new(&android_app);
    let handle = android_app.clone();
    let options = eframe::NativeOptions {
        android_app: Some(android_app),
        // Só redesenha com toque/animação; vsync ligado.
        vsync: true,
        dithering: false,
        ..Default::default()
    };
    let mut slot = Some((store, tts));
    let r = eframe::run_native(
        "Jardim das Descobertas",
        options,
        Box::new(move |cc| {
            let (store, tts) = slot.take().expect("app criado uma vez");
            Ok(Box::new(app::JardimApp::new(cc, store, tts, handle.clone())))
        }),
    );
    if let Err(e) = r {
        log::error!("eframe: {e}");
    }
}
