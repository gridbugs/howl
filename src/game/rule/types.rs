use std::result;

use game::*;
use ecs::*;
use spatial_hash::*;

pub type RuleResult = result::Result<(), RuleError>;

pub enum RuleResolution {
    Accept,
    Reject,
    Consume(ActionArgs),
}

pub enum RuleError {
    Resolution(RuleResolution),
    GameError(Error),
}

impl From<Error> for RuleError {
    fn from(e: Error) -> Self {
        RuleError::GameError(e)
    }
}

pub const RULE_ACCEPT: RuleResult = Ok(());
pub const RULE_REJECT: RuleResult = Err(RuleError::Resolution(RuleResolution::Reject));

pub fn rule_consume(action_args: ActionArgs) -> RuleResult {
    Err(RuleError::Resolution(RuleResolution::Consume(action_args)))
}

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
