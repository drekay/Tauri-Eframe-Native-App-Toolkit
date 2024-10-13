mod window_plugin;
mod window_controller_plugin;

pub use window_plugin::*;
pub use window_controller_plugin::*;

use eframe::egui::{self};
// If there are any shared structures or functions, you can define or re-export them here

pub trait FramePlugin {
    fn handle_message(&self, message: Message);
    fn update(&self);
     fn is_dragging(&self) -> bool;
    fn execute(&self, ui: &mut egui::Ui, ctx: &egui::Context);
}

