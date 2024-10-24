use app_core::{MessageHandler, UIPlugin};
use crossbeam_channel::Sender;
use crate::{Message, Plugin};

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
    
    fn plugin_type(&self) -> app_core::PluginType {
        todo!()
    }
    
    fn controller(&self) -> Option<&str> {
        todo!()
    }
    
    fn is_enabled(&self) -> bool {
        todo!()
    }
    
    fn set_enabled(&mut self, enabled: bool) {
        todo!()
    }
    
    fn handle_message(&mut self, message: Message, message_handler: &mut dyn app_core::MessageHandler) {
        todo!()
    }
}

impl UIPlugin for WindowControllerPlugin {
    fn update_ui(&mut self, ctx: &egui::Context) {
        // This plugin doesn't directly update UI
    }
}