use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::sync::Mutex;
use crossbeam_channel::SendError;

use crate::message_handler::MessageHandler;
use crate::message_priority::Priority;
use crate::messages::{Message, PluginMessage};
use crate::MessageFilter;
pub struct PrioritizedMessage(pub Box<dyn PluginMessage>);

impl PartialEq for PrioritizedMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0.priority() == other.0.priority()
    }
}

impl Eq for PrioritizedMessage {}

impl PartialOrd for PrioritizedMessage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PrioritizedMessage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.priority().cmp(&other.0.priority())
    }
}