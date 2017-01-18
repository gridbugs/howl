use game::*;
use ecs::*;

pub fn collision(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, position) in action.position().insertion_copy_iter() {

        if !env.spatial_hash.get(position).solid() {
            continue;
        }

        let entity = env.ecs.post_action_entity(entity_id, action);

        let mut reject = false;
        if entity.contains_collider() {
            reject = true;
        }
        if entity.contains_destroy_on_collision() {
            reactions.push(Reaction::new(ActionArgs::Destroy(entity_id), 0));
            reject = true;
        }
        if reject {
            return RULE_REJECT;
        }
    }

    RULE_ACCEPT
}
