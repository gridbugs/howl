use game::{
    ReserveEntityId,
    UpdateSummary,
    Level,
};

use std::collections::{
    VecDeque,
    vec_deque,
};

pub type Drain<'a> = vec_deque::Drain<'a, Reaction>;

#[derive(Clone, Copy)]
pub struct RuleContext<'a> {
    pub update: &'a UpdateSummary,
    pub level: &'a Level,
    pub ids: &'a ReserveEntityId,
}

impl<'a> RuleContext<'a> {
    pub fn new(update: &'a UpdateSummary, level: &'a Level, ids: &'a ReserveEntityId) -> Self {
        RuleContext {
            update: update,
            level: level,
            ids: ids,
        }
    }
}

#[derive(PartialEq)]
pub enum RuleDecision {
    Accept,
    Reject,
}

pub struct RuleResult {
    decision: RuleDecision,
    then: VecDeque<Reaction>,
}

impl RuleResult {
    fn new(decision: RuleDecision, mut then: Vec<Reaction>) -> Self {
        RuleResult {
            decision: decision,
            then: then.drain(..).collect(),
        }
    }

    pub fn instead_many(then: Vec<Reaction>) -> Self {
        Self::new(RuleDecision::Reject, then)
    }

    pub fn after_many(then: Vec<Reaction>) -> Self {
        Self::new(RuleDecision::Accept, then)
    }

    pub fn fail() -> RuleResult {
        Self::instead_many(vec![])
    }

    pub fn pass() -> RuleResult {
        Self::after_many(vec![])
    }

    pub fn instead(update: UpdateSummary) -> RuleResult {
        Self::instead_many(vec![Reaction::new(update)])
    }

    pub fn after(update: UpdateSummary) -> RuleResult {
        Self::after_many(vec![Reaction::new(update)])
    }

    pub fn is_accept(&self) -> bool {
        self.decision == RuleDecision::Accept
    }

    pub fn is_reject(&self) -> bool {
        self.decision == RuleDecision::Reject
    }

    pub fn drain_reactions(&mut self) -> Drain {
        self.then.drain(..)
    }

    pub fn then(self) -> VecDeque<Reaction> {
        self.then
    }
}

pub struct Reaction {
    pub delay: u64,
    pub action: UpdateSummary,
}

impl Reaction {
    pub fn with_delay(action: UpdateSummary, delay: u64) -> Self {
        Reaction {
            delay: delay,
            action: action,
        }
    }

    pub fn new(action: UpdateSummary) -> Self {
        Self::with_delay(action, 0)
    }
}

pub trait Rule {
    fn check(&self, RuleContext) -> RuleResult;
    fn name(&self) -> &'static str {
        "unnamed"
    }
}
