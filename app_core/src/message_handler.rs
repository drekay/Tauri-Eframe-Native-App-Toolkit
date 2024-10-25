// app_core/src/message_handler.rs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crossbeam_channel::{unbounded, Receiver, SendError, Sender, TryRecvError};
use crate::messages::Message;
use crate::message_priority::Priority;
use crate::MessageFilter;
use std::thread;

/*old
pub trait MessageHandler: Send + Sync {
    fn send_message(&self, message: Message) -> Result<(), crossbeam_channel::SendError<Message>>;
    fn receive_message(&self) -> Option<Message>;
    fn receive_message_with_priority(&mut self, min_priority: Priority) -> Option<Message>;
    fn filter_messages<F>(&mut self, filter: F) where F: Fn(&Message) -> bool;
}*/

pub trait MessageHandler: Send + Sync {
    fn send_message(&self, message: Message) -> Result<(), SendError<Message>>;
    fn receive_message(&mut self) -> Option<Message>;
    fn receive_message_with_priority(&mut self, min_priority: Priority) -> Option<Message>;
    fn filter_messages(&mut self, filter: Box<dyn MessageFilter + Send + Sync>) ;
}
////
struct FilterWrapper {
    filter: Arc<Box<dyn MessageFilter + Send + Sync>>,
}

impl FilterWrapper {
    fn new(filter: Box<dyn MessageFilter + Send + Sync>) -> Self {
        FilterWrapper {
            filter: Arc::new(filter),
        }
    }

    fn filter(&self, message: &Message) -> bool {
        self.filter.filter(message)
    }
}

impl Clone for FilterWrapper {
    fn clone(&self) -> Self {
        FilterWrapper {
            filter: Arc::clone(&self.filter),
        }
    }
}
/// 
//Purpose: For testing and profiling
pub struct DefaultMessageHandler {
    senders: Arc<Mutex<HashMap<String, Sender<Message>>>>,
    receiver: Receiver<Message>,
}

impl DefaultMessageHandler {
    pub fn new(senders: Arc<Mutex<HashMap<String, Sender<Message>>>>, receiver: Receiver<Message>) -> Self {
        Self { senders, receiver }
    }
}

impl MessageHandler for DefaultMessageHandler {
    fn send_message(&self, message: Message) -> Result<(), SendError<Message>> {
        let senders = self.senders.lock().unwrap();
        for sender in senders.values() {
            sender.send(message.clone())?;
        }
        Ok(())
    }

    fn receive_message(&mut self) -> Option<Message> {
        loop {
            match self.receiver.try_recv() {
                Ok(message) => return Some(message),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return None,
            }
        }
        None
    }

    fn receive_message_with_priority(&mut self, min_priority: Priority) -> Option<Message> {
        // Implement priority-based receiving if needed
        self.receive_message()
    }

   /* //non-blocking only
    fn filter_messages(&mut self, filter: &dyn MessageFilter) {
        let (filtered_sender, filtered_receiver) = unbounded();
        
        // Process messages without blocking
        while let Ok(message) = self.receiver.try_recv() {
            if filter.filter(&message) {
                let _ = filtered_sender.send(message);
            }
        }

        // Replace the current receiver with the filtered receiver
        self.receiver = filtered_receiver;
        
        // Update senders with the new filtered sender
        let mut senders = self.senders.lock().unwrap();
        senders.insert("filtered".to_string(), filtered_sender);
    }*/

    
    fn filter_messages(&mut self, filter: Box<dyn MessageFilter + Send + Sync>) {
        let (filtered_sender, filtered_receiver) = unbounded();
        let receiver_clone = self.receiver.clone();

        // Create a thread-safe version of the filter
        let filter = FilterWrapper::new(filter);
        let filtered_sender_clone = filtered_sender.clone();

        // Spawn a new thread for ongoing background processing
        thread::spawn(move || {
            for message in receiver_clone.iter() {
                if filter.filter(&message) {
                    let _ = filtered_sender_clone.send(message);
                }
            }
        });

        // Replace the current receiver with the filtered receiver
        self.receiver = filtered_receiver;
        
        // Update senders with the new filtered sender
        let mut senders = self.senders.lock().unwrap();
        senders.insert("filtered".to_string(), filtered_sender);
    }


    //end
}