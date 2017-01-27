use std::collections::{hash_map, HashMap};

use game::InputEvent;
use direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Control {
    Direction(Direction),
    Close,
    Fire,
    NextTarget,
    PrevTarget,
    Wait,
    DisplayMessageLog,
    Examine,
    Select,
    Quit,
    Help,
}

pub type ControlMapIter<'a> = hash_map::Iter<'a, InputEvent, Control>;

pub struct ControlMap {
    map: HashMap<InputEvent, Control>,
}

impl ControlMap {
    pub fn new_default() -> Self {
        let mut map = HashMap::new();

        map.insert(InputEvent::Up, Control::Direction(Direction::North));
        map.insert(InputEvent::Down, Control::Direction(Direction::South));
        map.insert(InputEvent::Left, Control::Direction(Direction::West));
        map.insert(InputEvent::Right, Control::Direction(Direction::East));

        map.insert(InputEvent::Char('c'), Control::Close);
        map.insert(InputEvent::Char('f'), Control::Fire);
        map.insert(InputEvent::Char('n'), Control::NextTarget);
        map.insert(InputEvent::Char('m'), Control::PrevTarget);
        map.insert(InputEvent::Char('t'), Control::DisplayMessageLog);
        map.insert(InputEvent::Char('x'), Control::Examine);

        map.insert(InputEvent::Char('.'), Control::Wait);

        map.insert(InputEvent::Char('q'), Control::Quit);
        map.insert(InputEvent::Char('?'), Control::Help);
        map.insert(InputEvent::Quit, Control::Quit);
        map.insert(InputEvent::Return, Control::Select);

        ControlMap {
            map: map,
        }
    }

    pub fn control(&self, event: InputEvent) -> Option<Control> {
        self.map.get(&event).map(|r| *r)
    }

    pub fn iter(&self) -> ControlMapIter {
        self.map.iter()
    }
}
