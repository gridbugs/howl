use game::entity::{EntityId, EntityTable};
use game::entity::Component::*;
use game::entity::ComponentType as CType;
use game::update::UpdateProgram;
use game::update::UpdateStatement::*;

use game::game_entity::GameEntity;

use game::components::door::DoorState;

use geometry::direction::Direction;
use renderer::tile::Tile;
use colour::ansi;

pub fn walk(entity_id: EntityId, direction: Direction, entities: &EntityTable)
    -> UpdateProgram
{
    let mut vec = entities.get(entity_id).position().unwrap();
    vec += direction.vector().convert::<isize>();

    UpdateProgram::new(vec![
        SetComponent(entity_id, Position(vec)),
    ])
}

pub fn open_door(door_id: EntityId)
    -> UpdateProgram
{
    UpdateProgram::new(vec![
        RemoveComponent(door_id, CType::Solid),
        RemoveComponent(door_id, CType::SolidTile),
        AddComponent(door_id, TransparentTile(Tile::new('-', ansi::WHITE))),
        SetComponent(door_id, Door(DoorState::Open)),
    ])
}
