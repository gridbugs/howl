use std::collections::HashMap;

use frontends::InputEvent;
use direction::Direction;

#[derive(Clone, Copy)]
pub enum Control {
    Direction(Direction),
    Close,
    Fire,
    Quit,
}

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

        map.insert(InputEvent::Char('h'), Control::Direction(Direction::West));
        map.insert(InputEvent::Char('j'), Control::Direction(Direction::South));
        map.insert(InputEvent::Char('k'), Control::Direction(Direction::North));
        map.insert(InputEvent::Char('l'), Control::Direction(Direction::East));
        map.insert(InputEvent::Char('y'), Control::Direction(Direction::NorthWest));
        map.insert(InputEvent::Char('u'), Control::Direction(Direction::NorthEast));
        map.insert(InputEvent::Char('b'), Control::Direction(Direction::SouthWest));
        map.insert(InputEvent::Char('n'), Control::Direction(Direction::SouthEast));

        map.insert(InputEvent::Char('c'), Control::Close);
        map.insert(InputEvent::Char('f'), Control::Fire);

        map.insert(InputEvent::Char('q'), Control::Quit);
        map.insert(InputEvent::Quit, Control::Quit);

        ControlMap {
            map: map,
        }
    }

    pub fn control(&self, event: InputEvent) -> Option<Control> {
        self.map.get(&event).map(|r| *r)
    }
}
