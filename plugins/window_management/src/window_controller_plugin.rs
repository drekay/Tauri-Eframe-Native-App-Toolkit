use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::sync::{Arc, RwLock};
use app_core::messages::{ControllerMessage, Message, PluginManifest, PluginMessage, WindowControllerMessage, WindowPluginMessage};
use app_core::{MessageFilter, PrioritizedMessage, Priority};
use crossbeam_channel::{Sender, Receiver};
use app_core::{MessageHandler, Plugin, PluginType};
use crate::window_controller_messages::WindowControllerPluginMessage;
use eframe::egui;
use crossbeam_queue::ArrayQueue;
use std::any::Any;

pub struct WindowControllerPluginState {
    message_filters: Arc<ArrayQueue<Box<dyn MessageFilter + Send + Sync>>>,
    settings: Arc<RwLock<ControllerSettings>>,
    filtered_out_sender: Sender<Message>,
    message_queue: Arc<ArrayQueue<PrioritizedMessage>>,
    messages_to_process: Arc<ArrayQueue<Message>>,
}

pub struct WindowControllerPlugin {
    message_handlers: HashMap<PluginType, PluginMessageHandler>,
    mutable_state: Arc<RwLock<WindowControllerPluginState>>,
    manifest: PluginManifest,
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
            self.handle_message(&message);
        }

        self.process_messages();
    }

    fn plugin_type(&self) -> PluginType {
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
    
    fn on_load(&mut self) {
        todo!()
    }
    
    fn on_unload(&mut self) {
        todo!()
    }
    
    // ... other Plugin trait methods ...
}

impl WindowControllerPlugin {
    fn handle_message(&mut self, message: &Message) {
        match message {
            Message::Plugin { plugin_type, message, priority } => {
                if let Some(plugin_message) = self.extract_plugin_message(message) {
                    let prioritized_message = PrioritizedMessage(plugin_message);
                    let mut state = self.mutable_state.write().unwrap();
                    if state.message_queue.push(prioritized_message).is_err() {
                        eprintln!("Message queue is full, dropping message");
                    }
                }
            },
            Message::ControllerMessage { target_controller_id, content, priority } => {
                if target_controller_id == &self.manifest.controller_id {
                    self.process_controller_message(content.clone(), *priority);
                } else {
                    self.forward_controller_message(target_controller_id, content, *priority);
                }
            },
            Message::PluginSpecific { content, priority } => {
                if let Some(window_msg) = content.as_any().downcast_ref::<WindowControllerPluginMessage>() {
                    self.process_window_controller_message(window_msg, *priority);
                } else {
                    self.forward_plugin_specific_message(content.clone(), *priority);
                }
            },
            Message::Whatever { content, priority } => {
                self.process_whatever_message(content.clone(), *priority);
            },
            Message::Broadcast { content, priority } => {
                println!("Received broadcast: {} with priority {:?}", content, priority);
                self.process_broadcast(content.to_string(), *priority);
            },
            Message::ControllerUpdated { controller_id, priority } => {
                println!("Controller updated: {} with priority {:?}", controller_id, priority);
                self.handle_controller_update(controller_id.to_string(), *priority);
            },
            Message::CriticalData { payload, priority } => {
                if let Some(plugin_message) = self.extract_plugin_message(payload) {
                    self.process_critical_message(plugin_message, *priority);
                }
            },
            Message::FilteredOut { original_message, priority } => {
                println!("Message filtered out with priority {:?}", priority);
                self.handle_filtered_out_message(original_message, *priority);
            },
        }
    }

    /*fn extract_plugin_message(&self, message: &dyn std::any::Any) -> Option<Box<dyn PluginMessage>> {
        if let Some(window_message) = message.downcast_ref::<WindowPluginMessage>() {
            Some(Box::new(window_message.clone()))
        } else if let Some(controller_message) = message.downcast_ref::<WindowControllerPluginMessage>() {
            Some(Box::new(controller_message.clone()))
        } else {
            None
        }
    }*/


    fn process_messages(&mut self) {
        let messages_to_process = ArrayQueue::new(1024);
        {
            let state = self.mutable_state.read().unwrap();
            while let Some(PrioritizedMessage(message)) = state.message_queue.pop() {
                if messages_to_process.push(message).is_err() {
                    eprintln!("messages_to_process queue is full, dropping message");
                    break;
                }
            }
        }
    
        while let Some(plugin_message) = messages_to_process.pop() {
            let plugin_type = plugin_message.plugin_type();
            if let Some(handler) = self.message_handlers.get(&plugin_type) {
                handler(&*plugin_message, &self.mutable_state);
            } else {
                eprintln!("No handler found for plugin type: {:?}", plugin_type);
            }
        }
    }

   
   //////
   fn process_controller_message(&mut self, content: ControllerMessage, priority: Priority) {
    match content {
        ControllerMessage::WindowController(window_msg) => {
            match window_msg {
                WindowControllerMessage::RequestCloseWindow { window_index, priority } => {
                    self.process_request_close_window(window_index, priority);
                },
                WindowControllerMessage::RequestAddWindow {priority} => {
                    self.process_request_add_window(priority);
                },
                // Add other WindowControllerMessage variants if needed in the future
            }
        },
        // Add other controller message types here
    }
}

// Helper methods to process specific window controller messages
fn process_request_close_window(&mut self, window_index: usize, priority: Priority) {
    println!("Processing request to close window {} with priority {:?}", window_index, priority);
    // Implement logic to close the window
    // For example, you might want to send a message to the WindowPlugin to confirm the window closure
    let close_message = Message::PluginSpecific {
        content: Arc::new(WindowPluginMessage::ConfirmedCloseWindow(window_index)),
        priority,
    };
    if let Err(e) = self.mutable_state.read().unwrap().filtered_out_sender.send(close_message) {
        eprintln!("Failed to send window close confirmation: {:?}", e);
    }
}

fn process_request_add_window(&mut self, priority: Priority) {
    println!("Processing request to add a new window with priority {:?}", priority);
    // Implement logic to add a new window
    // For example, you might want to send a message to the WindowPlugin to add a new window
    let add_message = Message::PluginSpecific {
        content: Arc::new(WindowPluginMessage::AddWindow),
        priority,
    };
    if let Err(e) = self.mutable_state.read().unwrap().filtered_out_sender.send(add_message) {
        eprintln!("Failed to send add window request: {:?}", e);
    }
}
   /// 

