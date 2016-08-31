use game::{
    rule,
    RuleResult,
    RuleContext,
};

pub fn delay(ctx: RuleContext)
    -> RuleResult
{
    if let Some(update) = ctx.update.delay() {
        rule::after(update.clone())
    } else {
        rule::pass()
    }
}
