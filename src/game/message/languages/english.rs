use game::*;
use colour::*;

pub struct English;

impl Language for English {
    fn translate(&self, message_type: MessageType, message: &mut Message) {

        message.clear();

        match message_type {
            MessageType::Welcome => {
                message.push(MessagePart::Plain("Welcome to "));
                message.push(MessagePart::Colour(colours::PURPLE, "HOWL"));
                message.push(MessagePart::Plain("!"));
            }
        }
    }
}
