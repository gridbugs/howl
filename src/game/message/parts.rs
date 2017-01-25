use std::mem;

use colour::Rgb24;

pub type Message = Vec<MessagePart>;
pub type TextMessage = Vec<TextMessagePart>;

#[derive(Clone, Debug)]
pub enum MessagePart {
    Text(TextMessagePart),
    Newline,
}

#[derive(Clone, Debug)]
pub enum TextMessagePart {
    Plain(String),
    Colour(Rgb24, String),
}

impl MessagePart {
    pub fn as_text(&self) -> Option<&TextMessagePart> {
        match *self {
            MessagePart::Text(ref m) => Some(m),
            _ => None,
        }
    }

    pub fn plain(string: &str) -> Self {
        MessagePart::Text(TextMessagePart::plain(string))
    }

    pub fn colour(colour: Rgb24, string: &str) -> Self {
        MessagePart::Text(TextMessagePart::colour(colour, string))
    }
}

impl TextMessagePart {

    pub fn plain(string: &str) -> Self {
        TextMessagePart::Plain(string.to_string())
    }

    pub fn colour(colour: Rgb24, string: &str) -> Self {
        TextMessagePart::Colour(colour, string.to_string())
    }

    pub fn len(&self) -> usize {
        match *self {
            TextMessagePart::Plain(ref s) => s.len(),
            TextMessagePart::Colour(_, ref s) => s.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            TextMessagePart::Plain(ref s) => s.is_empty(),
            TextMessagePart::Colour(_, ref s) => s.is_empty(),
        }
    }

    fn split_at_exclude_middle(&self, mid: usize) -> (TextMessagePart, TextMessagePart) {
        match *self {
            TextMessagePart::Plain(ref s) => {
                let (l, r) = s.split_at(mid);
                (TextMessagePart::Plain(l.to_string()), TextMessagePart::Plain(r[1..].to_string()))
            }
            TextMessagePart::Colour(c, ref s) => {
                let (l, r) = s.split_at(mid);
                (TextMessagePart::Colour(c, l.to_string()), TextMessagePart::Colour(c, r[1..].to_string()))
            }
        }
    }

    fn split_at(&self, mid: usize) -> (TextMessagePart, TextMessagePart) {
        match *self {
            TextMessagePart::Plain(ref s) => {
                let (l, r) = s.split_at(mid);
                (TextMessagePart::Plain(l.to_string()), TextMessagePart::Plain(r.to_string()))
            }
            TextMessagePart::Colour(c, ref s) => {
                let (l, r) = s.split_at(mid);
                (TextMessagePart::Colour(c, l.to_string()), TextMessagePart::Colour(c, r.to_string()))
            }
        }
    }

    pub fn string_ref(&self) -> &str {
        match *self {
            TextMessagePart::Plain(ref s) => s.as_ref(),
            TextMessagePart::Colour(_, ref s) => s.as_ref(),
        }
    }
}

pub fn wrap_message(message: &Message, width: usize, wrapped: &mut Vec<TextMessage>) {

    let mut x = 0;
    let mut current_message = TextMessage::new();

    for part in message.iter() {

        let text_part = match *part {
            MessagePart::Text(ref text_part) => text_part,
            MessagePart::Newline => {

                wrapped.push(mem::replace(&mut current_message, TextMessage::new()));
                x = 0;

                continue;
            }
        };

        let mut next_x = x + text_part.len();
        current_message.push(text_part.clone());

        if next_x < width {
            x = next_x;
        } else if next_x == width {
            wrapped.push(mem::replace(&mut current_message, TextMessage::new()));
            x = 0;
        } else {
            while next_x > width {
                let last = current_message.pop().unwrap();
                let mut split_idx = 0;
                let mut idx = 0;
                let mut first = true;
                for ch in last.string_ref().chars() {
                    if idx + x > width {
                        break;
                    }

                    if ch.is_whitespace() {
                        first = false;
                        split_idx = idx;
                    }

                    idx += 1;
                }

                let next_start = if first {
                    if x == 0 {
                        // split the part at its end - there's no whitespace
                        let (this_end, next_start) = last.split_at(width);
                        if !this_end.is_empty() {
                            current_message.push(this_end);
                        }
                        next_start
                    } else {
                        // move the entire part to the next line
                        last
                    }
                } else {
                    // split the part at white space
                    let (this_end, next_start) = last.split_at_exclude_middle(split_idx);
                    if !this_end.is_empty() {
                        current_message.push(this_end);
                    }
                    next_start
                };

                next_x = next_start.len();
                let mut next = TextMessage::new();
                next.push(next_start);
                wrapped.push(mem::replace(&mut current_message, next));
                x = 0;
            }

            x = next_x;
        }
    }

    if x < width {
        wrapped.push(current_message);
    }
}
