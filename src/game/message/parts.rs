use colour::Rgb24;

pub type Message = Vec<MessagePart>;

#[derive(Clone, Copy, Debug)]
pub enum MessagePart {
    Plain(&'static str),
    Colour(Rgb24, &'static str),
}
