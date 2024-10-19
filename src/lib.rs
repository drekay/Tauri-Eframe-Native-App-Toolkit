//src/lib.rs
use eframe::egui;
use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::path::Path;

pub trait Plugin {
    fn name(&self) -> &str;
    fn update(&mut self, ctx: &egui::Context);
}

pub struct PluginSystem {
    plugins: HashMap<String, Box<dyn Plugin>>,
    libraries: Vec<Library>,
}

impl PluginSystem {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            libraries: Vec::new(),
        }
    }

    pub fn load_plugin<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let library = Library::new(path.as_ref())?;
            let constructor: Symbol<fn() -> Box<dyn Plugin>> = library.get(b"_plugin_create")?;
            let plugin = constructor();
            let name = plugin.name().to_string();
            self.plugins.insert(name, plugin);
            self.libraries.push(library);
        }
        Ok(())
    }

    pub fn update_all(&mut self, ctx: &egui::Context) {
        for plugin in self.plugins.values_mut() {
            plugin.update(ctx);
        }
    }
}

pub struct TauriEframeNativeApp {
    plugin_system: PluginSystem,
}

impl TauriEframeNativeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut plugin_system = PluginSystem::new();
        
        // Load plugins from plugins.txt
        if let Ok(plugin_list) = std::fs::read_to_string("plugins.txt") {
            for plugin_path in plugin_list.lines() {
                if let Err(e) = plugin_system.load_plugin(plugin_path) {
                    eprintln!("Failed to load plugin {}: {}", plugin_path, e);
                }
            }
        }

        Self { plugin_system }
    }
}

impl eframe::App for TauriEframeNativeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.plugin_system.update_all(ctx);
    }
}