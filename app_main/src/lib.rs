//app_main/src/lib.rs
use eframe::{egui, NativeOptions};
use std::path::PathBuf;
use std::fs;
use app_core::PluginSystem;

pub struct TauriEframeNativeApp {
    plugin_system: PluginSystem,
}

impl TauriEframeNativeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut plugin_system = PluginSystem::new();

        // Load plugins from plugins.txt
        if let Ok(plugin_list) = fs::read_to_string("plugins.txt") {
            for plugin_path in plugin_list.lines() {
                
                if let Err(e) = plugin_system.load_plugin(PathBuf::from(plugin_path)) {
                    eprintln!("Failed to load plugin {}: {}", plugin_path, e);
                }
            }
        }

        Self {
            plugin_system,
        }
    }
}

impl eframe::App for TauriEframeNativeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update all plugins
        self.plugin_system.update_all(ctx);

        // Add a button to create new windows
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if ui.button("Add Window").clicked() {
             //addback   self.plugin_system.send_message("WindowPlugin", app_core::Message::WindowPlugin(app_core::WindowPluginMessage::AddWindow));
            }
        });
    }
}