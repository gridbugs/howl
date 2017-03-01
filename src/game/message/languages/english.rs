use game::*;
use colour::*;
use direction::*;

pub struct English;

impl English {
    fn translate_you_see(&self, name: YouSeeMessageType, message: &mut Message) {
        match name {
            YouSeeMessageType::Player => {
                message.push(MessagePart::plain("Myself"));
            }
            YouSeeMessageType::Tree => {
                message.push(MessagePart::plain("A tree"));
            }
        }
    }

    fn translate_action(&self, action: ActionMessageType, message: &mut Message) {
        match action {
            ActionMessageType::PlayerOpenDoor => {
                message.push(MessagePart::plain("I open the door."));
            }
            ActionMessageType::PlayerCloseDoor => {
                message.push(MessagePart::plain("I close the door."));
            }
        }
    }

    fn translate_description(&self, description: DescriptionMessageType, message: &mut Message) {
        match description {
            DescriptionMessageType::Player => {
                message.push(MessagePart::plain("I entered the forest in search of an ancient tome."));
            }
        }
    }

    fn translate_menu(&self, menu_message: MenuMessageType, message: &mut Message) {
        match menu_message {
            MenuMessageType::NewGame => {
                message.push(MessagePart::plain("New Game"));
            }
            MenuMessageType::Quit => {
                message.push(MessagePart::plain("Quit"));
            }
            MenuMessageType::Continue => {
                message.push(MessagePart::plain("Continue"));
            }
            MenuMessageType::SaveAndQuit => {
                message.push(MessagePart::plain("Save and Quit"));
            }
            MenuMessageType::Controls => {
                message.push(MessagePart::plain("Controls"));
            }
            MenuMessageType::Control(input, control) => {
                if let Some(input_message) = self.translate_input_event(input) {
                    self.translate_control(control, message);
                    message.push(MessagePart::plain(": "));
                    message.push(input_message);
                }
            }
            MenuMessageType::UnboundControl(control) => {
                self.translate_control(control, message);
                message.push(MessagePart::plain(": (unbound)"));
            }
            MenuMessageType::ControlBinding(control) => {
                self.translate_control(control, message);
                message.push(MessagePart::plain(": press a key..."));
            }
        }
    }

    fn translate_input_event(&self, input_event: InputEvent) -> Option<MessagePart> {
        let message_part = match input_event {
            InputEvent::Char(ch) => MessagePart::Text(TextMessagePart::Plain(format!("{}", ch))),
            InputEvent::Up => MessagePart::plain("up"),
            InputEvent::Down => MessagePart::plain("down"),
            InputEvent::Left => MessagePart::plain("left"),
            InputEvent::Right => MessagePart::plain("right"),
            InputEvent::Escape => MessagePart::plain("esc"),
            InputEvent::Return => MessagePart::plain("return"),
            InputEvent::Space => MessagePart::plain("space"),
            InputEvent::Quit => return None,
        };

        Some(message_part)
    }

    fn translate_direction(&self, direction: Direction) -> &'static str {
        match direction {
            Direction::North => "north",
            Direction::South => "south",
            Direction::East => "east",
            Direction::West => "west",
            Direction::NorthWest => "northwest",
            Direction::SouthWest => "southwest",
            Direction::NorthEast => "northeast",
            Direction::SouthEast => "southeast",
        }
    }

    fn translate_control(&self, control: Control, message: &mut Message) {
        let string = match control {
            Control::Direction(direction) => self.translate_direction(direction),
            Control::Close => "close door",
            Control::Fire => "fire",
            Control::NextTarget => "next target",
            Control::PrevTarget => "previous target",
            Control::Wait => "wait a turn",
            Control::DisplayMessageLog => "full screen message log",
            Control::Examine => "examine",
            Control::Pause => "pause",
            Control::Use => "use",
        };

        message.push(MessagePart::plain(string));
    }

    fn translate_intro(&self, message: &mut Message) {
        message.push(MessagePart::plain("Everything beneath the moonlight appears different. "));
        message.push(MessagePart::plain("An arcane tome is rumored to be hidden somewhere in the forest. "));
        message.push(MessagePart::plain("Perhaps the answers lie within."));
    }
}

impl Language for English {
    fn translate_repeated(&self, message_type: MessageType, repeated: usize, message: &mut Message) {

        match message_type {
            MessageType::Empty => {},
            MessageType::Intro => self.translate_intro(message),
            MessageType::Title => message.push(MessagePart::colour(colours::PURPLE, "HOWL")),
            MessageType::PressAnyKey => message.push(MessagePart::plain("Press any key...")),
            MessageType::Welcome => {
                message.push(MessagePart::plain("Welcome to "));
                message.push(MessagePart::colour(colours::PURPLE, "HOWL"));
                message.push(MessagePart::plain("!"));
            }
            MessageType::YouDied => {
                message.push(MessagePart::colour(colours::RED, "YOU DIED"));
            }
            MessageType::Action(action) => {
                self.translate_action(action, message);
            }
            MessageType::YouSee(name) => {
                message.push(MessagePart::plain("I see: "));
                if let Some(name) = name {
                    self.translate_you_see(name, message);
                }
            }
            MessageType::YouRemember(name) => {
                message.push(MessagePart::plain("I remember: "));
                if let Some(name) = name {
                    self.translate_you_see(name, message);
                }
            }
            MessageType::Unseen => {
                message.push(MessagePart::plain("I haven't seen this location."));
            }
            MessageType::Description(description) => {
                self.translate_description(description, message);
            }
            MessageType::YouSeeDescription(you_see) => {
                self.translate_you_see(you_see, message);
            }
            MessageType::NoDescription => {
                message.push(MessagePart::plain("I see nothing of interest."));
            }
            MessageType::Menu(menu_message) => {
                self.translate_menu(menu_message, message);
            }
        }

        if repeated > 1 {
            message.push(MessagePart::Text(TextMessagePart::Plain(format!("(x{})", repeated))));
        }
    }
}
