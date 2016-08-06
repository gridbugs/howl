use ecs::entity::EntityId;
use ecs::entity::Component::*;
use ecs::update;
use ecs::update_monad::{UpdateMonad, Action};

use game::game_entity::GameEntity;

use geometry::direction::Direction;

pub fn walk(entity_id: EntityId, direction: Direction) -> Action {
    UpdateMonad::new(move |summary, entities| {
        let mut vec = entities.get(entity_id).position().unwrap();
        vec += direction.vector().convert::<isize>();
        update::set_entity_component(summary, entities, entity_id, Position(vec));
    })
}
