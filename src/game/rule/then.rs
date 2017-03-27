use game::*;
use ecs_content::*;
use content_types::Reaction;

pub fn then(_env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    if let Some(reaction) = action.get_property_copy_then() {
        reactions.push(reaction);
    }

    RULE_ACCEPT
}
