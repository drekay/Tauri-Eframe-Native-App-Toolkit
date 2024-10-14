mod window_plugin;
mod window_controller_plugin;

use crossbeam_channel::Receiver;
pub use window_plugin::*;
pub use window_controller_plugin::*;

use eframe::egui::{self};
// If there are any shared structures or functions, you can define or re-export them here

pub trait FramePlugin {
    fn name(&self) -> &str;
    fn set_receiver(&mut self, rx: Receiver<Message>);
    //fn update(&mut self, ctx: &egui::Context);
    fn handle_message(&self, message: Message);
    fn update(&self);
     fn is_dragging(&self) -> bool;
    fn execute(&self, ui: &mut egui::Ui, ctx: &egui::Context);
}

