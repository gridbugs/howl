use ecs::message::Message;

use std::collections::VecDeque;

#[derive(Debug)]
pub struct MessageQueue {
    messages: VecDeque<Message>,
}

impl MessageQueue {
    pub fn new() -> Self {
        MessageQueue {
            messages: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, message: Message) {
        self.messages.push_back(message);
    }

    pub fn dequeue(&mut self) -> Option<Message> {
        self.messages.pop_front()
    }

    pub fn is_empty(&self) -> bool { self.messages.is_empty() }
}
