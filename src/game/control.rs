use std::collections::{hash_map, HashMap};
use std::slice;

use game::InputEvent;
use game::control_spec::ControlSpec;
use direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Control {
    Direction(Direction),
    Fire,
    Wait,
    DisplayMessageLog,
    Pause,
    Status,
}

const NUM_CONTROLS: usize = 9;
const CONTROL_ORDER: [Control; NUM_CONTROLS] = [
    Control::Direction(Direction::North),
    Control::Direction(Direction::South),
    Control::Direction(Direction::East),
    Control::Direction(Direction::West),
    Control::Wait,
    Control::Fire,
    Control::Status,
    Control::DisplayMessageLog,
    Control::Pause,
];

pub struct ControlMapIter<'a>(hash_map::Iter<'a, InputEvent, Control>);
impl<'a> Iterator for ControlMapIter<'a> {
    type Item = (InputEvent, Control);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(i, c)| (*i, *c))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlMap {
    map: HashMap<InputEvent, Control>,
}

impl ControlMap {
    pub fn get(&self, event: InputEvent) -> Option<Control> {
        self.map.get(&event).map(|r| *r)
    }

    pub fn iter(&self) -> ControlMapIter {
        ControlMapIter(self.map.iter())
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

        self.insert(InputEvent::Escape, Control::Pause);

        self.insert(InputEvent::Char('.'), Control::Wait);

        self.insert(InputEvent::Char('f'), Control::Fire);

        self.insert(InputEvent::Char('t'), Control::DisplayMessageLog);
    }

    pub fn descriptions(&self) -> ControlDescriptions {
        let mut descriptions = ControlDescriptions::new();

        let spec = ControlSpec::from(self);

        for control in CONTROL_ORDER.iter() {
            descriptions.descriptions.push((*control, spec.get(*control)));
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

pub struct ControlDescriptions {
    descriptions: Vec<(Control, Option<InputEvent>)>,
}

impl ControlDescriptions {
    fn new() -> Self {
        ControlDescriptions {
            descriptions: Vec::new(),
        }
    }

    pub fn iter(&self) -> ControlDescriptionsIter {
        ControlDescriptionsIter(self.descriptions.iter())
    }
}

pub struct ControlDescriptionsIter<'a>(slice::Iter<'a, (Control, Option<InputEvent>)>);
impl<'a> Iterator for ControlDescriptionsIter<'a> {
    type Item = (Control, Option<InputEvent>);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|d| *d)
    }
}
