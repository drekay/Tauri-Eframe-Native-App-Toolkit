use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::sync::{Arc, RwLock};
use app_core::messages::PluginMessage;
use app_core::{MessageFilter, PrioritizedMessage};
use crossbeam_channel::{Sender, Receiver};
use app_core::{messages::{Message, WindowPluginMessage}, plugin_version, MessageHandler, Plugin, PluginType, PluginVersion, VersionComparable, VersionEquatable, Versioned};
use crate::window_controller_messages::{WindowControllerPluginMessage};

pub struct WindowControllerPlugin {
    message_handlers: HashMap<PluginType, PluginMessageHandler>,
    message_filters: Vec<Box<dyn MessageFilter + Send + Sync>>,
    settings: Arc<RwLock<ControllerSettings>>,
    filtered_out_sender: Sender<Message>,
}

pub struct ControllerSettings {
    active_handlers: HashSet<PluginType>,
}

type PluginMessageHandler = Box<dyn Fn(&dyn PluginMessage, &mut dyn Plugin) + Send + Sync>;
//METHODS
impl Plugin for WindowControllerPlugin {
    fn name(&self) -> &str {
        "WindowControllerPlugin"
    }

    fn update(&mut self, ctx: &egui::Context, message_handler: &mut dyn MessageHandler) {
        while let Some(message) = message_handler.receive_message() {
            /*match message {
                Message::UIViewPlugin(ui_message) => self.handle_ui_view_message(ui_message),
                Message::WindowPlugin(window_message) => {
                    // Handle any responses from WindowPlugin if necessary
                },
                // Handle other relevant message types...
                _ => {} // Ignore messages not relevant to this plugin
            }*/
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
    
}

impl WindowControllerPlugin {
    pub fn new(filtered_out_sender: Sender<Message>) -> Self {
        Self {
            message_handlers: HashMap::new(),
            message_filters: Vec::new(),
            settings: Arc::new(RwLock::new(ControllerSettings {
                active_handlers: HashSet::new(),
            })),
            filtered_out_sender,
        }
    }

    pub fn register_handler(&mut self, plugin_type: PluginType, handler: PluginMessageHandler) {
        self.message_handlers.insert(plugin_type, handler);
    }
    pub fn activate_handler(&self, plugin_type: PluginType) {
        let mut settings = self.settings.write().unwrap();
        settings.active_handlers.insert(plugin_type);
    }

    pub fn deactivate_handler(&self, plugin_type: PluginType) {
        let mut settings = self.settings.write().unwrap();
        settings.active_handlers.remove(&plugin_type);
    }

    pub fn update(&mut self, ctx: &egui::Context, message_handler: &mut dyn MessageHandler) {
        let mut priority_queue: BinaryHeap<Reverse<PrioritizedMessage>> = BinaryHeap::new();

        while let Some(message) = message_handler.receive_message() {
            if self.message_filters.iter().all(|filter| filter.filter(&message)) {
                if let Some(plugin_message) = self.extract_plugin_message(&message) {
                    priority_queue.push(Reverse(PrioritizedMessage(plugin_message)));
                }
            } else {
                if let Err(e) = self.filtered_out_sender.send(message) {
                    eprintln!("Failed to send filtered-out message: {:?}", e);
                }
            }
        }

        let settings = self.settings.read().unwrap();
        while let Some(Reverse(PrioritizedMessage(message))) = priority_queue.pop() {
            let plugin_type = message.plugin_type();

            if settings.active_handlers.contains(&plugin_type) {
                if let Some(handler) = self.message_handlers.get(&plugin_type) {
              //      cannot borrow `*self` as mutable because it is also borrowed as immutable
                 //   mutable borrow occurs here
                    handler(&*message, self);
                }
            }
        }
    }

    fn extract_plugin_message(&self, message: &Message) -> Option<Box<dyn PluginMessage>> {
        match message {
            Message::WindowPlugin { message, priority } => Some(Box::new(message.clone())),
            Message::WindowControllerPlugin { message, priority } => {
                if let Some(plugin_message) = message.as_plugin_message() {
                    Some(Box::new(plugin_message))
                } else {
                    // Send to filtered-out channel
                    let _ = self.filtered_out_sender.send(message.clone());
                    None
                }
            },
            Message::Sample { message, priority } => 
                message.as_plugin_message().map(Box::new)
                    .or_else(|| {
                        let _ = self.filtered_out_sender.send(message.clone());
                        None
                    }),
            // ... handle other message types similarly ...
            Message::Broadcast { content, priority } => {
                // Send broadcast messages to filtered-out channel
                let _ = self.filtered_out_sender.send(message.clone());
                None
            },
            Message::CriticalData(payload) => 
                payload.as_plugin_message().map(Box::new)
                    .or_else(|| {
                        let _ = self.filtered_out_sender.send(message.clone());
                        None
                    }),
            Message::Menu { message, priority } => todo!(),
            Message::System { message, priority } => todo!(),
            Message::PluginControl { message, priority } => todo!(),
            Message::ControllerPlugin { message, priority } => todo!(),
        }
    }
}
//
