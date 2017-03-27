use std::cmp;

use control::*;
use game::*;
use message::*;

pub fn display_message_scrolling<R: KnowledgeRenderer, I: InputSource>(renderer: &mut R,
                                                                       input_source: &mut I,
                                                                       message: &Message,
                                                                       press_any_key: bool) {
    let mut offset = 0;

    let mut wrapped = Vec::new();
    renderer.fullscreen_wrap(message, &mut wrapped);

    let max_offset = if wrapped.len() > renderer.fullscreen_log_num_rows() {
        wrapped.len() - renderer.fullscreen_log_num_rows()
    } else {
        0
    };

    loop {
        renderer.publish_fullscreen_translated_message(message, offset);

        if let Some(event) = input_source.next_input() {
            match event {
                InputEvent::Down => {
                    if max_offset == 0 {
                        break;
                    }
                    offset = cmp::min(offset + 1, max_offset);
                }
                InputEvent::Up => {
                    if max_offset == 0 {
                        break;
                    }
                    if offset > 0 {
                        offset -= 1;
                    }
                }
                InputEvent::Escape => break,
                _ => {
                    if press_any_key {
                        break;
                    }
                }
            }
        } else {
            break;
        }
    }
}
