use std::{collections::HashMap, path::Path, sync::Arc};
use crate::{message_bus::MessageBus, Message, Plugin, UIPlugin};
use eframe::{egui, App};
use crossbeam_channel::{unbounded, Receiver, Sender};
use libloading::{Library, Symbol};
pub struct PluginSystem {
    //plugins: HashMap<String, Box<dyn Plugin>>,
    plugins: HashMap<String, (Box<dyn Plugin>, Arc<Library>)>,
    ui_plugins: HashMap<String, Box<dyn UIPlugin>>,
    message_bus: MessageBus,
    response_sender: Sender<Message>,
    response_receiver: Receiver<Message>,
}

impl PluginSystem {
    pub fn new() -> Self {
        let (response_sender, response_receiver) = unbounded();
        Self {
            plugins: HashMap::default(),
            ui_plugins: HashMap::default(),
            message_bus: MessageBus::new(),
            response_sender,
            response_receiver,
        }
    }

    pub fn load_plugin(&mut self, path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let lib = unsafe { Library::new(path)? };
        //let lib = Arc::new(unsafe { Library::new(path)? });
        let constructor: Symbol<fn() -> Box<dyn Plugin>> = unsafe { lib.get(b"create_plugin")? };
        let mut plugin = constructor();
        let name = plugin.name().to_string();
        plugin.on_load();

        self.plugins.insert(name, (plugin, lib.into()));
        Ok(())
    }

    /* no need?
    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        let name = plugin.name().to_string();
        self.plugins.insert(name, plugin);
    }*/

    pub fn add_ui_plugin(&mut self, plugin: Box<dyn UIPlugin>) {
        let name = plugin.name().to_string();
        self.ui_plugins.insert(name, plugin);
    }

     pub fn update_all(&mut self, ctx: &egui::Context) {
        let mut deferred_messages = Vec::new();

        //println!("Plugin-system update_all ...");

        // Update regular plugins
    for (_, (plugin, _)) in &mut self.plugins {
        plugin.update(ctx, &self.message_bus.receiver, &self.response_sender);
    }

        // Update UI plugins
        for (_, ui_plugin) in &mut self.ui_plugins {
           //add-this ui_plugin.update(ctx, &self.message_bus.receiver, &self.response_sender);
            ui_plugin.update_ui(ctx);
        }

        // Process messages in the main message bus
        while let Ok(message) = self.message_bus.receiver.try_recv() {
           // self.route_message(message); //todo
        }

        // Process responses from plugins
        while let Ok(response) = self.response_receiver.try_recv() {
            self.handle_response(response);
        }

        // Process deferred messages
        for message in deferred_messages {
            self.route_message(message);
        }
    }

    //todo
    fn route_message(&mut self, message: Message) {
        /*match &message {
            Message::Broadcast(content) => {
                // Broadcast the message to all plugins
                for plugin in self.plugins.values_mut() {
                    plugin.handle_message(message.clone(), &self.response_sender);
                }
                for ui_plugin in self.ui_plugins.values_mut() {
                    ui_plugin.handle_message(message.clone(), &self.response_sender);
                }
            },
            Message::WindowPlugin(window_message) => {
                if let Some(window_plugin) = self.plugins.get_mut("WindowPlugin") {
                    window_plugin.handle_message(Message::WindowPlugin(window_message.clone()), &self.response_sender);
                }
            },
            Message::WindowControllerPlugin(controller_message) => {
                if let Some(window_controller) = self.plugins.get_mut("WindowControllerPlugin") {
                    window_controller.handle_message(Message::WindowControllerPlugin(controller_message.clone()), &self.response_sender);
                }
            },
            // Add more message routing as needed
        }*/
    }

    //todo
    fn handle_response(&mut self, response: Message) {
        // Handle responses from plugins
        /*match &response {
            Message::WindowControllerPlugin(WindowControllerPluginMessage::WindowAdded) => {
                println!("Window added successfully");
                // Perform any necessary actions in response to the window being added
            },
            Message::WindowControllerPlugin(WindowControllerPluginMessage::WindowClosed(id)) => {
                println!("Window {} closed successfully", id);
                // Perform any necessary actions in response to the window being closed
            },
            // Add more response handling as needed
            _ => {}
        }*/
    }

    pub fn send_message(&mut self, message: Message) {
     //   self.message_bus.broadcast(message);
    }
}