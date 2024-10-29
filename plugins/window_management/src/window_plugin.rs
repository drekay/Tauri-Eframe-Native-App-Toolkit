use eframe::egui;
use std::sync::{Arc, Mutex};
use app_core::{messages::{ControllerMessage, Message, WindowControllerMessage, WindowPluginMessage}, plugin_version, MessageHandler, Plugin, PluginType, PluginVersion, VersionComparable, VersionEquatable, Versioned};
use egui_impl::EguiWindow as Window;

pub struct WindowPlugin {
    state: Arc<Mutex<WindowState>>,
    version: PluginVersion,    
}

pub struct WindowState {
    pub windows: Vec<Window>,
    pub grid: Vec<usize>,
    pub about_counter: usize,
    pub expanded_height: f32,
    pub collapsed_height: f32,
}

impl WindowState {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            grid: Vec::new(),
            about_counter: 0,
            expanded_height: 100.0,
            collapsed_height: 30.0,
        }
    }
}

impl WindowPlugin {
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(WindowState::new()));
        Self {
            version: plugin_version!(0, 1, 0),
            state,
        }
    }

    fn update_ui(&mut self, ctx: &egui::Context) {
        // Implement UI update logic here
        println!("Updating WindowPlugin UI");
    }
}

impl Versioned for WindowPlugin {
    fn get_version(&self) -> &PluginVersion {
        &self.version
    }
}

impl VersionComparable for WindowPlugin {}
impl VersionEquatable for WindowPlugin {}

impl Plugin for WindowPlugin {
    fn name(&self) -> &str {
        "WindowPlugin"
    }

    fn plugin_type(&self) -> PluginType {
        PluginType::UI
    }

    fn controller(&self) -> Option<&str> {
        Some("UIControllerPlugin")
    }

    fn is_enabled(&self) -> bool {
        true // Or implement a field for this
    }

    fn set_enabled(&mut self, _enabled: bool) {
        // Implement enabling/disabling logic
    }   
   
    fn update(&mut self, ctx: &egui::Context, message_handler: &mut dyn MessageHandler) {
        // Collect messages first
        let mut messages = Vec::new();
        while let Some(message) = message_handler.receive_message() {
            messages.push(message);
        }
    
        // Process collected messages
        for message in messages {
            self.handle_message(message, message_handler);
        }
    
        // Update UI
        self.update_ui(ctx);
        println!("WindowPlugin update...yey!");
    }

    fn on_load(&mut self) {
        // Add initialization logic if needed
    }

    fn on_unload(&mut self) {
        // Clean up if necessary
    }
}

impl WindowPlugin {
    fn handle_message(&mut self, message: Message, message_handler: &mut dyn MessageHandler) {
        match message {
            Message::PluginSpecific { content, priority } => {
                if let Some(window_message) = content.as_any().downcast_ref::<WindowPluginMessage>() {
                    match window_message {
                        WindowPluginMessage::AddWindow => {
                            println!("Adding a new window");
                            self.add_window();
                            let response = Message::ControllerMessage {
                                target_controller_id: "window_controller".to_string(),
                                content: ControllerMessage::WindowController(
                                    WindowControllerMessage::RequestAddWindow { priority }
                                ),
                                priority,
                            };
                            message_handler.send_message(response);
                        },
                        WindowPluginMessage::ConfirmedCloseWindow(index) => {
                            self.close_window(*index);
                        },
                        WindowPluginMessage::MinimizeWindow(index) => {
                            self.minimize_window(*index);
                        },
                        WindowPluginMessage::DragWindowStart(index, pos) => {
                            self.start_window_drag(*index, *pos);
                        },
                        WindowPluginMessage::DragWindowMove(pos) => {
                            self.move_dragged_window(*pos);
                        },
                        WindowPluginMessage::DragWindowEnd => {
                            self.end_window_drag();
                        },
                        WindowPluginMessage::CollapseWindow(index) => {
                            self.collapse_window(*index);
                        },
                    }
                }
            },
            Message::ControllerMessage { content: ControllerMessage::WindowController(controller_msg), priority, .. } => {
                match controller_msg {
                    WindowControllerMessage::RequestCloseWindow { window_index, .. } => {
                        /* ADDBACK self.close_window(*window_index);
                        let response = Message::ControllerMessage {
                            target_controller_id: "window_controller".to_string(),
                            content: ControllerMessage::WindowController(
                                WindowControllerMessage::RequestCloseWindow { window_index: *window_index, priority }
                            ),
                            priority,
                        };
                        message_handler.send_message(response); */
                    },
                    WindowControllerMessage::RequestAddWindow { .. } => {
                        self.add_window();
                    },
                    // Handle other WindowControllerMessage variants as needed
                }
            },
            _ => {
                // Ignore messages not relevant to this plugin
                println!("Message ignored by WindowPlugin");
            }
        }
    }

    fn add_window(&mut self) {
        println!("pass window to UIViewPlugin");
        // Implement actual window addition logic here
    }

    fn close_window(&mut self, index: usize) {
        let mut state = self.state.lock().unwrap();
        if index < state.windows.len() {
            state.windows.remove(index);
            state.grid.retain(|&x| x != index);
            for grid_index in state.grid.iter_mut() {
                if *grid_index > index {
                    *grid_index -= 1;
                }
            }
            println!("Window closed");
        }
    }

    fn minimize_window(&mut self, _index: usize) {
        // Implement minimize logic
    }

    fn start_window_drag(&mut self, _index: usize, _pos: egui::Pos2) {
        // Implement drag start logic
    }

    fn move_dragged_window(&mut self, _pos: egui::Pos2) {
        // Implement drag move logic
    }

    fn end_window_drag(&mut self) {
        // Implement drag end logic
    }

    fn collapse_window(&mut self, _index: usize) {
        // Implement collapse logic
    }
}