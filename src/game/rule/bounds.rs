use game::*;
use ecs::*;


pub fn bounds(env: RuleEnv, action: &EcsAction, _reactions: &mut Vec<Reaction>) -> RuleResult {
    for (_entity_id, position) in action.position_profile().insertion_copy_iter() {
        if !env.spatial_hash.is_valid_coord(position) {
            return RULE_REJECT;
        }
    }
    RULE_ACCEPT
}
