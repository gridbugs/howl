use std::collections::{hash_map, HashMap};

use game::InputEvent;
use direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Control {
    Direction(Direction),
    Use,
    Fire,
    NextTarget,
    PrevTarget,
    Close,
    Wait,
    DisplayMessageLog,
    Examine,
    Pause,
}

const NUM_CONTROLS: usize = 13;
const CONTROL_ORDER: [Control; NUM_CONTROLS] = [
    Control::Direction(Direction::North),
    Control::Direction(Direction::South),
    Control::Direction(Direction::East),
    Control::Direction(Direction::West),
    Control::Use,
    Control::Wait,
    Control::Close,
    Control::Fire,
    Control::NextTarget,
    Control::PrevTarget,
    Control::Examine,
    Control::DisplayMessageLog,
    Control::Pause,
];

pub type ControlMapIter<'a> = hash_map::Iter<'a, InputEvent, Control>;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn remove(&mut self, input: InputEvent) {
        self.map.remove(&input);
    }

    pub fn new() -> Self {
        ControlMap {
            map: HashMap::new(),
        }
    }

    pub fn add_defaults(&mut self) {
        self.insert(InputEvent::Up, Control::Direction(Direction::North));
        self.insert(InputEvent::Down, Control::Direction(Direction::South));
        self.insert(InputEvent::Left, Control::Direction(Direction::West));
        self.insert(InputEvent::Right, Control::Direction(Direction::East));

        self.insert(InputEvent::Return, Control::Use);
        self.insert(InputEvent::Escape, Control::Pause);

        self.insert(InputEvent::Char('c'), Control::Close);
        self.insert(InputEvent::Char('x'), Control::Examine);
        self.insert(InputEvent::Char('.'), Control::Wait);

        self.insert(InputEvent::Char('f'), Control::Fire);
        self.insert(InputEvent::Char('n'), Control::NextTarget);
        self.insert(InputEvent::Char('N'), Control::PrevTarget);

        self.insert(InputEvent::Char('t'), Control::DisplayMessageLog);
    }

    pub fn invert(&self) -> HashMap<Control, Vec<InputEvent>> {
        let mut inverted = HashMap::new();

        for (input_event, control) in self.iter() {
            inverted.entry(*control).or_insert_with(Vec::new).push(*input_event);
        }

        inverted
    }

    pub fn descriptions(&self) -> Vec<ControlDescription> {
        let mut descriptions = Vec::new();

        let inverted = self.invert();

        for control in CONTROL_ORDER.iter() {
            if let Some(inputs) = inverted.get(control) {
                descriptions.push(ControlDescription {
                    control: *control,
                    inputs: inputs.clone(),
                });
            } else {
                descriptions.push(ControlDescription {
                    control: *control,
                    inputs: Vec::new(),
                });
            }
        }

        descriptions
    }
}

impl Default for ControlMap {
    fn default() -> Self {
        let mut map = ControlMap::new();
        map.add_defaults();
        map
    }
}

pub struct ControlDescription {
    pub control: Control,
    pub inputs: Vec<InputEvent>,
}
