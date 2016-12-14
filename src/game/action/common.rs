use ecs::*;
use game::*;
use game::data::*;
use direction::{self, Direction};
use frontends::ansi;

pub fn wait(action: &mut EcsAction) -> Result<()> {
    action.set_turn_time(1);
    Ok(())
}

pub fn walk(action: &mut EcsAction, entity: EntityRef, direction: Direction) -> Result<()> {
    let current_position = entity.position().ok_or(Error::MissingComponent)?;
    let new_position = current_position + direction.vector();
    action.insert_position(entity.id(), new_position);

    let walk_delay = entity.walk_delay().unwrap_or(1);
    action.set_turn_time(walk_delay);
    Ok(())
}

pub fn open_door(action: &mut EcsAction, door: EntityRef) -> Result<()> {
    action.remove_solid(door.id());
    action.insert_opacity(door.id(), 0.0);
    action.insert_door_state(door.id(), DoorState::Open);

    action.insert_ansi_tile(door.id(),
        ansi::full('-', ansi::colours::WHITE, ansi::colours::DARK_GREY, ansi::styles::NONE));

    action.set_turn_time(1);
    Ok(())
}

pub fn close_door(action: &mut EcsAction, door: EntityRef) -> Result<()> {
    action.insert_solid(door.id());
    action.insert_opacity(door.id(), 1.0);
    action.insert_door_state(door.id(), DoorState::Closed);

    action.insert_ansi_tile(door.id(),
        ansi::full('+', ansi::colours::WHITE, ansi::colours::DARK_GREY, ansi::styles::NONE));

    action.set_turn_time(1);
    Ok(())
}

pub fn close(action: &mut EcsAction, entity_id: EntityId, direction: Direction) -> Result<()> {
    action.set_close(Close::new(entity_id, direction));
    Ok(())
}

pub fn fire_bullet(action: &mut EcsAction, entity: EntityRef, direction: Direction, ids: &EntityIdReserver) -> Result<()> {

    let firer_position = entity.position().ok_or(Error::MissingComponent)?;
    let bullet_position = firer_position + direction.vector();

    prototypes::bullet(action.entity_mut(ids.new_id()), bullet_position, direction);

    Ok(())
}

pub fn burst_bullets(action: &mut EcsAction, entity_id: EntityId, direction: Direction, count: usize) -> Result<()> {

    action.set_burst_fire(BurstFire::new(entity_id, direction, count));

    Ok(())
}

pub fn explode_bullets(action: &mut EcsAction, entity: EntityRef, ids: &EntityIdReserver) -> Result<()> {

    let firer_position = entity.position().ok_or(Error::MissingComponent)?;

    for direction in direction::iter() {
        let bullet_position = firer_position + direction.vector();
        prototypes::bullet(action.entity_mut(ids.new_id()), bullet_position, direction);
    }

    Ok(())
}

pub fn realtime_axis_velocity_move(action: &mut EcsAction, entity: EntityRef, velocity: RealtimeAxisVelocity) -> Result<()> {

    let current_position = entity.position().ok_or(Error::MissingComponent)?;

    action.insert_position(entity.id(), current_position + velocity.direction.vector());

    action.set_action_time_ms(velocity.speed.ms_per_cell());

    Ok(())
}

pub fn destroy(action: &mut EcsAction, entity: EntityRef) -> Result<()> {

    action.remove_entity(entity);

    Ok(())
}

pub fn move_clouds(action: &mut EcsAction, entity_id: EntityId, ecs: &EcsCtx, _spatial_hash: &SpatialHashTable) -> Result<()> {

    let mut cloud_state = ecs.cloud_state_borrow_mut(entity_id).ok_or(Error::MissingComponent)?;

    cloud_state.progress(1.0);

    for _moon_query_result in ecs.query_moon() {

    }

    action.set_turn_time(20);

    Ok(())
}
