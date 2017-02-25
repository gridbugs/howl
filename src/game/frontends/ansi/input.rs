use game::*;
use frontends::ansi::AnsiInputSource;

use rustty;

const ETX: char = '\u{3}';
const ESC: char = '\u{1b}';
const RETURN: char = '\r';
const SPACE: char = ' ';

impl InputSource for AnsiInputSource {
    fn next_input(&mut self) -> Option<InputEvent> {
        self.get_event().and_then(|event| {

            match event {
                rustty::Event::Char(ETX) => Some(InputEvent::Quit),
                rustty::Event::Char(ESC) => Some(InputEvent::Escape),
                rustty::Event::Char(RETURN) => Some(InputEvent::Return),
                rustty::Event::Char(SPACE) => Some(InputEvent::Space),
                rustty::Event::Char(ch) => Some(InputEvent::Char(ch)),
                rustty::Event::Up => Some(InputEvent::Up),
                rustty::Event::Down => Some(InputEvent::Down),
                rustty::Event::Left => Some(InputEvent::Left),
                rustty::Event::Right => Some(InputEvent::Right),
                _ => None,
            }
        })
    }
}
