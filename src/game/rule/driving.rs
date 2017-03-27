use game::*;
use ecs_content::*;
use content_types::Reaction;

pub fn driving(env: RuleEnv, action: &EcsAction, _reactions: &mut Vec<Reaction>) -> RuleResult {

    for entity_id in action.id_iter_steering() {
        let speed = env.ecs.get_copy_current_speed(entity_id).expect("Expected component current_speed");
        if speed == 0 {
            // can't steer if speed is 0
            return RULE_REJECT;
        }
    }

    RULE_ACCEPT
}
