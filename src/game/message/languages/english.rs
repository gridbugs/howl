use game::*;
use colour::*;

const DARK_YELLOW: Rgb24 = Rgb24 { red: 0x80, green: 0x80, blue: 0 };

pub struct English;

impl English {
    fn translate_name(&self, name: NameMessageType, message: &mut Message) {
        match name {
            NameMessageType::Pistol => {
                message.push(MessagePart::plain("Pistol"));
            }
            NameMessageType::Shotgun => {
                message.push(MessagePart::plain("Shotgun"));
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
            DescriptionMessageType::Pistol => {
                message.push(MessagePart::plain("Simple, reliable, accurate."));
            }
            DescriptionMessageType::Shotgun => {
                message.push(MessagePart::plain("Reasonable chance to hit the target...as well as anything that happens to be nearby."));
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
                message.push(MessagePart::Text(TextMessagePart::Plain(String::from(control))));
                message.push(MessagePart::plain(": "));
                message.push(MessagePart::Text(TextMessagePart::Plain(String::from(input))));
            }
            MenuMessageType::UnboundControl(control) => {
                message.push(MessagePart::Text(TextMessagePart::Plain(String::from(control))));
                message.push(MessagePart::plain(": (unbound)"));
            }
            MenuMessageType::ControlBinding(control) => {
                message.push(MessagePart::Text(TextMessagePart::Plain(String::from(control))));
                message.push(MessagePart::plain(": press a key..."));
            }
            MenuMessageType::NextDelivery => {
                message.push(MessagePart::plain("Next Delivery"));
            }
            MenuMessageType::Shop => {
                message.push(MessagePart::plain("Shop"));
            }
            MenuMessageType::Garage => {
                message.push(MessagePart::plain("Garage"));
            }
            MenuMessageType::Inventory => {
                message.push(MessagePart::plain("Inventory"));
            }
            MenuMessageType::Name(name) => {
                self.translate_name(name, message);
            }
            MenuMessageType::ShopItem(name, price) => {
                self.translate_name(name, message);
                message.push(MessagePart::plain(": "));
                message.push(MessagePart::Text(TextMessagePart::Plain(format!("{}", price))));
            }
            MenuMessageType::Back => {
                message.push(MessagePart::plain("Back"));
            }
            MenuMessageType::Remove => {
                message.push(MessagePart::plain("Remove"));
            }
        }
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
            MessageType::Title => message.push(MessagePart::colour(DARK_YELLOW, "Apocalypse Post")),
            MessageType::PressAnyKey => message.push(MessagePart::plain("Press any key...")),
            MessageType::Welcome => {
                message.push(MessagePart::plain("Welcome to "));
                message.push(MessagePart::colour(DARK_YELLOW, "Apocalypse Post"));
                message.push(MessagePart::plain("!"));
            }
            MessageType::YouDied => {
                message.push(MessagePart::colour(colours::RED, "YOU DIED"));
            }
            MessageType::Action(action) => {
                self.translate_action(action, message);
            }
            MessageType::Name(name) => {
                self.translate_name(name, message);
            }
            MessageType::YouRemember(name) => {
                message.push(MessagePart::plain("I remember: "));
                if let Some(name) = name {
                    self.translate_name(name, message);
                }
            }
            MessageType::Unseen => {
                message.push(MessagePart::plain("I haven't seen this location."));
            }
            MessageType::Description(description) => {
                self.translate_description(description, message);
            }
            MessageType::NameDescription(name) => {
                self.translate_name(name, message);
            }
            MessageType::NoDescription => {
                message.push(MessagePart::plain("I see nothing of interest."));
            }
            MessageType::Menu(menu_message) => {
                self.translate_menu(menu_message, message);
            }
            MessageType::ChooseDirection => {
                message.push(MessagePart::plain("Which direction?"));
            }
            MessageType::EmptyWeaponSlotMessage => {
                message.push(MessagePart::plain("No gun in slot!"));
            }
            MessageType::Front => {
                message.push(MessagePart::plain("Front"));
            }
            MessageType::Rear => {
                message.push(MessagePart::plain("Rear"));
            }
            MessageType::Left => {
                message.push(MessagePart::plain("Left"));
            }
            MessageType::Right => {
                message.push(MessagePart::plain("Right"));
            }
            MessageType::EmptyWeaponSlot => {
                message.push(MessagePart::plain("(empty)"));
            }
            MessageType::SurvivorCamp => {
                message.push(MessagePart::plain("Survivor Camp"));
            }
            MessageType::ShopTitle(balance) => {
                message.push(MessagePart::Text(TextMessagePart::Plain(format!("Shop - Your balance: {}", balance))));
            }
            MessageType::ShopTitleInsufficientFunds(balance) => {
                message.push(MessagePart::Text(TextMessagePart::Plain(format!("Shop - Your balance: {}", balance))));
                message.push(MessagePart::Newline);
                message.push(MessagePart::plain("You can't afford that!"));
            }
            MessageType::ShopTitleInventoryFull(balance) => {
                message.push(MessagePart::Text(TextMessagePart::Plain(format!("Shop - Your balance: {}", balance))));
                message.push(MessagePart::Newline);
                message.push(MessagePart::plain("No space in inventory!"));
            }
            MessageType::Inventory { size, capacity } => {
                message.push(MessagePart::Text(TextMessagePart::Plain(format!("Inventory: {}/{}", size, capacity))));
            }
            MessageType::NameAndDescription(name, description) => {
                self.translate_name(name, message);
                message.push(MessagePart::Newline);
                message.push(MessagePart::Newline);
                self.translate_description(description, message);
            }
        }

        if repeated > 1 {
            message.push(MessagePart::Text(TextMessagePart::Plain(format!("(x{})", repeated))));
        }
    }
}
