use std::f64;
use engine_defs::*;
use rand::Rng;
use ecs_core::*;
use ecs_content::*;
use content_types::*;
use math::Direction;
use math::Coord;
use math::Vector2;

use game::entity::EntityExtra;
use game::prototypes;

pub fn walk(action: &mut EcsAction, entity: EntityRef, direction: Direction) {
    let current_position = entity.copy_position().expect("Entity missing position");
    let new_position = current_position + direction.vector();
    action.insert_position(entity.id(), new_position);
}
