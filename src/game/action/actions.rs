use ecs::*;
use game::*;
use direction::Direction;

pub fn walk(action: &mut EcsAction, entity: EntityRef, direction: Direction) -> Result<()> {
    let current_position = entity.position().ok_or(Error::MissingComponent)?;
    let new_position = current_position + direction.vector();
    action.insert_position(entity.id(), new_position);
    Ok(())
}
