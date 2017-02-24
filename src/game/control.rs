use std::collections::{hash_map, HashMap};

use game::InputEvent;
use direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    Ascend,
    Descend,
}

pub type ControlMapIter<'a> = hash_map::Iter<'a, InputEvent, Control>;

#[derive(Clone, Serialize, Deserialize)]
pub struct ControlMap {
    map: HashMap<InputEvent, Control>,
}

impl ControlMap {
    pub fn control(&self, event: InputEvent) -> Option<Control> {
        self.map.get(&event).map(|r| *r)
    }

    pub fn iter(&self) -> ControlMapIter {
        self.map.iter()
    }

    pub fn insert(&mut self, input: InputEvent, control: Control) {
        self.map.insert(input, control);
    }

    pub fn bare() -> Self {
        let mut map = HashMap::new();

        map.insert(InputEvent::Quit, Control::Quit);
        map.insert(InputEvent::Return, Control::Select);
        map.insert(InputEvent::Escape, Control::Quit);

        ControlMap {
            map: map,
        }
    }

    pub fn add_defaults(&mut self) {
        self.insert(InputEvent::Up, Control::Direction(Direction::North));
        self.insert(InputEvent::Down, Control::Direction(Direction::South));
        self.insert(InputEvent::Left, Control::Direction(Direction::West));
        self.insert(InputEvent::Right, Control::Direction(Direction::East));

        self.insert(InputEvent::Char('<'), Control::Ascend);
        self.insert(InputEvent::Char('>'), Control::Descend);

        self.insert(InputEvent::Char('c'), Control::Close);
        self.insert(InputEvent::Char('x'), Control::Examine);
        self.insert(InputEvent::Char('.'), Control::Wait);

        self.insert(InputEvent::Char('f'), Control::Fire);
        self.insert(InputEvent::Char('n'), Control::NextTarget);
        self.insert(InputEvent::Char('m'), Control::PrevTarget);

        self.insert(InputEvent::Char('t'), Control::DisplayMessageLog);
        self.insert(InputEvent::Char('?'), Control::Help);
    }
}

impl Default for ControlMap {
    fn default() -> Self {
        let mut map = ControlMap::bare();
        map.add_defaults();
        map
    }
}
