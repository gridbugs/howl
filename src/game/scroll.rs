use std::cmp;

use game::*;

pub fn display_message_scrolling<R: KnowledgeRenderer, I: InputSource>(renderer: &mut R,
                                                                       input_source: &mut I,
                                                                       message: &Message,
                                                                       press_any_key: bool) {
    let mut offset = 0;

    let mut wrapped = Vec::new();
    renderer.wrap_message_to_fit(message, &mut wrapped);

    let max_offset = if wrapped.len() > renderer.display_log_num_lines() {
        wrapped.len() - renderer.display_log_num_lines()
    } else {
        0
    };

    loop {
        renderer.display_translated_message_fullscreen(message, offset);

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

    renderer.draw();
}
