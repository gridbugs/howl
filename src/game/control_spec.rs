use std::collections::HashMap;
use std::path;
use toml;
use game::*;
use direction::*;

type StringControlSpec = HashMap<String, String>;

impl<'a> From<&'a ControlMap> for StringControlSpec {
    fn from(map: &'a ControlMap) -> Self {
        let mut spec = StringControlSpec::new();

        for (input_event, control) in map.iter() {
            spec.insert(String::from(*control), String::from(*input_event));
        }

        spec
    }
}

impl<'a> From<&'a StringControlSpec> for ControlMap {
    fn from(spec: &'a StringControlSpec) -> Self {
        let mut map = ControlMap::new();

        for (control, input) in spec.iter() {
            map.insert(InputEvent::from(input), Control::from(control));
        }

        map
    }
}

impl<'a> From<&'a String> for Control {
    fn from(s: &'a String) -> Self {
        match s.as_ref() {
            "North" => Control::Direction(Direction::North),
            "South" => Control::Direction(Direction::South),
            "East" => Control::Direction(Direction::East),
            "West" => Control::Direction(Direction::West),
            "Use" => Control::Use,
            "Close" => Control::Close,
            "Examine" => Control::Examine,
            "Wait" => Control::Wait,
            "Fire" => Control::Fire,
            "NextTarget" => Control::NextTarget,
            "PrevTarget" => Control::PrevTarget,
            "DisplayMessageLog" => Control::DisplayMessageLog,
            "Pause" => Control::Pause,
            _ => panic!("No such control: {}", s),
        }
    }
}

impl From<Control> for String {
    fn from(control: Control) -> Self {
        match control {
            Control::Direction(Direction::North) => "North",
            Control::Direction(Direction::South) => "South",
            Control::Direction(Direction::East) => "East",
            Control::Direction(Direction::West) => "West",
            Control::Use => "Use",
            Control::Close => "Close",
            Control::Examine => "Examine",
            Control::Wait => "Wait",
            Control::Fire => "Fire",
            Control::NextTarget => "NextTarget",
            Control::PrevTarget => "PrevTarget",
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

fn parse_control(name: &str) -> Option<Control> {

    let control = match name {
        "North" => Control::Direction(Direction::North),
        "South" => Control::Direction(Direction::South),
        "East" => Control::Direction(Direction::East),
        "West" => Control::Direction(Direction::West),
        "Use" => Control::Use,
        "Close" => Control::Close,
        "Examine" => Control::Examine,
        "Wait" => Control::Wait,
        "Fire" => Control::Fire,
        "NextTarget" => Control::NextTarget,
        "PrevTarget" => Control::PrevTarget,
        "DisplayMessageLog" => Control::DisplayMessageLog,
        "Pause" => Control::Pause,
        _ => return None,
    };

    Some(control)
}

fn parse_input_event(string: &str) -> Option<InputEvent> {
    match string {
        "Up" => return Some(InputEvent::Up),
        "Down" => return Some(InputEvent::Down),
        "Left" => return Some(InputEvent::Left),
        "Right" => return Some(InputEvent::Right),
        "Escape" => return Some(InputEvent::Escape),
        "Return" => return Some(InputEvent::Return),
        _ => {},
    }

    if string.len() == 1 {
        if let Some(c) = string.chars().next() {
            return Some(InputEvent::Char(c));
        }
    }

    None
}

fn from_toml(table: toml::value::Table) -> Option<ControlMap> {
    table.get("controls")
        .and_then(|t| t.as_table())
        .map(|controls| {
            let mut control_map = ControlMap::new();

            for (control_str, input_event_str) in controls.iter() {
                if let Some(control) = parse_control(control_str.as_str()) {
                    if let Some(input_event) = input_event_str.as_str().and_then(parse_input_event) {
                        control_map.insert(input_event, control);
                    }
                }
            }

            control_map
        })
}

pub fn from_file<P: AsRef<path::Path>>(path: P) -> Option<ControlMap> {
    let spec: Option<StringControlSpec> = game_file::read_toml(path).ok();
    spec.as_ref().map(ControlMap::from)
}

pub fn to_file<P: AsRef<path::Path>>(path: P, map: &ControlMap) {
    game_file::write_toml(path, &StringControlSpec::from(map))
        .expect("Failed to write controls file");
}
