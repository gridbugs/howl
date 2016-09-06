use game::{
    actions,
    RuleResult,
    RuleContext,
    EntityWrapper,
    EntityStore,
};

pub fn maintain_velocity_movement(ctx: RuleContext)
    -> RuleResult
{
    let mut reactions = Vec::new();

    if ctx.update.is_axis_velocity() {
        for (entity_id, _) in &ctx.update.added_components {
            let entity = ctx.level.get(*entity_id).unwrap();
            if let Some((direction, speed)) = entity.axis_velocity() {
                let position = entity.position().unwrap();
                reactions.push((0, actions::axis_velocity_move(*entity_id, position, direction, speed)));
            }
        }
    }

    RuleResult::After(reactions)
}

/// When a new entity with velocity is added, add an update that
/// moves it under its velocity.
pub fn start_velocity_movement(ctx: RuleContext)
    -> RuleResult
{
    let mut reactions = Vec::new();
    for (id, entity) in &ctx.update.added_entities {
        if let Some((direction, speed)) = entity.axis_velocity() {
            let position = entity.position().unwrap();
            reactions.push((0, actions::axis_velocity_move(*id, position, direction, speed)));
        }
    }

    RuleResult::After(reactions)
}
