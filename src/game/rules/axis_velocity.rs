use game::{
    actions,
    UpdateSummary,
    EntityTable,
    RuleResult,
};

pub fn maintain_velocity_movement(summary: &UpdateSummary,
                                  entities: &EntityTable)
    -> RuleResult
{
    let mut reactions = Vec::new();

    if summary.metadata.is_axis_velocity() {
        for (entity_id, _) in &summary.added_components {
            let entity = entities.get(*entity_id);
            if let Some((direction, speed)) = entity.axis_velocity() {
                reactions.push(actions::axis_velocity_move(entity, direction, speed));
            }
        }
    }

    RuleResult::After(reactions)
}

/// When a new entity with velocity is added, add an update that
/// moves it under its velocity.
pub fn start_velocity_movement(summary: &UpdateSummary,
                               _: &EntityTable)
    -> RuleResult
{
    let mut reactions = Vec::new();
    for (_, entity) in &summary.added_entities {
        if let Some((direction, speed)) = entity.axis_velocity() {
            reactions.push(actions::axis_velocity_move(entity, direction, speed));
        }
    }

    RuleResult::After(reactions)
}
