use ecs::*;
use game::*;
use game::data::*;
use direction::Direction;
use frontends::ansi;

pub fn walk(action: &mut EcsAction, entity: EntityRef, direction: Direction) -> Result<()> {
    let current_position = entity.position().ok_or(Error::MissingComponent)?;
    let new_position = current_position + direction.vector();
    action.insert_position(entity.id(), new_position);
    action.set_turn_time(1);
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
