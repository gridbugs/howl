use game::*;
use ecs::*;

pub fn driving(env: RuleEnv, action: &EcsAction, _reactions: &mut Vec<Reaction>) -> RuleResult {

    if !action.contains_steer() {
        return RULE_ACCEPT;
    }

    for (entity_id, _) in action.position_profile().insertion_copy_iter() {
        let speed = env.ecs.current_speed(entity_id).expect("Expected component current_speed");
        if speed == 0 {
            // can't steer if speed is 0
            return RULE_REJECT;
        }
    }

    RULE_ACCEPT
}
