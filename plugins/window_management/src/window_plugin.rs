//plugins/window_management/src/window_plugin.rs
use eframe::egui;
use crossbeam_channel::{Sender, Receiver};
use std::sync::{Arc, Mutex};
use app_core::{Plugin, Message, WindowPluginMessage};


pub struct WindowPlugin {
    state: Arc<Mutex<WindowState>>,
    tx: Sender<Message>,
}

pub struct WindowState {
    pub windows: Vec<Window>,
    pub grid: Vec<usize>,
    pub about_counter: usize,
    pub expanded_height: f32,
    pub collapsed_height: f32,
    pub sender: Sender<Message>,
}

impl WindowState {
    pub fn new(sender: Sender<Message>) -> Self {
        Self {
            windows: Vec::new(),
            grid: Vec::new(),
            about_counter: 0,
            expanded_height: 100.0,
            collapsed_height: 30.0,
            sender,
        }
    }
}
struct Window {
    title: String,
    content: String,
    is_minimized: bool,
}

impl WindowPlugin {
    pub fn new(tx: Sender<Message>) -> Self {
        let state = Arc::new(Mutex::new(WindowState::new(tx.clone())));
        Self {
            state,
            tx,
            // Initialize other fields as needed
        }
    }
}

impl Plugin for WindowPlugin {
    fn name(&self) -> &str {
        "WindowPlugin"
    }

   
     fn update(&mut self, ctx: &egui::Context, rx: &Receiver<Message>, tx: &Sender<Message>) {
        // Process incoming messages
        while let Ok(message) = rx.try_recv() {
            self.handle_message(message, tx);
        }

        // Update UI
        self.update_ui(ctx);  
        println!("WindowPlug update ...yey!");
    }

    fn on_load(&mut self) {
        // Add a placeholder window when the plugin loads
        let mut state = self.state.lock().unwrap();
        state.windows.push(Window {
            title: "Placeholder".to_string(),
            content: "This is a placeholder window".to_string(),
            is_minimized: false,
        });
    }

    fn on_unload(&mut self) {
        // Clean up if necessary
    }
}

impl WindowPlugin {
    fn handle_message(&mut self, message: Message, tx: &Sender<Message>) {
        match message {
            Message::WindowPlugin(window_message) => {
                match window_message {
                    WindowPluginMessage::AddWindow => {
                        let mut state = self.state.lock().unwrap();
                        state.about_counter += 1;
                        /*addback state.windows.push(Window {
                            title: format!("Window {}", state.about_counter),
                            content: format!("This is window number {}", state.about_counter),
                            is_minimized: false,
                        });*/
                        println!("Added a new window");
                    },
                    WindowPluginMessage::CloseWindow(_) => todo!(), /*remove*/
                    /*addback WindowPluginMessage::CollapseWindow(id) => {
                        let mut state = self.state.lock().unwrap();
                        if let Some(window) = state.windows.get_mut(id) {
                            window.is_minimized = !window.is_minimized;
                            println!("Toggled window {} minimization", id);
                        }
                    },*/
                    // Handle other WindowPluginMessage variants as needed
                }
            },
            _ => {} // Ignore other message types
        }
    }

    fn update_ui(&self, ctx: &egui::Context) {
        let state = self.state.lock().unwrap();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (index, window) in state.windows.iter().enumerate() {
                    self.render_window(ui, window, index);
                }
            });
        });
    }

    fn render_window(&self, ui: &mut egui::Ui, window: &Window, index: usize) {
        let frame = egui::Frame::none()
            .fill(egui::Color32::from_gray(240))
            .stroke(egui::Stroke::new(1.0, egui::Color32::BLACK))
            .rounding(egui::Rounding::same(5.0))
            .shadow(egui::epaint::Shadow::small_light());

        frame.show(ui, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(&window.title);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("❌").clicked() {
                        //addback    let _ = self.tx.send(Message::WindowPlugin(WindowPluginMessage::ConfirmedCloseWindow(index)));
                        }
                        if ui.button(if window.is_minimized { "🗖" } else { "🗕" }).clicked() {
                         //addback   let _ = self.tx.send(Message::WindowPlugin(WindowPluginMessage::CollapseWindow(index)));
                        }
                    });
                });

                if !window.is_minimized {
                    ui.label(&window.content);
                }
            });
        });
    }
}