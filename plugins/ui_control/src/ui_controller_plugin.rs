use crate::message_handler::MessageHandler;
use crate::{Message, Plugin, UIPlugin, WindowPluginMessage, ControllerPluginMessage};

pub struct WindowControllerPlugin {
    tx: Sender<Message>,
}

impl WindowControllerPlugin {
    pub fn new(tx: Sender<Message>) -> Self {
        Self { tx }
    }

    fn handle_ui_view_message(&self, message: UIViewPluginMessage) {
        match message {
            UIViewPluginMessage::WindowInteraction(window_id, interaction) => {
                match interaction {
                    WindowInteraction::Close => {
                        self.tx.send(Message::WindowPlugin(WindowPluginMessage::ConfirmedCloseWindow(window_id))).unwrap();
                    },
                    WindowInteraction::Minimize => {
                        self.tx.send(Message::WindowPlugin(WindowPluginMessage::MinimizeWindow(window_id))).unwrap();
                    },
                    WindowInteraction::Drag(pos) => {
                        self.tx.send(Message::WindowPlugin(WindowPluginMessage::DragWindowMove(pos))).unwrap();
                    },
                    // Handle other interactions...
                }
            },
            // Handle other UIViewPlugin messages...
        }
    }
}

impl Plugin for WindowControllerPlugin {
    fn name(&self) -> &str {
        "WindowControllerPlugin"
    }

    fn update(&mut self, ctx: &egui::Context, message_handler: &mut dyn MessageHandler) {
        while let Some(message) = message_handler.receive_message() {
            match message {
                Message::UIViewPlugin(ui_message) => self.handle_ui_view_message(ui_message),
                Message::WindowPlugin(window_message) => {
                    // Handle any responses from WindowPlugin if necessary
                },
                // Handle other relevant message types...
                _ => {} // Ignore messages not relevant to this plugin
            }
        }
    }

    fn on_load(&mut self) {
        // Initialization logic
    }

    fn on_unload(&mut self) {
        // Cleanup logic
    }
}

impl UIPlugin for WindowControllerPlugin {
    fn update_ui(&mut self, ctx: &egui::Context) {
        // This plugin doesn't directly update UI
    }
}