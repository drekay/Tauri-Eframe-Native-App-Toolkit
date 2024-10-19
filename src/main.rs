//src/main/.rs
use app_core::TauriEframeNativeApp;
use eframe::NativeOptions;


fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Tauri EFrame Native Demo",
        options,
        Box::new(|cc| {
            match TauriEframeNativeApp::new() {
                Ok(app) => Box::new(app),
                Err(e) => {
                    eprintln!("Error creating app: {}", e);
                    std::process::exit(1);
                }
            }
        }),
    )
}