use game::*;
use ecs::*;

pub fn then(_env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    if let Some(reaction) = action.then() {
        reactions.push(reaction);
    }

    RULE_ACCEPT
}
