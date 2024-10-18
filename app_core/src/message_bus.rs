use std::collections::HashMap;
use crossbeam_channel::{unbounded, Receiver, Sender};


use crate::Message;

pub struct MessageBus {
    pub senders: HashMap<String, Sender<Message>>,
    pub receiver: Receiver<Message>,
}

impl MessageBus {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Self {
            senders: HashMap::default(),
            receiver: rx,
        }
    }

    pub fn register_plugin(&mut self, plugin_name: &str) -> Receiver<Message> {
        let (tx, rx) = unbounded();
        self.senders.insert(plugin_name.to_string(), tx);
        rx
    }

    pub fn send(&self, target: &str, message: Message) {
        if let Some(sender) = self.senders.get(target) {
            sender.send(message).unwrap();
        }
    }

    pub fn broadcast(&self, message: Message) {
        for sender in self.senders.values() {
            sender.send(message.clone()).unwrap();
        }
    }
}