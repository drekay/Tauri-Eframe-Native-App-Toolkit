use app_core::{PluginSystem, Message};
use eframe::{egui, NativeOptions};
use std::path::PathBuf;
use std::fs;

struct TauriEframeNativeApp {
    plugin_system: PluginSystem,
}

impl TauriEframeNativeApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut plugin_system = PluginSystem::new();
        
        // Get the current executable's directory
        let exe_dir = std::env::current_exe()?
            .parent()
            .ok_or("Failed to get executable directory")?
            .to_path_buf();

        // Load plugin list from a configuration file
        let plugin_list = fs::read_to_string(exe_dir.join("plugins.txt"))?;
        let plugins: Vec<String> = plugin_list.lines().map(String::from).collect();

        // Load plugins
        for plugin in plugins {
            let plugin_path = exe_dir.join(&plugin);
            match plugin_system.load_plugin(&plugin_path.to_string_lossy()) {
                Ok(_) => println!("Loaded plugin: {}", plugin),
                Err(e) => eprintln!("Failed to load plugin {}: {}", plugin, e),
            }
        }
        
        Ok(Self { plugin_system })
    }
}

impl eframe::App for TauriEframeNativeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.plugin_system.update_all(ctx);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = NativeOptions::default();

    eframe::run_native(
        "Tauri EFrame Native Demo",
        options,
        Box::new(|cc| Box::new(TauriEframeNativeApp::new(cc)?)),
    )?;

    Ok(())
}