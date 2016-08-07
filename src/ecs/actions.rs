use ecs::entity::EntityId;
use ecs::entity::Component::*;
use ecs::update;
use ecs::update_monad::Action;

use game::game_entity::GameEntity;

use geometry::direction::Direction;

pub fn walk(entity_id: EntityId, direction: Direction) -> Action {
    update::set_entity_component(move |entities| {
        let mut vec = entities.get(entity_id).position().unwrap();
        vec += direction.vector().convert::<isize>();

        (entity_id, Position(vec))
    })
}
