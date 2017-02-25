use std::path;
use toml;
use game::*;
use direction::*;

const CONTROL_FILE: &'static str = "controls.toml";

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
        "Help" => Control::Help,
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
            let mut control_map = ControlMap::bare();

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
    game_file::read_toml(path).ok().and_then(from_toml)
}
