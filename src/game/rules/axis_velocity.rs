use game::{
    actions,
    UpdateSummary,
    EntityTable,
    RuleResult,
    ComponentType,
};

use geometry::Direction;

/// When a new entity with velocity is added, add an update that
/// moves it under its velocity.
pub fn start_velocity_movement(summary: &UpdateSummary,
                               entities: &EntityTable)
    -> RuleResult
{
    let mut reactions = Vec::new();
    for (_, entity) in &summary.added_entities {
        if entity.has(ComponentType::AxisVelocity) {
            reactions.push(actions::axis_velocity_move(entity, Direction::North, 1.0));
        }
    }

    RuleResult::After(reactions)
}
