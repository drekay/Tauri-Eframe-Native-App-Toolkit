//app_core/src/lib.rs
pub mod ui;
pub mod plugin_system;
pub mod message_bus;

use eframe::egui;

pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn on_load(&mut self);
    fn on_unload(&mut self);
    fn update(&mut self, ctx: &egui::Context);
}

pub trait UIPlugin: Plugin {
    fn update_ui(&mut self, ctx: &egui::Context);
  }

#[derive(Debug, Clone)]
pub enum Message {
    Broadcast(String),
    WindowPlugin(WindowPluginMessage),
    WindowControllerPlugin(WindowControllerPluginMessage),
}

#[derive(Debug, Clone)]
pub enum WindowPluginMessage {
    AddWindow,
    CloseWindow(usize),
    // Add other window-related messages
}

#[derive(Debug, Clone)]
pub enum WindowControllerPluginMessage {
    CloseWindow(usize),
    // Add other window controller messages
}

pub use plugin_system::PluginSystem;
pub use ui::Window;
pub use ui::WindowResponse;

// UI traits
/*pub mod ui {
    use std::any::Any;

    pub trait Window {
        fn new(title: &str) -> Self where Self: Sized;
        fn show(&mut self, context: &mut dyn Any) -> WindowResponse;
        fn title(&self) -> &str;
        fn set_title(&mut self, title: &str);
        fn is_minimized(&self) -> bool;
        fn set_minimized(&mut self, minimized: bool);
    }

    pub struct WindowResponse {
        pub is_closed: bool,
        pub is_minimized: bool,
        pub drag_delta: (f32, f32),
    }
}*/

/* need Frame re-imported here??
failed to resolve: use of undeclared crate or module `egui_impl`
use of undeclared crate or module `egui_impl`
// Re-export the Frame from egui_impl
pub use egui_impl::components::frame::Frame;*/