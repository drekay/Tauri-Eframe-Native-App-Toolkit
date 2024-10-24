use crossbeam_channel::{bounded, Sender, Receiver};
use crate::{Message, MessageHandler};

pub struct MessageSystem {
    senders: Vec<Sender<Message>>,
    receivers: Vec<Receiver<Message>>,
}

impl MessageSystem {
    pub fn new(channel_count: usize) -> Self {
        let mut senders = Vec::with_capacity(channel_count);
        let mut receivers = Vec::with_capacity(channel_count);

        for _ in 0..channel_count {
            let (tx, rx) = bounded(1024); // Adjust buffer size as needed
            senders.push(tx);
            receivers.push(rx);
        }

        Self { senders, receivers }
    }

    pub fn create_handler(&self, index: usize) -> Box<dyn MessageHandler> {
        Box::new(ChannelMessageHandler {
            sender: self.senders[index].clone(),
            receiver: self.receivers[index].clone(),
        })
    }
}

struct ChannelMessageHandler {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl MessageHandler for ChannelMessageHandler {
    fn send_message(&mut self, message: Message) {
        let _ = self.sender.try_send(message);
    }

    fn receive_message(&mut self) -> Option<Message> {
        self.receiver.try_recv().ok()
    }
}