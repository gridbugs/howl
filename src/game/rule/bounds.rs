use game::*;
use ecs::*;


pub fn bounds(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {
    let mut reject = false;

    for (entity_id, position) in action.position_profile().insertion_copy_iter() {
        if !env.spatial_hash.is_valid_coord(position) {
            // destroy entities that move out of bounds
            reactions.push(Reaction::new(ActionArgs::Destroy(entity_id), 0));
            reject = true;
        }
    }

    if reject {
        RULE_REJECT
    } else {
        RULE_ACCEPT
    }
}
