use colour::Rgb24;

pub type Message = Vec<MessagePart>;

pub enum MessagePart {
    Plain(&'static str),
    Colour(Rgb24, &'static str),
}
