//main.rs
use eframe::{egui, CreationContext, NativeOptions, App};
use plugins::window_management::{WindowPlugin, WindowState, WindowControllerPlugin};
use std::sync::{Arc, Mutex};
use crossbeam_channel::{unbounded, Sender};

mod plugins;
use crate::plugins::ui_controller::UiControllerPlugin;

struct TauriEframeNativeApp {
    ui_controller: UiControllerPlugin,
}

impl TauriEframeNativeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        let state = Arc::new(Mutex::new(WindowState::new(tx.clone())));
        
        let (tx, rx) = crossbeam_channel::unbounded();
        let mut ui_controller = UiControllerPlugin::new(cc, tx.clone(), state.clone());
        
        let window_plugin = WindowPlugin::new(tx.clone(),rx.clone());
        ui_controller.add_plugin(Box::new(window_plugin));
        
        let window_management_plugin = WindowControllerPlugin::new(tx.clone(),rx.clone());
        ui_controller.add_plugin(Box::new(window_management_plugin));
        
        // Add other plugins as needed
        // let new_plugin = NewPlugin::new(tx.clone(),rx.clone());
        // ui_controller.add_plugin(Box::new(new_plugin));

        Self {
            ui_controller,
            // initialize other fields if needed...
        }
    }
}

impl App for TauriEframeNativeApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.ui_controller.update(ctx, frame);
      //  println!("Main update");
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions::default();

    eframe::run_native(
        "window_management Demo",
        options,
        Box::new(|cc| Ok(Box::new(TauriEframeNativeApp::new(cc))))
    )
}