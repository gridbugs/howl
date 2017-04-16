use std::cmp;

use rand::Rng;
use num::PrimInt;

use math::Coord;
use math::Direction;

use ecs_core::*;
use ecs_content::*;
use game::*;
use content_types::*;
use message::*;
use engine_defs::*;
use tile::TileType;

pub const ENV_TURN_OFFSET: u64 = 0;
pub const NPC_TURN_OFFSET: u64 = 1;
pub const PC_TURN_OFFSET: u64 = 2;
pub const PHYSICS_TURN_OFFSET: u64 = 3;
pub const ANIMATION_TURN_OFFSET: u64 = 4;

pub fn pc<E: EntityMut>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);
    entity.insert_tile(TileType::Player);
    entity.insert_pc();
    entity.insert_should_render();
    entity.insert_vision_distance(10);

    entity
}

pub fn wall<E: EntityMut>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);
    entity.insert_tile(TileType::Wall);
    entity.insert_opacity(1.0);
    entity.insert_solid();

    entity
}

pub fn stone_floor<E: EntityMut>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);
    entity.insert_tile(TileType::StoneFloor);
    entity.insert_floor();

    entity
}
