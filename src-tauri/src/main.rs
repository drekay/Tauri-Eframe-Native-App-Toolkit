use eframe::{egui, NativeOptions};
use std::sync::{Arc, Mutex};
use crossbeam_channel::unbounded;

mod plugins;
use plugins::about_window;
use plugins::ui_controller::UiController;

struct TauriEframeNativeApp {
    state: Arc<Mutex<about_window::AboutWindowState>>,
    ui_controller: UiController,
}

impl eframe::App for TauriEframeNativeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let messages = self.ui_controller.update(ctx);

        // Send all collected messages
        let state = self.state.lock().unwrap();
        for message in messages {
            let _ = state.sender.send(message);
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions::default();
    
    eframe::run_native(
        "My eframe app",
        options,
        Box::new(|_cc| {
            // Create state and channels inside the closure
            let (sender, receiver) = unbounded();
            let state = Arc::new(Mutex::new(about_window::AboutWindowState {
                windows: Vec::new(),
                about_counter: 0,
                expanded_height: 110.0,
                collapsed_height: 40.0,
                gap_height: 10.0,
                dragged_window: None,
                sender: sender.clone(),
                grid: Vec::new(),
            }));

            let ui_controller = UiController::new(&_cc.egui_ctx, state.clone(), receiver);
   
            Ok(Box::new(TauriEframeNativeApp {
                state: state,
                ui_controller,
            }))
        }),
    )
}
