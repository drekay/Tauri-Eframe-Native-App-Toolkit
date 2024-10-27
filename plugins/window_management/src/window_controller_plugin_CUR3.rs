// plugins/window_management/src/window_controller_plugin.rs
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::sync::{Arc, RwLock};
use app_core::messages::PluginMessage;
use app_core::{MessageFilter, PrioritizedMessage};
use crossbeam_channel::{Sender, Receiver};
use app_core::{messages::{Message, WindowPluginMessage}, plugin_version, MessageHandler, Plugin, PluginType, PluginVersion, VersionComparable, VersionEquatable, Versioned};
use crate::window_controller_messages::{WindowControllerPluginMessage};
use eframe::egui;
use crossbeam_queue::ArrayQueue; 

pub struct WindowControllerPluginState {
    message_filters: Vec<Box<dyn MessageFilter + Send + Sync>>,
    settings: Arc<RwLock<ControllerSettings>>,
    filtered_out_sender: Sender<Message>,
    message_queue: Arc<ArrayQueue<PrioritizedMessage>>,
    messages_to_process: Arc<ArrayQueue<(PluginType, Box<dyn PluginMessage>)>>,
}

pub struct WindowControllerPlugin {
    message_handlers: HashMap<PluginType, PluginMessageHandler>,
    mutable_state: Arc<RwLock<WindowControllerPluginState>>,
}

pub struct ControllerSettings {
    active_handlers: HashSet<PluginType>,
}

type PluginMessageHandler = Box<dyn Fn(&dyn PluginMessage, &Arc<RwLock<WindowControllerPluginState>>) + Send + Sync>;

impl Plugin for WindowControllerPlugin {
    fn name(&self) -> &str {
        "WindowControllerPlugin"
    }

    fn update(&mut self, ctx: &egui::Context, message_handler: &mut dyn MessageHandler) {
        while let Some(message) = message_handler.receive_message() {
            let should_process = {
                let state = self.mutable_state.read().unwrap();
                state.message_filters.iter().all(|filter| filter.filter(&message))
            };

            if should_process {
                if let Some(plugin_message) = self.extract_plugin_message(&message) {
                    let prioritized_message = PrioritizedMessage(plugin_message);
                    let mut state = self.mutable_state.write().unwrap();
                    if state.message_queue.push(prioritized_message).is_err() {
                        eprintln!("Message queue is full, dropping message");
                    }
                }
            } else {
                let state = self.mutable_state.read().unwrap();
                if state.filtered_out_sender.send(message).is_err() {
                    eprintln!("Failed to send filtered-out message");
                }
            }
        }

        self.process_messages();
    }
    
    fn on_load(&mut self) {
        // Initialization logic
    }

    fn on_unload(&mut self) {
        // Cleanup logic
    }
    
    fn plugin_type(&self) -> app_core::PluginType {
        app_core::PluginType::WindowController
    }
    
    fn controller(&self) -> Option<&str> {
        None // This plugin is its own controller
    }
    
    fn is_enabled(&self) -> bool {
        true // Assuming it's always enabled, or implement logic as needed
    }
    
    fn set_enabled(&mut self, _enabled: bool) {
        // Implement if needed
    }
}

impl WindowControllerPlugin {
    pub fn new(filtered_out_sender: Sender<Message>) -> Self {
        let mutable_state = WindowControllerPluginState {
            message_filters: Vec::new(),
            settings: Arc::new(RwLock::new(ControllerSettings {
                active_handlers: HashSet::new(),
            })),
            filtered_out_sender,
            message_queue: Arc::new(ArrayQueue::new(1024)), // Adjust capacity as needed
            messages_to_process: Arc::new(ArrayQueue::new(1024)), // Adjust capacity as needed
        };

        Self {
            message_handlers: HashMap::new(),
            mutable_state: Arc::new(RwLock::new(mutable_state)),
        }
    }

    pub fn register_handler(&mut self, plugin_type: PluginType, handler: PluginMessageHandler) {
        self.message_handlers.insert(plugin_type, handler);
    }

    pub fn activate_handler(&self, plugin_type: PluginType) {
        let mut state = self.mutable_state.write().unwrap();
        let mut settings = state.settings.write().unwrap();
        settings.active_handlers.insert(plugin_type);
    }

    pub fn deactivate_handler(&self, plugin_type: PluginType) {
        let mut state = self.mutable_state.write().unwrap();
        let mut settings = state.settings.write().unwrap();
        settings.active_handlers.remove(&plugin_type);
    }

    fn process_messages(&mut self) {
        let messages_to_process = ArrayQueue::new(1024); // Adjust capacity as needed
        {
            let state = self.mutable_state.read().unwrap();
            while let Some(PrioritizedMessage(message)) = state.message_queue.pop() {
                if messages_to_process.push(message).is_err() {
                    eprintln!("messages_to_process queue is full, dropping message");
                    break;
                }
            }
        }

        while let Some(message) = messages_to_process.pop() {
            let plugin_type = message.plugin_type();
            if let Some(handler) = self.message_handlers.get(&plugin_type) {
                handler(&*message, &self.mutable_state);
            }
        }
    }

    fn extract_plugin_message(&self, message: &Message) -> Option<Box<dyn PluginMessage>> {
        match message {
            Message::Plugin { plugin_type, message, priority } => {
                // Try to downcast the Any to concrete PluginMessage types
                if let Some(window_message) = message.downcast_ref::<WindowPluginMessage>() {
                    Some(Box::new(window_message.clone()))
                } else if let Some(controller_message) = message.downcast_ref::<WindowControllerPluginMessage>() {
                    Some(Box::new(controller_message.clone()))
                } else {
                    // If it's neither, we can't handle it as a PluginMessage
                    None
                }
            },
            Message::Broadcast { content, priority } => {
                // Handle broadcast messages if needed, or filter them out
               //addback let _ = self.filtered_out_sender.send(message.clone());
                None
            },
            Message::CriticalData(payload) => {
                // Try to downcast the Any to a concrete PluginMessage type
                if let Some(critical_message) = payload.downcast_ref::<WindowPluginMessage>() {
                    Some(Box::new(critical_message.clone()))
                } else if let Some(critical_message) = payload.downcast_ref::<WindowControllerPluginMessage>() {
                    Some(Box::new(critical_message.clone()))
                } else {
                    // If it's neither, we can't handle it as a PluginMessage
                    None
                }
            },
            Message::Custom(custom_data) => {
                // Handle custom messages if needed, or filter them out
                //addback let _ = self.filtered_out_sender.send(message.clone());
                None
            },
        }
    }
 
    
 }

 // Add this struct to represent the rest of WindowControllerPlugin
pub struct WindowControllerPluginRest<'a> {
    pub message_filters: &'a mut Vec<Box<dyn MessageFilter + Send + Sync>>,
    pub settings: &'a mut Arc<RwLock<ControllerSettings>>,
    pub filtered_out_sender: &'a mut Sender<Message>,
    pub message_queue: &'a mut Arc<ArrayQueue<PrioritizedMessage>>,
    pub messages_to_process: &'a mut Arc<ArrayQueue<(PluginType, Box<dyn PluginMessage>)>>,
}