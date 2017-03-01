use game::{MessageType, Message};

pub trait Language {
    fn translate_repeated(&self, message_type: MessageType, repeated: usize, message: &mut Message);
    fn translate(&self, message_type: MessageType, message: &mut Message) {
        self.translate_repeated(message_type, 1, message);
    }
}
