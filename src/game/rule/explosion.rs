use game::*;
use ecs_content::*;
use content_types::{ActionArgs, Reaction};

pub fn explosion_destroy(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, position) in action.copy_iter_position() {

        if env.ecs.contains_explosion(entity_id) {
            for id in env.spatial_hash.get(position).iter_destroyed_by_explosion() {
                reactions.push(Reaction::new(ActionArgs::Destroy(id), 0));
            }
        }
    }

    RULE_ACCEPT
}
