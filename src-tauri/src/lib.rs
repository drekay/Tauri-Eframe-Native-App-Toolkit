use std::sync::Arc;
use std::sync::Mutex;

// lib.rs
use eframe::egui;
use crossbeam_channel::{unbounded, Receiver, Sender};

mod plugins;
use plugins::window_management::WindowState;
use plugins::window_management::Message;
use crate::plugins::ui_controller::UiControllerPlugin;

pub struct TauriEframeNativeApp {
    ui_controller: UiControllerPlugin,
    state: Arc<Mutex<WindowState>>,
    tx: Sender<Message>,
    rx: Receiver<Message>,
}

impl TauriEframeNativeApp {
  pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    let (tx, rx) = crossbeam_channel::unbounded();
    let state = Arc::new(Mutex::new(WindowState::new(tx.clone())));
    
    Self {
      ui_controller: UiControllerPlugin::new(cc, tx.clone(),state.clone()),
      state: state.clone(),
      tx,
      rx,
  }
}
}

impl eframe::App for TauriEframeNativeApp {
  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
   

    // Pass the shared state to the UI controller
    self.ui_controller.update(ctx);//, frame);
}
}