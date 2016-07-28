use ecs::message_queue::MessageQueue;
use ecs::message::Message;
use ecs::message::Field::*;
use ecs::message::FieldType;
use ecs::entity::{EntityId, EntityTable};
use ecs::entity::ComponentType as Type;

use ecs;

pub fn schedule_player_turn(entity: EntityId,
                            entities: &mut EntityTable,
                            message: &Message,
                            message_queue: &mut MessageQueue) {

    if let Some(&NewTurn) = message.get(FieldType::NewTurn) {
        let mut message = message![
            ActorTurn { actor: entity },
        ];

        if entities.get(entity).has(Type::PlayerActor) {
            // for playper characters, render the level before getting input
            message.add(RenderLevel { level: 0 });
        }

        message_queue.enqueue(message);
    }
}
