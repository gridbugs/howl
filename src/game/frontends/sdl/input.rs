use std::time;
use std::cmp;
use std::thread;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::{self, Keycode, Mod};

use control::*;

const MIN_TIMEOUT_MS: u32 = 4;
const FRAME_TIME_MS: u64 = 16;

#[derive(Clone)]
pub struct SdlInputSource {
    sdl: Sdl,
    last: time::Instant,
    count: usize,
}

#[derive(Clone)]
pub struct SdlBlockingInputSource(SdlInputSource);

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

fn event_to_input(event: Event) -> Option<InputEvent> {
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
        _ => None,
    }
}

impl SdlInputSource {
    pub fn new(sdl: Sdl) -> Self {
        SdlInputSource {
            sdl: sdl,
            last: time::Instant::now(),
            count: 0,
        }
    }

    fn increase_count(&mut self) -> usize {
        let count = self.count;
        self.count += 1;
        count
    }

    fn frame_now(&mut self) -> Frame {
        Frame::new(self.increase_count(), time::Instant::now())
    }

    fn frame_duration(&self) -> time::Duration { time::Duration::from_millis(FRAME_TIME_MS) }
}

impl InputSource for SdlInputSource {
    fn next_input(&mut self) -> InputEvent {
        let mut event_pump = self.sdl.event_pump().expect("Failed to initialise event pump");
        loop {
            if let Some(input_event) = event_to_input(event_pump.wait_event()) {
                return input_event;
            }
        }
    }

    fn next_external(&mut self) -> ExternalEvent {
        let mut event_pump = self.sdl.event_pump().expect("Failed to initialise event pump");

        let now = time::Instant::now();
        let elapsed = now.duration_since(self.last);
        let timeout = if let Some(remaining) = self.frame_duration().checked_sub(elapsed) {
            let ms = remaining.subsec_nanos() / 1000000;
            cmp::max(ms, MIN_TIMEOUT_MS)
        } else {
            MIN_TIMEOUT_MS
        };

        loop {
            if let Some(event) = event_pump.wait_event_timeout(timeout) {
                if let Some(input_event) = event_to_input(event) {
                    return ExternalEvent::Input(input_event);
                }
            } else {
                return ExternalEvent::Frame(self.frame_now());
            }
        }
    }

    fn next_frame_periodic(&mut self, period: &mut Period) -> Option<Frame> {
        if period.decrease() {
            let now = time::Instant::now();
            let elapsed = now.duration_since(self.last);
            if let Some(remaining) = self.frame_duration().checked_sub(elapsed) {
                thread::sleep(remaining);
                Some(self.frame_now())
            } else {
                Some(Frame::new(self.increase_count(), now))
            }
        } else {
            None
        }
    }
}

impl SdlBlockingInputSource {
    pub fn new(sdl: Sdl) -> Self {
        SdlBlockingInputSource(SdlInputSource::new(sdl))
    }
}

impl InputSource for SdlBlockingInputSource {
    fn next_input(&mut self) -> InputEvent { self.0.next_input() }

    fn next_external(&mut self) -> ExternalEvent {
        ExternalEvent::Input(self.next_input())
    }

    fn next_frame_periodic(&mut self, period: &mut Period) -> Option<Frame> {

        if period.is_empty() {
            return None;
        }

        let maybe_frame = if let Some(duration) = self.0.frame_duration().checked_mul(period.remaining() as u32) {
            thread::sleep(duration);
            Some(self.0.frame_now())
        } else {
            None
        };

        period.clear();

        maybe_frame
    }
}
