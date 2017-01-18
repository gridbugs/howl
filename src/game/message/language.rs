use game::{MessageType, Message};

pub trait Language {
    fn translate(&self, message_type: MessageType, message: &mut Message);
}
