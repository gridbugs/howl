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

    Ok(())
}
