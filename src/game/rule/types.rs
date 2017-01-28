use std::result;

use game::*;
use ecs::*;
use spatial_hash::*;

pub type RuleResult = result::Result<(), RuleError>;

pub enum RuleError {
    Rejection,
    GameError(Error),
}

impl From<Error> for RuleError {
    fn from(e: Error) -> Self {
        RuleError::GameError(e)
    }
}

pub const RULE_ACCEPT: RuleResult = Ok(());
pub const RULE_REJECT: RuleResult = Err(RuleError::Rejection);

pub struct Reaction {
    pub action: ActionArgs,
    pub delay: u64,
}

#[derive(Clone, Copy)]
pub struct RuleEnv<'a> {
    pub ecs: &'a EcsCtx,
    pub spatial_hash: &'a SpatialHashTable,
}

impl Reaction {
    pub fn new(action: ActionArgs, delay: u64) -> Self {
        Reaction {
            action: action,
            delay: delay,
        }
    }
}
