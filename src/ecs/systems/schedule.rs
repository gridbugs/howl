use ecs::message::Message;
use ecs::message::Field::*;
use ecs::entity::EntityId;

use ecs;

pub struct Schedule {
    entity: EntityId,
}

impl Schedule {
    pub fn new(entity: EntityId) -> Schedule {
        Schedule { entity: entity }
    }

    pub fn schedule(&self) -> Option<Message> {
        Some(message![
            ActorTurn { actor: self.entity },
        ])
    }
}
