use game::entity::EntityId;
use game::entity::Component::*;
use game::updates;
use game::update::monad::Action;

use game::game_entity::GameEntity;

use geometry::direction::Direction;

pub fn walk(entity_id: EntityId, direction: Direction) -> Action {
    updates::set_entity_component(move |entities| {
        let mut vec = entities.get(entity_id).position().unwrap();
        vec += direction.vector().convert::<isize>();

        (entity_id, Position(vec))
    })
}
