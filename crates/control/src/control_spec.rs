use std::collections::{hash_map, HashMap};
use control::{Control, ControlMap};
use input::InputEvent;
use math::*;

pub struct ControlSpec {
    controls: HashMap<Control, InputEvent>,
}

impl ControlSpec {
    pub fn new() -> Self {
        ControlSpec {
            controls: HashMap::new(),
        }
    }

    pub fn iter(&self) -> ControlSpecIter {
        ControlSpecIter(self.controls.iter())
    }

    pub fn get(&self, control: Control) -> Option<InputEvent> {
        self.controls.get(&control).map(|i| *i)
    }
}

impl<'a> From<&'a ControlMap> for ControlSpec {
    fn from(map: &'a ControlMap) -> Self {
        let mut spec = ControlSpec::new();

        for (input, control) in map.iter() {
            spec.controls.insert(control, input);
        }

        spec
    }
}

impl<'a> From<&'a ControlSpec> for ControlMap {
    fn from(spec: &'a ControlSpec) -> Self {
        let mut map = ControlMap::new();

        for (control, input) in spec.iter() {
            map.insert(input, control);
        }

        map
    }
}

pub struct ControlSpecIter<'a>(hash_map::Iter<'a, Control, InputEvent>);
impl<'a> Iterator for ControlSpecIter<'a> {
    type Item = (Control, InputEvent);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(c, i)| (*c, *i))
    }
}

pub type StringControlSpec = HashMap<String, String>;

impl<'a> From<&'a ControlSpec> for StringControlSpec {
    fn from(spec: &'a ControlSpec) -> Self {
        let mut string_spec = StringControlSpec::new();

        for (control, input_event) in spec.iter() {
            string_spec.insert(String::from(control), String::from(input_event));
        }

        string_spec
    }
}

impl<'a> From<&'a StringControlSpec> for ControlSpec {
    fn from(string_spec: &'a StringControlSpec) -> Self {
        let mut spec = ControlSpec::new();

        for (control, input_event) in string_spec.iter() {
            spec.controls.insert(Control::from(control), InputEvent::from(input_event));
        }

        spec
    }
}


impl<'a> From<&'a ControlMap> for StringControlSpec {
    fn from(map: &'a ControlMap) -> Self {
        StringControlSpec::from(&ControlSpec::from(map))
    }
}

impl<'a> From<&'a StringControlSpec> for ControlMap {
    fn from(spec: &'a StringControlSpec) -> Self {
        ControlMap::from(&ControlSpec::from(spec))
    }
}

impl<'a> From<&'a String> for Control {
    fn from(s: &'a String) -> Self {
        match s.as_ref() {
            "SteerUp" => Control::Direction(Direction::North),
            "SteerDown" => Control::Direction(Direction::South),
            "Accelerate" => Control::Direction(Direction::East),
            "Decelerate" => Control::Direction(Direction::West),
            "Wait" => Control::Wait,
            "Fire" => Control::Fire,
            "Inventory" => Control::Inventory,
            "Status" => Control::Status,
            "DisplayMessageLog" => Control::DisplayMessageLog,
            "Pause" => Control::Pause,
            _ => panic!("No such control: {}", s),
        }
    }
}

impl From<Control> for String {
    fn from(control: Control) -> Self {
        match control {
            Control::Direction(Direction::North) => "SteerUp",
            Control::Direction(Direction::South) => "SteerDown",
            Control::Direction(Direction::East) => "Accelerate",
            Control::Direction(Direction::West) => "Decelerate",
            Control::Wait => "Wait",
            Control::Fire => "Fire",
            Control::Inventory => "Inventory",
            Control::Status => "Status",
            Control::DisplayMessageLog => "DisplayMessageLog",
            Control::Pause => "Pause",
            _ => panic!("Unencodable control: {:?}", control),
        }.to_string()
    }
}

impl From<InputEvent> for String {
    fn from(input: InputEvent) -> Self {
        match input {
            InputEvent::Char(c) => return format!("{}", c),
            InputEvent::Up => "Up",
            InputEvent::Down => "Down",
            InputEvent::Left => "Left",
            InputEvent::Right => "Right",
            InputEvent::Escape => "Escape",
            InputEvent::Return => "Return",
            InputEvent::Space => "Space",
            InputEvent::Quit => panic!("Quit is not a key"),
        }.to_string()
    }
}

impl<'a> From<&'a String> for InputEvent {
    fn from(s: &'a String) -> Self {
        match s.as_ref() {
            "Up" => return InputEvent::Up,
            "Down" => return InputEvent::Down,
            "Left" => return InputEvent::Left,
            "Right" => return InputEvent::Right,
            "Escape" => return InputEvent::Escape,
            "Return" => return InputEvent::Return,
            "Space" => return InputEvent::Space,
            _ => (),
        }

        if s.len() == 1 {
            if let Some(c) = s.chars().next() {
                return InputEvent::Char(c);
            }
        }

        panic!("No such input: {}", s);
    }
}
