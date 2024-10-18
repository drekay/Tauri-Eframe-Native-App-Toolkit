//plugins/ui_controller/src/lib.rs
mod ui_controller_plugin;

use app_core::Plugin;
use eframe::egui;

pub struct UiControllerPlugin {
    controller: ui_controller_plugin::UiController,
}

impl Plugin for UiControllerPlugin {
    fn name(&self) -> &str {
        "UI Controller"
    }

    fn on_load(&mut self) {
        println!("UI Controller plugin loaded");
        self.controller.on_load();
    }

    fn on_unload(&mut self) {
        println!("UI Controller plugin unloaded");
        self.controller.on_unload();
    }

    fn update(&mut self, ctx: &egui::Context) {
        self.controller.update(ctx);
    }
}

#[no_mangle]
pub fn create_plugin() -> Box<dyn Plugin> {
    Box::new(UiControllerPlugin {
        controller: ui_controller_plugin::UiController::new(),
    })
}