use std::cmp;

use game::*;

pub struct MessageLogEntry {
    pub message: MessageType,
    pub repeated: usize,
}

pub struct MessageLog {
    messages: Vec<MessageLogEntry>,
}

impl MessageLog {
    pub fn new() -> Self {
        MessageLog {
            messages: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn tail(&self, count: usize) -> &[MessageLogEntry] {
        let mid = cmp::max(0, (self.len() as isize) - (count as isize)) as usize;
        &self.messages[mid..]
    }

    pub fn add(&mut self, message: MessageType) {
        if let Some(ref mut entry) = self.messages.last_mut() {
            if message == entry.message {
                entry.repeated += 1;
                return;
            }
        }

        self.messages.push(MessageLogEntry {
            message: message,
            repeated: 1,
        });
    }
}
