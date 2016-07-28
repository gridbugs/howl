use ecs::message::Message;
use ecs::entity::EntityTable;
use ecs::message::Field;
use ecs::message::FieldType as FType;
use ecs::actions;

use ecs;

use rustty::Event;
use terminal::window_manager::InputSource;
use geometry::direction::Direction;

const ETX: char = '\u{3}';

pub fn get_action<'a>(input_source: &InputSource<'a>,
                                      entities: &EntityTable,
                                      message: &Message)
    -> Option<Message>
{
    if let Some(&Field::ActorTurn { actor: actor_id }) = message.get(FType::ActorTurn) {
        let entity = entities.get(actor_id);
        if let Some(event) = input_source.get_event() {
            if let Some(direction) = event_to_direction(event) {
                actions::walk(entity, direction)
            } else {
                event_to_message(event)
            }
        } else {
            None
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

fn event_to_message(event: Event) -> Option<Message> {
    match event {
        Event::Char(ETX) => Some(message![ Field::QuitGame ]),
        Event::Char('q') => Some(message![ Field::QuitGame ]),
        _ => None,
    }
}
