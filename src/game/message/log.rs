use std::cmp;

use game::*;

pub type MessageLog = Vec<MessageType>;

pub fn message_log_tail(log: &MessageLog, count: usize) -> &[MessageType] {
    let mid = cmp::max(0, (log.len() as isize) - (count as isize)) as usize;
    &log[mid..]
}
