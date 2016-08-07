use game::entity::EntityId;
use game::entity::Component::*;
use game::update::Action;
use game::update::{UpdateProgram, UpdateProgramFn};
use game::update::UpdateStatement::*;
use game::updates;

use game::game_entity::GameEntity;

use geometry::direction::Direction;

pub fn walk_(entity_id: EntityId, direction: Direction) -> Action {
    updates::set_entity_component(move |entities| {
        let mut vec = entities.get(entity_id).position().unwrap();
        vec += direction.vector().convert::<isize>();

        (entity_id, Position(vec))
    })
}

pub fn walk(entity_id: EntityId, direction: Direction) -> UpdateProgramFn {
    Box::new(move |entities| {
        let mut vec = entities.get(entity_id).position().unwrap();
        vec += direction.vector().convert::<isize>();

        UpdateProgram::new(vec![
            SetEntityComponent(entity_id, Position(vec)),
        ])
    })
}
