use game::{
    actions,
    RuleResult,
    RuleContext,
};

pub fn maintain_velocity_movement(ctx: RuleContext)
    -> RuleResult
{
    let mut reactions = Vec::new();

    if ctx.update.metadata.is_axis_velocity() {
        for (entity_id, _) in &ctx.update.added_components {
            let entity = ctx.entities.get(*entity_id);
            if let Some((direction, speed)) = entity.axis_velocity() {
                reactions.push(actions::axis_velocity_move(entity, direction, speed));
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
    for (_, entity) in &ctx.update.added_entities {
        if let Some((direction, speed)) = entity.axis_velocity() {
            reactions.push(actions::axis_velocity_move(entity, direction, speed));
        }
    }

    RuleResult::After(reactions)
}
