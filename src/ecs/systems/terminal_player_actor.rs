use ecs::system_queue::SystemQueue;
use ecs::message::Message;
use ecs::entity::EntityTable;
use ecs::entity::ComponentType as CType;
use ecs::message::Field::*;
use ecs::message::FieldType as FType;

use terminal::window_manager::InputSource;

use std::fmt;
use debug;


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
                           entities: &mut EntityTable, _: &SystemQueue)
    {
        if let Some(&ActorTurn { actor: actor_id }) = message.get(FType::ActorTurn) {
            let entity = entities.get(actor_id);
            if entity.has(CType::PlayerActor) {
                debug_println!("player's turn");
            }
        }
    }
}

impl<'a> fmt::Debug for TerminalPlayerActor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TerminalPlayerActor {{}}")
    }
}
