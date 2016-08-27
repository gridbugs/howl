use game::{
    rule,
    UpdateSummary,
    EntityTable,
    RuleResult,
};

pub fn delay(summary: &UpdateSummary,
             _: &EntityTable)
    -> RuleResult
{
    if let Some(update) = summary.metadata.delay() {
        rule::after(update.clone())
    } else {
        rule::pass()
    }
}
