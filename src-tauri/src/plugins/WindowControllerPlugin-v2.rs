// plugins/window_controller.rs
use eframe::{egui, Context};
use crossbeam_channel::{Receiver, Sender};

pub struct WindowControllerPlugin {
    // existing fields...
}

impl WindowControllerPlugin {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        tx: Sender<()>,
        state: Arc<Mutex<AboutWindowState>>,
        plugins: Vec<Box<dyn Plugin + Send>>,
    ) -> Self {
        let mut this = Self {
            // existing fields...
        };

        for plugin in plugins.iter_mut() {
            this.add_plugin(plugin);
        }

        this
    }
}

impl Plugin for WindowControllerPlugin {
    fn update(&mut self, _ctx: &egui::Context, frame: &mut eframe::Frame) -> eframe::UpdateStatus {
        return eframe::UpdateStatus::NoChange;
    }

    fn execute(
        &mut self,
        ctx: &egui::Context,
        app: &eframe::App,
        _input: &egui::InputState,
        frame: &mut eframe::Frame,
        _ui: &mut egui::Ui,
    ) {
        // Execute logic here...
        return;
    }

    fn handle_message(
        &mut self,
        ctx: &egui::Context,
        app: &eframe::App,
        input: &egui::InputState,
        frame: &mut eframe::Frame,
        message: &eframe::Message,
    ) -> eframe::UpdateStatus {
        match message {
            // Handle window closing event...
            eframe::Message::CloseWindow => {
                app.quit();
                return eframe::UpdateStatus::NoChange;
            }
            _ => {}
        }

        return eframe::UpdateStatus::NoChange;
    }
}