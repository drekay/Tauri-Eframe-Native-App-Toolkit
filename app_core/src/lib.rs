//app_core/src/lib.rs
pub mod ui;
pub mod plugin_system;
pub mod message_bus;

use crossbeam_channel:: {Sender, Receiver};
use eframe::egui;

pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn update(&mut self, ctx: &egui::Context, rx: &Receiver<Message>, tx: &Sender<Message>);
    fn on_load(&mut self);
    fn on_unload(&mut self);
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
