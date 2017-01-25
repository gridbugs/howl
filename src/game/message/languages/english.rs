use game::*;
use colour::*;

pub struct English;

impl English {
    fn translate_you_see(&self, name: YouSeeMessageType, message: &mut Message) {
        match name {
            YouSeeMessageType::Player => {
                message.push(MessagePart::plain("Yourself"));
            }
            YouSeeMessageType::Tree => {
                message.push(MessagePart::plain("A tree"));
            }
        }
    }

    fn translate_action(&self, action: ActionMessageType, message: &mut Message) {
        match action {
            ActionMessageType::PlayerOpenDoor => {
                message.push(MessagePart::plain("You open the door."));
            }
            ActionMessageType::PlayerCloseDoor => {
                message.push(MessagePart::plain("You close the door."));
            }
        }
    }
}

impl Language for English {
    fn translate_repeated(&self, message_type: MessageType, repeated: usize, message: &mut Message) {

        message.clear();

        match message_type {
            MessageType::Empty => {},
            MessageType::Intro => intro_message(message),
            MessageType::Welcome => {
                message.push(MessagePart::plain("Welcome to "));
                message.push(MessagePart::colour(colours::PURPLE, "HOWL"));
                message.push(MessagePart::plain("!"));
            }
            MessageType::Action(action) => {
                self.translate_action(action, message);
            }
            MessageType::YouSee(name) => {
                message.push(MessagePart::plain("You see: "));
                if let Some(name) = name {
                    self.translate_you_see(name, message);
                }
            }
        }

        if repeated > 1 {
            message.push(MessagePart::Text(TextMessagePart::Plain(format!("(x{})", repeated))));
        }
    }
}

fn intro_message(message: &mut Message) {
    message.push(MessagePart::plain("Everything beneath the moonlight appears different. "));
    message.push(MessagePart::plain("An arcane tome is rumored to be hidden somewhere in the forest. "));
    message.push(MessagePart::plain("Perhaps the answers lie within."));
    message.push(MessagePart::Newline);
    message.push(MessagePart::Newline);
    message.push(MessagePart::plain("Press a key to begin..."));
}
