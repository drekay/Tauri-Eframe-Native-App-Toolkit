use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::sync::Mutex;
use crossbeam_channel::SendError;

use crate::message_handler::MessageHandler;
use crate::message_priority::Priority;
use crate::messages::Message;
use crate::MessageFilter;

//(crate)
pub struct PrioritizedMessage {
    message: Message,
    priority: Priority,
}

impl Ord for PrioritizedMessage {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for PrioritizedMessage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PrioritizedMessage {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for PrioritizedMessage {}

pub struct PriorityMessageHandler {
    queue: Mutex<BinaryHeap<PrioritizedMessage>>,
}

impl PriorityMessageHandler {
    pub fn new() -> Self {
        Self { queue: Mutex::new(BinaryHeap::new()) }
    }

    pub fn send_message_with_priority(&self, message: Message, priority: Priority) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(PrioritizedMessage { message, priority });
    }
}

impl MessageHandler for PriorityMessageHandler {
    fn send_message(&self, message: Message) -> Result<(), SendError<Message>> {
        let mut queue = self.queue.lock().unwrap();
        queue.push(PrioritizedMessage { message, priority: Priority::Medium });
        Ok(())
    }
    fn receive_message(&mut self) -> Option<Message> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop().map(|pm| pm.message)
    }

    fn receive_message_with_priority(&mut self, min_priority: Priority) -> Option<Message> {
        let mut queue = self.queue.lock().unwrap();
        if let Some(pm) = queue.peek() {
            if pm.priority >= min_priority {
                return queue.pop().map(|pm| pm.message);
            }
        }
        None
    }

    fn filter_messages(&mut self, filter: Box<dyn MessageFilter + Send + Sync>)  {
        todo!()
    }
    
    //let mut queue = self.queue.lock().unwrap();
    //*queue = queue.drain().filter(|pm| filter(&pm.message)).collect();

    /*fn filter_messages(&mut self, filter: &dyn MessageFilter) {
        let mut queue = self.queue.lock().unwrap();
        queue.retain(|pm| filter.filter(&pm.message));
    }*/

   
}