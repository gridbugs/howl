use game::*;
use ecs::*;

pub fn bounds(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {
    for (entity_id, position) in action.copy_iter_position() {
        if !env.spatial_hash.is_valid_signed_coord(position.x, position.y) {
            if env.ecs.contains_realtime_velocity(entity_id) {
                reactions.push(Reaction::new(ActionArgs::RealtimeVelocityStop(entity_id), 0));
            }
            if env.ecs.contains_destroy_on_collision(entity_id) ||
                env.ecs.contains_destroy_when_out_of_bounds(entity_id) {
                reactions.push(Reaction::new(ActionArgs::Destroy(entity_id), 0));
            }
            return RULE_REJECT;
        }
    }
    RULE_ACCEPT
}
