use crate::{messages::Message, MessageFilter, Priority};

struct WindowMessageFilter {
    min_priority: Priority,
}

impl MessageFilter for WindowMessageFilter {
    fn filter(&self, message: &Message) -> bool {
        match message {
            Message::WindowPlugin { priority, .. } => *priority >= self.min_priority,
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
        /*ADBACK - create UI enums
            match message {
            Message::UIPlugin { priority, .. } => *priority >= self.min_priority,
            _ => false,
        }*/
        false
    }

    fn clone_box(&self) -> Box<dyn MessageFilter + Send + Sync> {
        Box::new(UIMessageFilter {
            min_priority: self.min_priority,
        })
    }
}