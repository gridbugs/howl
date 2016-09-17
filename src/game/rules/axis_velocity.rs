use game::{
    actions,
    RuleResult,
    Reaction,
    RuleContext,
    EntityWrapper,
    EntityStore,
};

use game::rule::Rule;

pub struct MaintainVelocityMovement;

impl Rule for MaintainVelocityMovement {
    fn check(&self, ctx: RuleContext) -> RuleResult {
        let mut reactions = Vec::new();

        if ctx.update.is_axis_velocity() {
            for (entity_id, _) in &ctx.update.added_components {
                let entity = ctx.level.get(*entity_id).unwrap();
                if let Some((direction, speed)) = entity.axis_velocity() {
                    let position = entity.position().unwrap();
                    reactions.push(
                        Reaction::new(
                            actions::axis_velocity_move(
                                *entity_id,
                                position,
                                direction,
                                speed)));
                }
            }
        }

        RuleResult::after_many(reactions)
    }
}

/// When a new entity with velocity is added, add an update that
/// moves it under its velocity.
pub struct StartVelocityMovement;

impl Rule for StartVelocityMovement {
    fn check(&self, ctx: RuleContext) -> RuleResult {
        let mut reactions = Vec::new();
        for (id, entity) in &ctx.update.added_entities {
            if let Some((direction, speed)) = entity.axis_velocity() {
                let position = entity.position().unwrap();
                reactions.push(
                    Reaction::new(
                        actions::axis_velocity_move(
                            *id,
                            position,
                            direction,
                            speed)));
            }
        }

        RuleResult::after_many(reactions)
    }
}
