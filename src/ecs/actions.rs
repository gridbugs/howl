use ecs::entity::Entity;
use ecs::message::Message;
use ecs::entity::Component::*;
use ecs::entity::ComponentType as CType;
use ecs::update::Update::*;
use ecs::message::Field;

use ecs;

use geometry::direction::Direction;

pub fn walk(entity: &Entity, direction: Direction) -> Option<Message> {

    if let Some(&Position(position)) = entity.get(CType::Position) {
        Some(message![
            Field::Update(SetEntityComponent {
                entity_id: entity.id.unwrap(),
                component_type: CType::Position,
                component_value: Position(position + direction.vector().convert::<isize>()),
            }),
        ])
    } else {
        None
    }
}
