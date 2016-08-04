use ecs::update::Update;
use ecs::message::{
    Message,
    FieldType,
    Field,
};
use ecs::entity::{
    Entity,
    EntityId,
    Component,
    ComponentType,
};
use ecs::components::level::Level;

use geometry::vector2::Vector2;

pub fn get_update(message: &Message) -> Option<&Update> {
    if let Some(&Field::Update(ref update)) = message.get(FieldType::Update) {
        Some(update)
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

pub fn get_level(entity: &Entity) -> Option<EntityId> {
    if let Some(&Component::OnLevel(level_id)) = entity.get(ComponentType::OnLevel) {
        Some(level_id)
    } else {
        None
    }
}

pub fn get_level_data(entity: &Entity) -> Option<&Level> {
     if let Some(&Component::LevelData(ref level)) = entity.get(ComponentType::LevelData) {
        Some(level)
    } else {
        None
    }
}

pub fn get_mut_level_data(entity: &mut Entity) -> Option<&mut Level> {
     if let Some(&mut Component::LevelData(ref mut level)) = entity.get_mut(ComponentType::LevelData) {
        Some(level)
    } else {
        None
    }
}
