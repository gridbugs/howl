use std::collections::HashMap;

use frontends::InputEvent;
use direction::Direction;

#[derive(Clone, Copy)]
pub enum Control {
    Direction(Direction),
    Close,
    Fire,
    Explode,
    Burst,
    Wait,
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

        map.insert(InputEvent::Char('c'), Control::Close);
        map.insert(InputEvent::Char('f'), Control::Fire);
        map.insert(InputEvent::Char('F'), Control::Explode);
        map.insert(InputEvent::Char('g'), Control::Burst);

        map.insert(InputEvent::Char('.'), Control::Wait);

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
