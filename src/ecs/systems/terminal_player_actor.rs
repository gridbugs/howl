use ecs::system_queue::SystemQueue;
use ecs::message::Message;
use ecs::message_queue::MessageQueue;
use ecs::entity::EntityTable;
use ecs::entity::ComponentType as CType;
use ecs::message::Field;
use ecs::message::FieldType as FType;
use ecs::update::UpdateStage;
use ecs::actions;

use ecs;

use rustty::Event;
use terminal::window_manager::InputSource;
use geometry::direction::Direction;

use std::fmt;

const ETX: char = '\u{3}';

pub struct TerminalPlayerActor<'a> {
    input_source: InputSource<'a>,
}

impl<'a> TerminalPlayerActor<'a> {
    pub fn new(input_source: InputSource<'a>) -> Self {
        TerminalPlayerActor {
            input_source: input_source,
        }
    }

    pub fn process_message(&mut self, message: &mut Message,
                           message_queue: &mut MessageQueue,
                           entities: &mut EntityTable, _: &SystemQueue)
    {
        if let Some(&Field::ActorTurn { actor: actor_id }) = message.get(FType::ActorTurn) {
            let entity = entities.get(actor_id);
            if entity.has(CType::PlayerActor) {
                let event = self.input_source.get_event().unwrap();

                if let Some(direction) = event_to_direction(event) {
                    let mut message = actions::walk(entity, direction).unwrap();

                    message.add(Field::UpdateStage(UpdateStage::Commit));
                    message.add(Field::RenderLevel { level: 0 });

                    message_queue.enqueue(message);
                } else if let Some(message) = event_to_message(event) {
                    //debug_println!("{:?}", message);
                    message_queue.enqueue(message);
                }
            }
        }
    }
}

impl<'a> fmt::Debug for TerminalPlayerActor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TerminalPlayerActor {{}}")
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
