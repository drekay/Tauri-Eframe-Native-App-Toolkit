//plugins/window_management/src/lib.rs
mod window_plugin;
mod window_controller_plugin;
mod window_controller_messages;

use crossbeam_channel::{Receiver, Sender};
pub use window_plugin::WindowPlugin;
pub use window_controller_plugin::{WindowControllerPlugin, ControllerSettings};
use app_core::{messages::Message, Plugin};
use egui_impl::EguiWindow;
use eframe::egui;

pub struct WindowManagementPlugin {
    window_plugin: window_plugin::WindowPlugin,
   window_controller: window_controller_plugin::WindowControllerPlugin,
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


    
    fn plugin_type(&self) -> app_core::PluginType {
        todo!()
    }
    
    fn controller(&self) -> Option<&str> {
        todo!()
    }
    
    fn is_enabled(&self) -> bool {
        todo!()
    }
    
    fn set_enabled(&mut self, enabled: bool) {
        todo!()
    }
    
    fn update(&mut self, ctx: &egui::Context, message_handler: &mut dyn app_core::MessageHandler) {
       // self.window_plugin.update(ctx);
        //addback  self.window_controller.update(ctx);
    }

}

#[no_mangle]
pub fn create_plugin(tx: Sender<Message>) -> Box<dyn Plugin> {
    Box::new(WindowPlugin::new(tx)) //,
    //addback    window_controller: window_controller::WindowController::new(),

}
