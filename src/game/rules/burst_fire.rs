use game::{
    rule,
    actions,
    RuleResult,
    RuleContext,
};

use game::update::Metadatum::*;

pub fn burst_fire(ctx: RuleContext)
    -> RuleResult
{
    if let Some((prototype, count, period)) =
        ctx.update.burst_fire()
    {
        let mut spawn_bullet =
            actions::add_entity(prototype.clone(), ctx.entities);
        let (_, speed) = prototype.axis_velocity().unwrap();

        spawn_bullet.set_metadata(ActionTime(speed.ms_per_cell()));

        let mut reactions = vec![spawn_bullet];

        if count > 0 {
            let mut burst_rest = ctx.update.clone();
            burst_rest.set_metadata(BurstFire {
                prototype: prototype.clone(),
                count: count - 1,
                period: period,
            });
            reactions.push(actions::delay(burst_rest, period));
        }

        RuleResult::After(reactions)
    } else {
        rule::pass()
    }
}
