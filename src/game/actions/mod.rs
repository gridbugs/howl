use game::{
    EntityId,
    Entity,
    UpdateSummary,
    EntityTable,
    Speed,
};
use game::Component::*;
use game::ComponentType as CType;

use game::update::Metadatum::*;

use game::components::DoorState;
use game::entities;

use geometry::direction::Direction;
use geometry::direction;
use renderer::Tile;
use colour::ansi;

pub fn walk(entity: &Entity, direction: Direction) -> UpdateSummary {
    let mut summary = UpdateSummary::new();

    let vec = entity.position().unwrap() + direction.vector().convert::<isize>();
    summary.add_component(entity.id.unwrap(), Position(vec));

    summary.set_metadata(Name("walk"));
    summary
}

pub fn open_door(door_id: EntityId) -> UpdateSummary {
    let mut summary = UpdateSummary::new();

    summary.remove_component(door_id, CType::Solid);
    summary.remove_component(door_id, CType::SolidTile);
    summary.add_component(door_id, TransparentTile(Tile::new('-', ansi::WHITE)));
    summary.add_component(door_id, Door(DoorState::Open));
    summary.add_component(door_id, Opacity(0.0));

    summary.set_metadata(Name("open_door"));
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

    summary.set_metadata(Name("close_door"));
    summary
}

pub fn fire_single_bullet(source: &Entity, direction: Direction, entities: &EntityTable) -> UpdateSummary {
    let mut summary = UpdateSummary::new();

    let start_coord = source.position().unwrap() + direction.vector();
    let level = source.on_level().unwrap();

    let mut bullet = entities::make_bullet(start_coord.x, start_coord.y, level);

    let speed = Speed::from_cells_per_sec(100.0);

    bullet.add(AxisVelocity { direction: direction, speed: speed });

    summary.add_entity(entities.reserve_id(), bullet);

    summary.set_metadata(ActionTime(speed.ms_per_cell()));

    summary.set_metadata(Name("fire_single_bullet"));
    summary
}

pub fn burst_fire_bullet(source: &Entity, direction: Direction,
                         num_bullets: u64, period: u64)
    -> UpdateSummary
{
    let mut summary = UpdateSummary::new();

    let start_coord = source.position().unwrap() + direction.vector();
    let level = source.on_level().unwrap();

    let mut bullet = entities::make_bullet(start_coord.x, start_coord.y, level);
    let speed = Speed::from_cells_per_sec(100.0);
    bullet.add(AxisVelocity { direction: direction, speed: speed });

    summary.set_metadata(BurstFire {
        prototype: bullet,
        count: num_bullets,
        period: period,
    });
    summary.set_metadata(ActionTime(speed.ms_per_cell()));

    summary.set_metadata(Name("burst_fire_bullet"));
    summary
}

pub fn fire_bullets_all_axes(source: &Entity, entities: &EntityTable) -> UpdateSummary {
    let mut summary = UpdateSummary::new();

    let level = source.on_level().unwrap();
    let speed = Speed::from_cells_per_sec(100.0);

    for dir in direction::iter() {
        let start_coord = source.position().unwrap() + dir.vector();

        let mut bullet = entities::make_bullet(start_coord.x, start_coord.y, level);
        bullet.add(AxisVelocity { direction: dir, speed: speed });

        summary.add_entity(entities.reserve_id(), bullet);
    }

    summary.set_metadata(ActionTime(speed.ms_per_cell()));

    summary.set_metadata(Name("fire_bullets_all_axes"));
    summary
}

pub fn axis_velocity_move(entity: &Entity, direction: Direction, speed: Speed) -> UpdateSummary {
    let mut summary = UpdateSummary::new();

    let vec = entity.position().unwrap() + direction.vector().convert::<isize>();
    summary.add_component(entity.id.unwrap(), Position(vec));

    summary.set_metadata(ActionTime(speed.ms_per_cell()));
    summary.set_metadata(AxisVelocityMovement);

    summary.set_metadata(Name("axis_velocity_move"));
    summary
}

pub fn add_entity(entity: Entity, entities: &EntityTable) -> UpdateSummary {
    let mut summary = UpdateSummary::new();

    summary.add_entity(entities.reserve_id(), entity);

    summary.set_metadata(Name("add_entity"));
    summary
}

pub fn remove_entity(entity: &Entity) -> UpdateSummary {
    let mut summary = UpdateSummary::new();

    summary.remove_entity(entity.id.unwrap());

    summary.set_metadata(Name("remove_entity"));
    summary
}

pub fn delay(update: UpdateSummary, time_ms: u64) -> UpdateSummary {
    let mut summary = UpdateSummary::new();

    summary.set_metadata(Delay(update));
    summary.set_metadata(ActionTime(time_ms));

    summary.set_metadata(Name("delay"));
    summary
}
