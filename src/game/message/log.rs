use std::cmp;

use game::*;

#[derive(Debug)]
pub struct MessageLogEntry {
    pub message: MessageType,
    pub repeated: usize,
}

pub struct MessageLog {
    messages: Vec<MessageLogEntry>,
    last_temporary: bool,
}

impl MessageLog {
    pub fn new() -> Self {
        MessageLog {
            messages: Vec::new(),
            last_temporary: false,
        }
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn tail(&self, count: usize) -> &[MessageLogEntry] {
        let mid = cmp::max(0, (self.len() as isize) - (count as isize)) as usize;
        &self.messages[mid..]
    }

    pub fn tail_with_offset(&self, count: usize, offset: usize) -> &[MessageLogEntry] {
        let end = cmp::max(0, (self.len() as isize) - (offset as isize)) as usize;
        let start = cmp::max(0, (end as isize) - (count as isize)) as usize;
        &self.messages[start..end]
    }

    pub fn add(&mut self, message: MessageType) {

        if self.last_temporary {
            self.messages.pop();
        }

        self.last_temporary = false;

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

    pub fn add_temporary(&mut self, message: MessageType) {

        if self.last_temporary {
            self.messages.pop();
        }

        self.last_temporary = true;

        self.messages.push(MessageLogEntry {
            message: message,
            repeated: 1,
        });
    }
}
