use ecs::*;
use game::*;

pub fn transform_terror_pillar_terror_fly(action: &mut EcsAction, entity: EntityRef) -> Result<()> {

    let shadow_id = entity.shadow_entity().ok_or(Error::MissingComponent)?;
    action.swap_tile(entity.id(), shadow_id);
    action.swap_turn_time(entity.id(), shadow_id);
    action.swap_transformation_state(entity.id(), shadow_id);

    Ok(())
}

pub fn transform_tree(action: &mut EcsAction, entity: EntityRef) -> Result<()> {

    let shadow_id = entity.shadow_entity().ok_or(Error::MissingComponent)?;
    action.swap_transformation_state(entity.id(), shadow_id);
    action.swap_tile(entity.id(), shadow_id);
    action.swap_opacity(entity.id(), shadow_id);

    Ok(())
}
