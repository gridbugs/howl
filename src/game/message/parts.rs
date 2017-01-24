use std::mem;

use colour::Rgb24;

pub type Message = Vec<MessagePart>;

#[derive(Clone, Debug)]
pub enum MessagePart {
    Plain(String),
    Colour(Rgb24, String),
}

impl MessagePart {
    pub fn len(&self) -> usize {
        match *self {
            MessagePart::Plain(ref s) => s.len(),
            MessagePart::Colour(_, ref s) => s.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            MessagePart::Plain(ref s) => s.is_empty(),
            MessagePart::Colour(_, ref s) => s.is_empty(),
        }
    }


    fn split_at_exclude_middle(&self, mid: usize) -> (MessagePart, MessagePart) {
        match *self {
            MessagePart::Plain(ref s) => {
                let (l, r) = s.split_at(mid);
                (MessagePart::Plain(l.to_string()), MessagePart::Plain(r[1..].to_string()))
            }
            MessagePart::Colour(c, ref s) => {
                let (l, r) = s.split_at(mid);
                (MessagePart::Colour(c, l.to_string()), MessagePart::Colour(c, r[1..].to_string()))
            }
        }
    }

    fn split_at(&self, mid: usize) -> (MessagePart, MessagePart) {
        match *self {
            MessagePart::Plain(ref s) => {
                let (l, r) = s.split_at(mid);
                (MessagePart::Plain(l.to_string()), MessagePart::Plain(r.to_string()))
            }
            MessagePart::Colour(c, ref s) => {
                let (l, r) = s.split_at(mid);
                (MessagePart::Colour(c, l.to_string()), MessagePart::Colour(c, r.to_string()))
            }
        }
    }

    pub fn string_ref(&self) -> &str {
        match *self {
            MessagePart::Plain(ref s) => s.as_ref(),
            MessagePart::Colour(_, ref s) => s.as_ref(),
        }
    }
}

pub fn wrap_message(message: &Message, width: usize, wrapped: &mut Vec<Message>) {

    let mut x = 0;
    let mut current_message = Message::new();

    for part in message.iter() {

        let mut next_x = x + part.len();
        current_message.push(part.clone());

        if next_x < width {
            x = next_x;
        } else if next_x == width {
            wrapped.push(mem::replace(&mut current_message, Message::new()));
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
                let mut next = Message::new();
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
