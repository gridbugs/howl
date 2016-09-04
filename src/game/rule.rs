use game::{
    EntityContext,
    UpdateSummary,
};

#[derive(Clone, Copy)]
pub struct RuleContext<'a> {
    pub update: &'a UpdateSummary,
    pub entities: &'a EntityContext,
}

impl<'a> RuleContext<'a> {
    pub fn new(update: &'a UpdateSummary, entities: &'a EntityContext) -> Self {
        RuleContext {
            update: update,
            entities: entities,
        }
    }
}

pub enum RuleResult {
    After(Vec<(u64, UpdateSummary)>),
    Instead(Vec<(u64, UpdateSummary)>),
}

// Helper functions
pub fn pass() -> RuleResult { RuleResult::After(vec![]) }
pub fn fail() -> RuleResult { RuleResult::Instead(vec![]) }

pub fn instead(update: UpdateSummary) -> RuleResult {
    RuleResult::Instead(vec![(0, update)])
}

pub fn after(update: UpdateSummary) -> RuleResult {
    RuleResult::After(vec![(0, update)])
}

pub trait Rule {
    fn check(&self, RuleContext) -> RuleResult;
}

// Default implementation for functions
impl<F: Fn(RuleContext) -> RuleResult> Rule for F {
    fn check(&self, ctx: RuleContext) -> RuleResult {
        self(ctx)
    }
}
