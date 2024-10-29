
use std::any::Any;
use std::sync::Arc;
use crate::{messages::Message, MessageFilter, Priority, PluginType};


/*use crate::{messages::Message, MessageFilter, Priority};

pub struct MyConcreteFilter;

impl MessageFilter for MyConcreteFilter {
    fn filter(&self, message: &Message) -> bool {
        match message {
            Message::Broadcast { priority, .. }
            | Message::WindowPlugin { priority, .. }
            | Message::WindowControllerPlugin { priority, .. }
            | Message::Sample { priority, .. }
            | Message::Menu { priority, .. }
            | Message::System { priority, .. }
            | Message::PluginControl { priority, .. }
            | Message::ControllerPlugin { priority, .. } => matches!(priority, Priority::High | Priority::Critical),
            _ => false, // Default case if no other matches
        }
    }

    fn clone_box(&self) -> Box<dyn MessageFilter + Send + Sync> {
        Box::new(MyConcreteFilter)
    }
}*/