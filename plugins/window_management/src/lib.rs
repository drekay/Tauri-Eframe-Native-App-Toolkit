//plugins/window_management/src/lib.rs
mod window_plugin;
mod window_controller;

use app_core::Plugin;
use eframe::egui;

pub struct WindowManagementPlugin {
    window_plugin: window_plugin::WindowPlugin,
    window_controller: window_controller::WindowController,
}

impl Plugin for WindowManagementPlugin {
    fn name(&self) -> &str {
        "Window Management"
    }

    fn on_load(&mut self) {
        println!("Window Management plugin loaded");
        self.window_plugin.on_load();
        self.window_controller.on_load();
    }

    fn on_unload(&mut self) {
        println!("Window Management plugin unloaded");
        self.window_plugin.on_unload();
        self.window_controller.on_unload();
    }

    fn update(&mut self, ctx: &egui::Context) {
        self.window_plugin.update(ctx);
        self.window_controller.update(ctx);
    }
}

#[no_mangle]
pub fn create_plugin() -> Box<dyn Plugin> {
    Box::new(WindowManagementPlugin {
        window_plugin: window_plugin::WindowPlugin::new(),
        window_controller: window_controller::WindowController::new(),
    })
}