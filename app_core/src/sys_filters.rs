use std::any::Any;
use std::sync::Arc;
use crate::{messages::Message, MessageFilter, Priority, PluginType};
use crate::messages::PluginMessage;

struct WindowMessageFilter {
    min_priority: Priority,
}

impl MessageFilter for WindowMessageFilter {
    fn filter(&self, message: &Message) -> bool {
        match message {
            Message::Plugin { plugin_type, priority, .. } => {
                *plugin_type == PluginType::Window && *priority >= self.min_priority
            },
            _ => false,
        }
    }

    fn clone_box(&self) -> Box<dyn MessageFilter + Send + Sync> {
        Box::new(WindowMessageFilter {
            min_priority: self.min_priority,
        })
    }
}

struct UIMessageFilter {
    min_priority: Priority,
}

impl MessageFilter for UIMessageFilter {
    fn filter(&self, message: &Message) -> bool {
        match message {
            Message::Plugin { plugin_type, priority, .. } => {
                *plugin_type == PluginType::UI && *priority >= self.min_priority
            },
            _ => false,
        }
    }

    fn clone_box(&self) -> Box<dyn MessageFilter + Send + Sync> {
        Box::new(UIMessageFilter {
            min_priority: self.min_priority,
        })
    }
}

pub struct DynamicMessageFilter {
    controller_id: String,
}

impl DynamicMessageFilter {
    pub fn new(controller_id: String) -> Self {
        Self { controller_id }
    }

    pub fn update_controller_id(&mut self, new_controller_id: String) {
        self.controller_id = new_controller_id;
    }
}

impl MessageFilter for DynamicMessageFilter {
    fn filter(&self, message: &Message) -> bool {
        message.get_target_controller_id() == Some(&self.controller_id)
    }

    fn clone_box(&self) -> Box<dyn MessageFilter + Send + Sync> {
        Box::new(Self {
            controller_id: self.controller_id.clone(),
        })
    }
}