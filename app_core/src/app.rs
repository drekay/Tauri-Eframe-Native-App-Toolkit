// src/core/app.rs
use eframe::{egui, App};
use std::sync::{Arc, Mutex};
use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::plugins::window_management::{WindowControllerPlugin, WindowPlugin};
use super::message_bus::MessageBus;
use super::PluginSystem;

pub struct TauriEframeNativeApp {
    plugin_system: PluginSystem,
  
} // window_state: Arc<Mutex<WindowState>>,

impl TauriEframeNativeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut plugin_system = PluginSystem::new();
        /*let window_state = Arc::new(Mutex::new(WindowState::new (None)));

        let (tx, rx) = unbounded();

        // Create UIControllerPlugin
        let ui_controller_plugin = UiControllerPlugin::new(
            cc,
            tx.clone(),
            rx.clone(),
            window_state.clone()
        );*/

        let (tx, rx) = unbounded();
        let (ui_tx, ui_rx) = unbounded();

        let window_plugin = WindowPlugin::new(tx.clone(), rx.clone());
        let window_controller_plugin = WindowControllerPlugin::new(ui_tx,  ui_rx);

        // Add plugins to the system
       // plugin_system.add_ui_plugin(Box::new(ui_controller_plugin));
        //plugin_system.add_plugin(Box::new(window_plugin));
        //plugin_system.add_plugin(Box::new(window_controller_plugin));

        Self {
            plugin_system,
        }
    }
}

impl App for TauriEframeNativeApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.plugin_system.update_all(ctx);
    }
}