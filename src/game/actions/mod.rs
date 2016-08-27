use game::{
    EntityId,
    Entity,
    UpdateSummary,
    EntityTable,
};
use game::Component::*;
use game::ComponentType as CType;

use game::components::DoorState;
use game::entities;

use geometry::direction::Direction;
use renderer::Tile;
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
    summary.add_component(door_id, Opacity(0.0));

    summary
}

pub fn close_door(door_id: EntityId) -> UpdateSummary {
    let mut summary = UpdateSummary::new();

    summary.add_component(door_id, Solid);
    summary.remove_component(door_id, CType::TransparentTile);
    summary.add_component(door_id, SolidTile {
        tile: Tile::new('+', ansi::WHITE),
        background: ansi::DARK_GREY,
    });
    summary.add_component(door_id, Door(DoorState::Closed));
    summary.add_component(door_id, Opacity(1.0));

    summary
}

pub fn fire_bullet(source: &Entity, direction: Direction, entities: &EntityTable) -> UpdateSummary {
    let mut summary = UpdateSummary::new();

    let start_coord = source.position().unwrap() + direction.vector();
    let level = source.on_level().unwrap();

    let bullet = entities::make_bullet(start_coord.x, start_coord.y, level);

    summary.add_entity(entities.reserve_id(), bullet);

    summary
}
