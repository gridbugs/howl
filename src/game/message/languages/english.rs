use game::*;
use colour::*;

pub struct English;

impl Language for English {
    fn translate_repeated(&self, message_type: MessageType, repeated: usize, message: &mut Message) {

        message.clear();

        match message_type {
            MessageType::Welcome => {
                message.push(MessagePart::Plain("Welcome to ".to_string()));
                message.push(MessagePart::Colour(colours::PURPLE, "HOWL".to_string()));
                message.push(MessagePart::Plain("!".to_string()));
            }
            MessageType::PlayerOpenDoor => {
                message.push(MessagePart::Plain("You open the door.".to_string()));
            }
            MessageType::PlayerCloseDoor => {
                message.push(MessagePart::Plain("You close the door.".to_string()));
            }
        }

        if repeated > 1 {
            message.push(MessagePart::Plain(format!("(x{})", repeated)));
        }
    }
}
