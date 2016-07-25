use ecs::message::Message;

use std::collections::VecDequeue;


pub struct MessageQueue {
    messages: VecDequeue<Message>,
}

impl MessageQueue {
    pub fn new() -> Self {
        MessageQueue {
            messages: VecDequeue::new(),
        }
    }

//    pub fn enqueue(
}
