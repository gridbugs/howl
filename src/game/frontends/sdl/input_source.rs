use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use game::*;

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

impl InputSource for SdlInputSource {
    fn next_input(&self) -> Option<InputEvent> {
        let mut event_pump = self.sdl.event_pump().expect("Failed to initialise event pump");

        loop {
            let event = event_pump.wait_event();

            match event {
                Event::Quit { .. } => return Some(InputEvent::Quit),
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    return match keycode {
                        Keycode::Up => Some(InputEvent::Up),
                        Keycode::Down => Some(InputEvent::Down),
                        Keycode::Left => Some(InputEvent::Left),
                        Keycode::Right => Some(InputEvent::Right),
                        Keycode::A => Some(InputEvent::Char('a')),
                        Keycode::B => Some(InputEvent::Char('b')),
                        Keycode::C => Some(InputEvent::Char('c')),
                        Keycode::D => Some(InputEvent::Char('d')),
                        Keycode::E => Some(InputEvent::Char('e')),
                        Keycode::F => Some(InputEvent::Char('f')),
                        Keycode::G => Some(InputEvent::Char('g')),
                        Keycode::H => Some(InputEvent::Char('h')),
                        Keycode::I => Some(InputEvent::Char('i')),
                        Keycode::J => Some(InputEvent::Char('j')),
                        Keycode::K => Some(InputEvent::Char('k')),
                        Keycode::L => Some(InputEvent::Char('l')),
                        Keycode::M => Some(InputEvent::Char('m')),
                        Keycode::N => Some(InputEvent::Char('n')),
                        Keycode::O => Some(InputEvent::Char('o')),
                        Keycode::P => Some(InputEvent::Char('p')),
                        Keycode::Q => Some(InputEvent::Char('q')),
                        Keycode::R => Some(InputEvent::Char('r')),
                        Keycode::S => Some(InputEvent::Char('s')),
                        Keycode::T => Some(InputEvent::Char('t')),
                        Keycode::U => Some(InputEvent::Char('u')),
                        Keycode::V => Some(InputEvent::Char('v')),
                        Keycode::W => Some(InputEvent::Char('w')),
                        Keycode::X => Some(InputEvent::Char('x')),
                        Keycode::Y => Some(InputEvent::Char('y')),
                        Keycode::Z => Some(InputEvent::Char('z')),
                        _ => None,
                    };
                },
                _ => (),
            }
        }
    }
}
