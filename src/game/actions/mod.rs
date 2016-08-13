use game::entity::{EntityId, Entity};
use game::entity::Component::*;
use game::entity::ComponentType as CType;
use game::update::UpdateSummary;

use game::game_entity::GameEntity;

use game::components::door::DoorState;

use geometry::direction::Direction;
use renderer::tile::Tile;
use colour::ansi;

pub fn walk(entity: &Entity, direction: Direction) -> UpdateSummary {
    let mut summary = UpdateSummary::new();

    let vec = entity.position().unwrap() + direction.vector().convert::<isize>();
    summary.add_component(entity.id.unwrap(), Position(vec));

    summary
}

pub fn open_door(door_id: EntityId) -> UpdateSummary {
    let mut summary = UpdateSummary::new();

    summary.remove_component(door_id, CType::Solid);
    summary.remove_component(door_id, CType::SolidTile);
    summary.add_component(door_id, TransparentTile(Tile::new('-', ansi::WHITE)));
    summary.add_component(door_id, Door(DoorState::Open));

    summary
}
