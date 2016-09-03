use game::{
    actions,
    rule,
    RuleResult,
    RuleContext,
};

pub fn beast_transformation(ctx: RuleContext) -> RuleResult {
    for (entity_id, changes) in &ctx.update.added_components {
        if let Some(counter) = changes.beast_transform() {
            if counter.is_zero() {
                return rule::instead(actions::beast_transform(*entity_id));
            }
        }
    }

    rule::pass()
}

pub fn human_transformation(ctx: RuleContext) -> RuleResult {
    for (entity_id, changes) in &ctx.update.added_components {
        if let Some(counter) = changes.human_transform() {
            if counter.is_zero() {
                return rule::instead(actions::human_transform(*entity_id));
            }
        }
    }

    rule::pass()
}
