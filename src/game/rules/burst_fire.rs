use game::{
    Rule,
    actions,
    RuleResult,
    Reaction,
    RuleContext,
    EntityWrapper,
};

use game::update::Metadatum::*;

pub struct BurstFireRule;

impl Rule for BurstFireRule {
    fn check(&self, ctx: RuleContext) -> RuleResult {
        if let Some((prototype, count, period)) =
            ctx.update.burst_fire()
        {
            let mut spawn_bullet =
                actions::add_entity(prototype.clone(), ctx.ids);
            let (_, speed) = prototype.axis_velocity().unwrap();

            spawn_bullet.set_metadata(ActionTime(speed.ms_per_cell()));

            let mut reactions = vec![Reaction::new(spawn_bullet)];

            if count > 0 {
                let mut burst_rest = ctx.update.clone();
                burst_rest.set_metadata(BurstFire {
                    prototype: prototype.clone(),
                    count: count - 1,
                    period: period,
                });
                reactions.push(Reaction::with_delay(burst_rest, period));
            }

            RuleResult::after_many(reactions)
        } else {
            RuleResult::pass()
        }
    }
}
