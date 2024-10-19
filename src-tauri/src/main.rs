#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app_main::TauriEframeNativeApp;
use tauri::Manager;
use eframe::NativeOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("Run Webview TauriEframeNativeApp::new(cc)");
   /* let options = NativeOptions::default();
  
    eframe::run_native(
        "Tauri EFrame Native Demo",
        options,
        Box::new(|cc|  Ok(Box::new(TauriEframeNativeApp::new(cc))),
    )?;
  */
    Ok(())
  }