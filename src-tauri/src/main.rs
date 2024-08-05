use eframe::{egui, NativeOptions};
use std::sync::{Arc, Mutex};
use crossbeam_channel::unbounded;

mod plugins;
use plugins::about_window;
use plugins::ui_controller::UiController;

struct MyEguiApp {
    state: Arc<Mutex<about_window::AboutWindowState>>,
    ui_controller: UiController,
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if ui.button("Add About Window").clicked() {
                let state = self.state.lock().unwrap();
                let _ = state.sender.send(about_window::Message::AddWindow);
            }
        });

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
            }));

            let ui_controller = UiController::new(state.clone(), receiver);

            Ok(Box::new(MyEguiApp {
                state: state,
                ui_controller,
            }))
        }),
    )
}
