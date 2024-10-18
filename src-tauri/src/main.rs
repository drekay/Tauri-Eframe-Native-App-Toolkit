#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use eframe::egui;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            
            // Create the eframe app
            let options = eframe::NativeOptions::default();
            eframe::run_native(
                "Tauri EFrame Native Demo",
                options,
                Box::new(|cc| Ok(Box::new(TauriEframeNativeApp::new(cc)))),
            );

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct TauriEframeNativeApp {
    // Your app state here
}

impl TauriEframeNativeApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Initialize your app here
        Self {
            // Initialize your app state
        }
    }
}

impl eframe::App for TauriEframeNativeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Your app update logic here
    }
}