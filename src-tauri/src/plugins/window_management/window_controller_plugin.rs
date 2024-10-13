// plugins/window_management/window_controller_plugin.rs
use std::collections::HashMap;
use crossbeam_channel::{Receiver, Sender};

use super::{FramePlugin, Message};

pub struct WindowControllerPlugin {
    receiver: Receiver<Message>,
    tx: Sender<Message>,
    window_metadata: HashMap<usize, WindowMetadata>,
}

struct WindowMetadata {
    needs_verification: bool,
    associated_plugins: Vec<String>,
}

impl WindowControllerPlugin {
    pub fn new(tx: Sender<Message>, rx: Receiver<Message>) -> Self {
        Self {
            receiver: rx,
            tx,
            window_metadata: HashMap::new(),
        }
    }

    pub fn add_window_metadata(&mut self, index: usize, metadata: WindowMetadata) {
        self.window_metadata.insert(index, metadata);
    }

    fn verify_close(&self, index: usize) -> bool {
        // Implement verification logic
        if let Some(metadata) = self.window_metadata.get(&index) {
            if metadata.needs_verification {
                // Implement verification logic here
                // For now, always return true
                true
            } else {
                true
            }
        } else {
            true
        }
    }

    fn inform_associated_plugins(&self, index: usize) {
        if let Some(metadata) = self.window_metadata.get(&index) {
            for plugin in &metadata.associated_plugins {
                // Implement logic to inform associated plugins
                println!("Informing plugin {} about window {} closing", plugin, index);
            }
        }
    }
}

impl FramePlugin for WindowControllerPlugin {
    fn update(&self) {
        while let Ok(message) = self.receiver.try_recv() {
            match message {
                Message::CloseWindow(index) => {
                    if self.verify_close(index) {
                        self.inform_associated_plugins(index);
                        // Send message to AboutWindowPlugin to actually close the window
                        self.tx.send(Message::ConfirmedCloseWindow(index)).unwrap();
                    }
                },
                // Handle other messages...
                _ => {},
            }
        }
    }

    
 
    fn execute(&self, ui: &mut eframe::egui::Ui, ctx: &eframe::egui::Context) {
        //todo!()
    }
    
    fn handle_message(&self, message: Message) {
        //todo!()
    }
    
    fn is_dragging(&self) -> bool {
        todo!()
    }
}