// egui_impl/src/lib.rs

mod components;

pub use components::frame::Frame;
// Rename our local Window to avoid conflict
pub use components::window::Window as EguiWindow;

use app_core::{messages::ComponentInteraction, ui::{Window as CoreWindow, WindowResponse as CoreWindowResponse}};
use eframe::egui;

// Define the EguiWindowTrait instead of EguiWindow to avoid naming conflicts
pub trait EguiWindowTrait: CoreWindow {
    fn show_egui(&mut self, ui: &mut egui::Ui, index: usize) -> EguiWindowResponse;
}

pub struct EguiWindowResponse {
    pub core: CoreWindowResponse,
    pub is_closed: bool,
    pub drag_started: bool,
    pub drag_released: bool,
    pub drag_delta: egui::Vec2,
    pub component_interactions: Vec<ComponentInteraction>,
}

#[no_mangle]
pub extern "C" fn create_window(title: &str) -> Box<dyn CoreWindow> {
    Box::new(EguiWindow::new(
        0,  // id
        title,
        egui::Pos2::ZERO,  // position
        egui::Vec2::new(300.0, 200.0)  // size
    ))
}