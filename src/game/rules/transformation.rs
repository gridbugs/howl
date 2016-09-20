use game::{actions, Rule, RuleResult, RuleContext, EntityWrapper};

pub struct BeastTransformation;

impl Rule for BeastTransformation {
    fn check(&self, ctx: RuleContext) -> RuleResult {
        for (entity_id, changes) in &ctx.update.added_components {
            if let Some(counter) = changes.beast_transform() {
                if counter.is_zero() {
                    return RuleResult::instead(actions::beast_transform(*entity_id));
                }
            }
        }

        RuleResult::pass()
    }
}

pub struct HumanTransformation;

impl Rule for HumanTransformation {
    fn check(&self, ctx: RuleContext) -> RuleResult {
        for (entity_id, changes) in &ctx.update.added_components {
            if let Some(counter) = changes.human_transform() {
                if counter.is_zero() {
                    return RuleResult::instead(actions::human_transform(*entity_id));
                }
            }
        }

        RuleResult::pass()
    }
}
