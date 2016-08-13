use game::entity::{EntityId, EntityTable};
use game::actions;
use game::control::Control;

use rustty::Event;
use terminal::window_manager::InputSource;
use geometry::direction::Direction;

const ETX: char = '\u{3}';

pub fn get_control<'a>(input_source: &InputSource<'a>,
                       entity_id: EntityId,
                       entities: &EntityTable)
    -> Option<Control>
{
    if let Some(event) = input_source.get_event() {
        if let Some(direction) = event_to_direction(event) {
            Some(Control::Update(actions::walk(entities.get(entity_id), direction)))
        } else {
            event_to_control(event)
        }
    } else {
        None
    }
}

fn event_to_direction(event: Event) -> Option<Direction> {
    match event {
        // Arrow keys
        Event::Up => Some(Direction::North),
        Event::Down => Some(Direction::South),
        Event::Right => Some(Direction::East),
        Event::Left => Some(Direction::West),

        // Vi keys
        Event::Char('k') => Some(Direction::North),
        Event::Char('j') => Some(Direction::South),
        Event::Char('l') => Some(Direction::East),
        Event::Char('h') => Some(Direction::West),
        Event::Char('y') => Some(Direction::NorthWest),
        Event::Char('u') => Some(Direction::NorthEast),
        Event::Char('b') => Some(Direction::SouthWest),
        Event::Char('n') => Some(Direction::SouthEast),
        _ => None,
    }
}

fn event_to_control(event: Event) -> Option<Control> {
    match event {
        Event::Char(ETX) => Some(Control::Quit),
        Event::Char('q') => Some(Control::Quit),
        _ => None,
    }
}
