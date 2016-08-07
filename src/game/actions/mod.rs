use game::entity::EntityId;
use game::entity::Component::*;
use game::update::{UpdateProgram, UpdateProgramFn};
use game::update::UpdateStatement::*;

use game::game_entity::GameEntity;

use geometry::direction::Direction;

pub fn walk(entity_id: EntityId, direction: Direction) -> UpdateProgramFn {
    Box::new(move |entities| {
        let mut vec = entities.get(entity_id).position().unwrap();
        vec += direction.vector().convert::<isize>();

        UpdateProgram::new(vec![
            SetEntityComponent(entity_id, Position(vec)),
        ])
    })
}
