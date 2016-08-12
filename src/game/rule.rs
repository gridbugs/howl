use game::entity::EntityTable;
use game::update::{UpdateProgram, UpdateSummary_};

pub enum RuleResult {
    After(Vec<UpdateSummary_>),
    Instead(Vec<UpdateSummary_>),
}

pub fn pass() -> RuleResult { RuleResult::After(vec![]) }
pub fn fail() -> RuleResult { RuleResult::Instead(vec![]) }

pub trait Rule {
    fn check(&self,
             summary: &UpdateSummary_,
             entities: &EntityTable)
        -> RuleResult;
}

impl<F: Fn(&UpdateSummary_, &EntityTable) -> RuleResult> Rule for F {
    fn check(&self,
             summary: &UpdateSummary_,
             entities: &EntityTable)
        -> RuleResult
    {
        self(summary, entities)
    }
}
