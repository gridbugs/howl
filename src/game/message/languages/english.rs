use game::*;
use colour::*;

pub struct English;

impl English {
    fn translate_you_see(&self, name: NameMessageType, message: &mut Message) {
        match name {
            NameMessageType::Player => {
                message.push(MessagePart::Plain("Yourself".to_string()));
            }
            NameMessageType::Tree => {
                message.push(MessagePart::Plain("A tree".to_string()));
            }
        }
    }
}

impl Language for English {
    fn translate_repeated(&self, message_type: MessageType, repeated: usize, message: &mut Message) {

        message.clear();

        match message_type {
            MessageType::Empty => {},
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
            MessageType::YouSee(name) => {
                message.push(MessagePart::Plain("You see: ".to_string()));
                if let Some(name) = name {
                    self.translate_you_see(name, message);
                }
            }
        }

        if repeated > 1 {
            message.push(MessagePart::Plain(format!("(x{})", repeated)));
        }
    }
}
