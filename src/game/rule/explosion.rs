use game::*;
use ecs::*;

pub fn explosion_destroy(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, position) in action.position_profile().insertion_copy_iter() {

        if env.ecs.contains_explosion(entity_id) {
            for id in env.spatial_hash.get(position).destroyed_by_explosion_iter() {
                reactions.push(Reaction::new(ActionArgs::Destroy(id), 0));
            }
        }
    }

    RULE_ACCEPT
}
