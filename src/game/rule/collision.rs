use game::*;
use ecs::*;

pub fn collision(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, position) in action.position_profile().insertion_copy_iter() {

        if let Some(solid_id) = env.spatial_hash.get(position).any_solid() {

            let entity = env.ecs.post_action_entity(entity_id, action);

            if entity.contains_collider() {
                if env.ecs.contains_explode_on_collision(solid_id) && entity.contains_triggers_explosion() {
                    reactions.push(Reaction::new(ActionArgs::Explode(solid_id), 0));
                }
                if entity.current_realtime_velocity().is_some() {
                    reactions.push(Reaction::new(ActionArgs::RealtimeVelocityStop(entity_id), 0));
                }
                if entity.contains_destroy_on_collision() {
                    reactions.push(Reaction::new(ActionArgs::Destroy(entity_id), 0));
                }
                return RULE_REJECT;
            }
        }
    }

    RULE_ACCEPT
}
