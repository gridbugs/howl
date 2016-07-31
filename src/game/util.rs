use ecs::update::Update;
use ecs::message::{
    Message,
    FieldType,
    Field,
};
use ecs::entity::{
    EntityId,
    Entity,
    Component,
    ComponentType,
};

use geometry::vector2::Vector2;

pub fn get_update(message: &Message) -> Option<&Update> {
    if let Some(&Field::Update(ref update)) = message.get(FieldType::Update) {
        Some(update)
    } else {
        None
    }
}

pub fn get_update_entity(message: &Message) -> Option<EntityId> {
    if let Some(&Field::UpdateEntity(entity)) = message.get(FieldType::UpdateEntity) {
        Some(entity)
    } else {
        None
    }
}

pub fn get_position(entity: &Entity) -> Option<Vector2<isize>> {
     if let Some(&Component::Position(ref vec)) = entity.get(ComponentType::Position) {
        Some(*vec)
    } else {
        None
    }
}
