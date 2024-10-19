//app_main/src/lib.rs
use eframe::{egui, NativeOptions};
use std::path::PathBuf;
use std::fs;
use app_core::PluginSystem;

pub struct TauriEframeNativeApp {
    plugin_system: PluginSystem,
    // Add other fields from your existing TauriEframeNativeApp struct
}

impl TauriEframeNativeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut plugin_system = PluginSystem::new();
     //  let expand_icon = load_image_from_bytes(&cc.egui_ctx, include_bytes!("../../assets/expand.png"), "expand_icon");
     //   let collapse_icon = load_image_from_bytes(&cc.egui_ctx, include_bytes!("../../assets/collapse.png"), "collapse_icon");

        Self {
            plugin_system
    }
}
    
    pub fn newx(cc: &eframe::CreationContext<'_>) -> Self {
        let mut plugin_system = PluginSystem::new();
        
        // Load plugins from plugins.txt
        if let Ok(plugin_list) = std::fs::read_to_string("plugins.txt") {
            for plugin_path in plugin_list.lines() {
               /*  if let Err(e) = plugin_system.load_plugin(Path::new(plugin_path)) {
                    eprintln!("Failed to load plugin {}: {}", plugin_path, e);
                }*/
            }
        }

        // Initialize other fields...
        Self {
            plugin_system,
            // Initialize other fields...
        }
    }
}

impl eframe::App for TauriEframeNativeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.plugin_system.update_all(ctx);
        // Add other update logic
    }
}

// Include other necessary structs, implementations, and functions