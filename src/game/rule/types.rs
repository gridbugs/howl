use std::result;

use ecs_content::*;
use spatial_hash::*;
use content_types::ActionArgs;

pub type RuleResult = result::Result<(), RuleResolution>;

pub enum RuleResolution {
    Accept,
    Reject,
    Consume(ActionArgs),
}

pub const RULE_ACCEPT: RuleResult = Ok(());
pub const RULE_REJECT: RuleResult = Err(RuleResolution::Reject);
pub const RULE_FORCE: RuleResult = Err(RuleResolution::Accept);

pub fn rule_consume(action_args: ActionArgs) -> RuleResult {
    Err(RuleResolution::Consume(action_args))
}

#[derive(Clone, Copy)]
pub struct RuleEnv<'a> {
    pub ecs: &'a EcsCtx,
    pub spatial_hash: &'a SpatialHashTable,
}
