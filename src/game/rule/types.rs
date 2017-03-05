use std::result;

use game::*;
use ecs::*;
use spatial_hash::*;

pub type RuleResult = result::Result<RuleResolution, RuleResolution>;

pub enum RuleResolution {
    Accept,
    Reject,
    Consume(ActionArgs),
}

pub const RULE_ACCEPT: RuleResult = Ok(RuleResolution::Accept);
pub const RULE_REJECT: RuleResult = Err(RuleResolution::Reject);

pub fn rule_consume(action_args: ActionArgs) -> RuleResult {
    Err(RuleResolution::Consume(action_args))
}

#[derive(Clone, Copy)]
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
