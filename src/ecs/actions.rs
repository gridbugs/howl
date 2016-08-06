use ecs::entity::{Entity, EntityTable, EntityId, Component};
use ecs::message::Message;
use ecs::entity::Component::*;
use ecs::entity::ComponentType as CType;
use ecs::update::Update::*;
use ecs::update::UpdateSummary;
use ecs::update;
use ecs::message::Field;
use ecs::update_monad::{UpdateMonad, Action};

use ecs;

use game::game_entity::GameEntity;

use geometry::direction::Direction;

use std::mem;

pub fn walk(entity_id: EntityId, direction: Direction) -> Action {
    UpdateMonad::new(move |summary, entities| {
        let mut vec = entities.get(entity_id).position().unwrap();
        vec += direction.vector().convert::<isize>();
        update::set_entity_component(summary, entities, entity_id, Position(vec));
    })
}
