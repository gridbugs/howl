use colour::Rgb24;

pub type Message = Vec<MessagePart>;

#[derive(Clone, Debug)]
pub enum MessagePart {
    Plain(String),
    Colour(Rgb24, String),
}
