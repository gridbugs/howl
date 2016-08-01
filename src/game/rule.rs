use ecs::entity::EntityTable;
use ecs::message::Message;
use ecs::update::UpdateSummary;

pub enum RuleResult {
    After(Vec<Message>),
    Instead(Vec<Message>),
}

pub fn pass() -> RuleResult { RuleResult::After(vec![]) }
pub fn fail() -> RuleResult { RuleResult::Instead(vec![]) }

pub trait Rule {
    fn check(&self,
             message: &Message,
             summary: &UpdateSummary,
             before: &EntityTable,
             after: &EntityTable)
        -> RuleResult;
}

impl<F: Fn(&Message, &UpdateSummary, &EntityTable, &EntityTable) -> RuleResult> Rule for F {
    fn check(&self,
             message: &Message,
             summary: &UpdateSummary,
             before: &EntityTable,
             after: &EntityTable)
        -> RuleResult
    {
        self(message, summary, before, after)
    }
}
