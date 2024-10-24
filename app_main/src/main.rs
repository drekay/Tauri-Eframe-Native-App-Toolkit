use eframe::NativeOptions;
use app_main::TauriEframeNativeApp;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = NativeOptions::default();
    println!("Run App_main TauriEframeNativeApp::new(cc)");
    eframe::run_native(
        "Tauri EFrame Native Demo",
        options,
        Box::new(|cc|  Ok(Box::new(TauriEframeNativeApp::new(cc)))),
    )?;
  
    Ok(())
  }