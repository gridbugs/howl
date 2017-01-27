use game::{MessageType, Message, ControlMap};

pub trait Language {
    fn translate_repeated(&self, message_type: MessageType, repeated: usize, message: &mut Message);
    fn translate(&self, message_type: MessageType, message: &mut Message) {
        self.translate_repeated(message_type, 1, message);
    }
    fn translate_controls(&self, control_map: &ControlMap, message: &mut Message);
}