    fn forward_controller_message(&self, target_controller_id: &str, content: &ControllerMessage, priority: Priority) {
        let message = Message::ControllerMessage {
            target_controller_id: target_controller_id.to_string(),
            content: content.clone(),
            priority,
        };
        if let Err(e) = self.mutable_state.read().unwrap().filtered_out_sender.send(message) {
            eprintln!("Failed to forward controller message: {:?}", e);
        }
    }

    fn process_window_controller_message(&mut self, msg: &WindowControllerPluginMessage, priority: Priority) {
        match msg {
            WindowControllerPluginMessage::RequestCloseWindow { window_id } => {
                // Logic to request window closure
                println!("Requesting to close window {}", window_id);
                // You might want to send a message to the WindowPlugin here
            },
            WindowControllerPluginMessage::RequestAddWindow => {
                // Logic to request adding a new window
                println!("Requesting to add a new window");
                // You might want to send a message to the WindowPlugin here
            },
            WindowControllerPluginMessage::AddWindow { title, content, plugin_type } => todo!(),
            WindowControllerPluginMessage::CloseWindow { window_id } => todo!(),
            WindowControllerPluginMessage::MinimizeWindow { window_id } => todo!(),
            WindowControllerPluginMessage::MaximizeWindow { window_id } => todo!(),
            WindowControllerPluginMessage::DragWindowStart { window_id, position } => todo!(),
            WindowControllerPluginMessage::DragWindowMove { window_id, position } => todo!(),
            WindowControllerPluginMessage::DragWindowEnd { window_id, final_position } => todo!(),
            WindowControllerPluginMessage::WindowAdded { window_id } => todo!(),
            WindowControllerPluginMessage::WindowClosed { window_id } => todo!(),
            WindowControllerPluginMessage::UpdateWindowContent { window_id, new_content } => todo!(),
            WindowControllerPluginMessage::RequestWindowList => todo!(),
            // Handle other WindowControllerPluginMessage variants...
        }
    }

    fn forward_plugin_specific_message(&self, content: Arc<dyn PluginMessage>, priority: Priority) {
        let message = Message::PluginSpecific {
            content,
            priority,
        };
        if let Err(e) = self.mutable_state.read().unwrap().filtered_out_sender.send(message) {
            eprintln!("Failed to forward plugin-specific message: {:?}", e);
        }
    }

    fn process_whatever_message(&mut self, content: Arc<dyn Any + Send + Sync>, priority: Priority) {
        // Process arbitrary data
        println!("Processing 'Whatever' message with priority {:?}", priority);
        // You might want to check for specific types and handle them accordingly
    }

    fn process_broadcast(&mut self, content: String, priority: Priority) {
        println!("Processing broadcast: {} with priority {:?}", content, priority);
        // Implement broadcast handling logic
    }

    fn handle_controller_update(&mut self, controller_id: String, priority: Priority) {
        println!("Handling controller update: {} with priority {:?}", controller_id, priority);
        // Implement logic to update internal state or notify other components
    }

    fn process_critical_message(&mut self, message: Box<dyn PluginMessage>, priority: Priority) {
        println!("Processing critical message with priority {:?}", priority);
        // Implement critical message processing logic
        // You might want to interrupt other operations or handle this message immediately
    }

    fn handle_filtered_out_message(&mut self, original_message: &Message, priority: Priority) {
        println!("Handling filtered out message with priority {:?}", priority);
        // Implement logic to handle filtered out messages
        // You might want to log these messages or take specific actions
    }

    fn extract_plugin_message(&self, message: &dyn Any) -> Option<Box<dyn PluginMessage>> {
        if let Some(window_message) = message.downcast_ref::<WindowPluginMessage>() {
            Some(Box::new(window_message.clone()))
        } else if let Some(controller_message) = message.downcast_ref::<WindowControllerPluginMessage>() {
            Some(Box::new(controller_message.clone()))
        } else {
            None
        }
    }
    
}

// Assuming you have a CustomData type for Whatever messages
#[derive(Debug)]
struct CustomData {
    // Define your custom data fields here
}