use std::{collections::HashMap, path::Path, sync::{Arc, Mutex}};
use crate::{message_bus::MessageBus, message_handler::DefaultMessageHandler, messages::{Message, PluginControlMessage, SystemMessage}, MessageHandler, Plugin, UIPlugin};
use eframe::{egui, App};
use crossbeam_channel::{unbounded, Receiver, Sender};
use libloading::{Library, Symbol};
pub struct PluginSystem {
    plugins: HashMap<String, (Box<dyn Plugin>, Arc<Library>)>,
    ui_plugins: HashMap<String, Box<dyn UIPlugin>>,
    message_bus: MessageBus,
    response_sender: Sender<Message>,
    response_receiver: Receiver<Message>,
    message_handler: Box<dyn MessageHandler>,
    
}

impl PluginSystem {
    pub fn new() -> Self {
        let message_bus = MessageBus::new();
        let (response_sender, response_receiver) = crossbeam_channel::unbounded();

        Self {
            plugins: HashMap::default(),
            ui_plugins: HashMap::default(),
            message_bus: message_bus.clone(),
            response_sender,
            response_receiver,
            message_handler: Box::new(DefaultMessageHandler::new(
                Arc::new(Mutex::new(message_bus.senders)),
                message_bus.receiver
            )),
        }
    }

    pub fn load_plugin(&mut self, path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let lib = unsafe { Library::new(path)? };
        let constructor: Symbol<fn() -> Box<dyn Plugin>> = unsafe { lib.get(b"create_plugin")? };
        let mut plugin = constructor();
        let name = plugin.name().to_string();
        plugin.on_load();

        self.plugins.insert(name, (plugin, lib.into()));
        Ok(())
    }
  
    fn handle_system_message(&mut self, message: Message) {
     /* ADDBACK 
       match message {
            Message::System(system_message) => {
                match system_message {
                    SystemMessage::ShutdownPlugin(plugin_name) => {
                        if let Some((plugin, _)) = self.plugins.get_mut(&plugin_name) {
                            plugin.on_unload();
                            // Remove the plugin from the system
                            self.plugins.remove(&plugin_name);
                        }
                    },
                    SystemMessage::ReloadPlugin(plugin_name) => {
                        if let Some((plugin, library)) = self.plugins.get_mut(&plugin_name) {
                            plugin.on_unload();
                            // Reload the plugin
                            // This is a simplified version; you might need to handle errors and re-creation of the plugin
                            let create_plugin: Symbol<fn() -> Box<dyn Plugin>> = unsafe { library.get(b"create_plugin") }.unwrap();
                            let new_plugin = create_plugin();
                            *plugin = new_plugin;
                            plugin.on_load();
                        }
                    },
                    SystemMessage::LogMessage(level, message) => {
                        // Handle logging
                        println!("[{}] {}", level, message);
                    },
                    SystemMessage::PerformanceMetric(metric_name, value) => {
                        // Handle performance metric
                        println!("Performance Metric - {}: {}", metric_name, value);
                    },
                    SystemMessage::Update => todo!(),
                    // Handle other system messages...
                }
            }, Message::PluginControl(control_message) => {
                // Handle plugin-specific control messages
                let plugin_name = match &control_message {
                    PluginControlMessage::Enable(name) => name,
                    PluginControlMessage::Disable(name) => name,
                    PluginControlMessage::Configure(name, _) => name,
                };
                if let Some((plugin, _)) = self.plugins.get_mut(plugin_name) {
                    plugin.handle_message(Message::PluginControl(control_message), self.message_handler.as_mut());
                }
            },
            // Handle other message types that require system-level processing
            _ => {
                // Forward unhandled messages to the appropriate controller plugin
                self.forward_to_controller(message);
            }
        }
   
   */
    }

    fn forward_to_controller(&mut self, message: Message) {
        // Logic to determine which controller should handle the message
        // and forward it to that controller
        // This is a simplified version; you might need more sophisticated routing
        /*ADDBACK if let Some((controller, _)) = self.plugins.get_mut("UIControllerPlugin") {
            controller.handle_message(message, &mut self.message_handler);
        }*/
    }
}