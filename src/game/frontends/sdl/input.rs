use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::{self, Keycode, Mod};

use game::*;

#[derive(Clone)]
pub struct SdlInputSource {
    sdl: Sdl,
}

impl SdlInputSource {
    pub fn new(sdl: Sdl) -> Self {
        SdlInputSource {
            sdl: sdl,
        }
    }
}

fn is_shift_pressed(keymod: &Mod) -> bool {
    keymod.contains(keyboard::LSHIFTMOD) || keymod.contains(keyboard::RSHIFTMOD)
}

fn to_char_event(ch: char, keymod: &Mod) -> InputEvent {
    if is_shift_pressed(keymod) {
        let chars = ch.to_uppercase().collect::<Vec<char>>();
        InputEvent::Char(chars[0])
    } else {
        // ch must be lowercase
        InputEvent::Char(ch)
    }
}

impl InputSource for SdlInputSource {
    fn next_input(&mut self) -> Option<InputEvent> {
        let mut event_pump = self.sdl.event_pump().expect("Failed to initialise event pump");

        loop {
            let event = event_pump.wait_event();

            match event {
                Event::Quit { .. } => return Some(InputEvent::Quit),
                Event::KeyDown { keycode: Some(keycode), ref keymod, .. } => {
                    return match keycode {
                        Keycode::Up => Some(InputEvent::Up),
                        Keycode::Down => Some(InputEvent::Down),
                        Keycode::Left => Some(InputEvent::Left),
                        Keycode::Right => Some(InputEvent::Right),
                        Keycode::A => Some(to_char_event('a', keymod)),
                        Keycode::B => Some(to_char_event('b', keymod)),
                        Keycode::C => Some(to_char_event('c', keymod)),
                        Keycode::D => Some(to_char_event('d', keymod)),
                        Keycode::E => Some(to_char_event('e', keymod)),
                        Keycode::F => Some(to_char_event('f', keymod)),
                        Keycode::G => Some(to_char_event('g', keymod)),
                        Keycode::H => Some(to_char_event('h', keymod)),
                        Keycode::I => Some(to_char_event('i', keymod)),
                        Keycode::J => Some(to_char_event('j', keymod)),
                        Keycode::K => Some(to_char_event('k', keymod)),
                        Keycode::L => Some(to_char_event('l', keymod)),
                        Keycode::M => Some(to_char_event('m', keymod)),
                        Keycode::N => Some(to_char_event('n', keymod)),
                        Keycode::O => Some(to_char_event('o', keymod)),
                        Keycode::P => Some(to_char_event('p', keymod)),
                        Keycode::Q => Some(to_char_event('q', keymod)),
                        Keycode::R => Some(to_char_event('r', keymod)),
                        Keycode::S => Some(to_char_event('s', keymod)),
                        Keycode::T => Some(to_char_event('t', keymod)),
                        Keycode::U => Some(to_char_event('u', keymod)),
                        Keycode::V => Some(to_char_event('v', keymod)),
                        Keycode::W => Some(to_char_event('w', keymod)),
                        Keycode::X => Some(to_char_event('x', keymod)),
                        Keycode::Y => Some(to_char_event('y', keymod)),
                        Keycode::Z => Some(to_char_event('z', keymod)),
                        Keycode::Space => Some(InputEvent::Space),
                        Keycode::Escape => Some(InputEvent::Escape),
                        Keycode::Return => Some(InputEvent::Return),
                        Keycode::Period => {
                            if is_shift_pressed(keymod) {
                                Some(InputEvent::Char('>'))
                            } else {
                                Some(InputEvent::Char('.'))
                            }
                        }
                        Keycode::Comma => {
                            if is_shift_pressed(keymod) {
                                Some(InputEvent::Char('<'))
                            } else {
                                Some(InputEvent::Char(','))
                            }
                        }
                        Keycode::Slash => {
                            if is_shift_pressed(keymod) {
                                Some(InputEvent::Char('?'))
                            } else {
                                Some(InputEvent::Char('/'))
                            }
                        }
                        _ => None,
                    };
                },
                _ => (),
            }
        }
    }
}
