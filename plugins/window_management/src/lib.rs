//plugins/window_management/src/lib.rs
mod window_plugin;
//addback mod window_controller;

use crossbeam_channel::{Receiver, Sender};
pub use window_plugin::WindowPlugin;
use app_core::{Message, Plugin};
use eframe::egui;

pub struct WindowManagementPlugin {
    window_plugin: window_plugin::WindowPlugin,
   //addback window_controller: window_controller::WindowController,
}

impl Plugin for WindowManagementPlugin {
    fn name(&self) -> &str {
        "Window Management"
    }

    fn on_load(&mut self) {
        println!("Window Management plugin loaded");
        self.window_plugin.on_load();
       //addback  self.window_controller.on_load();
    }

    fn on_unload(&mut self) {
        println!("Window Management plugin unloaded");
        self.window_plugin.on_unload();
       //addback  self.window_controller.on_unload();
    }

    fn update(&mut self, ctx: &egui::Context, rx: &Receiver<Message>, tx: &Sender<Message>) {
       // self.window_plugin.update(ctx);
       //addback  self.window_controller.update(ctx);
    }
}

#[no_mangle]
pub fn create_plugin(tx: Sender<Message>) -> Box<dyn Plugin> {
    Box::new(WindowPlugin::new(tx)) //,
    //addback    window_controller: window_controller::WindowController::new(),

}