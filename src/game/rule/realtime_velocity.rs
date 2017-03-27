use game::*;
use ecs_content::*;
use content_types::{ActionArgs, Reaction};

pub fn realtime_velocity_start(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    if !action.contains_property_start_realtime_move() {
        return RULE_ACCEPT;
    }

    for (entity_id, velocity) in action.positive_iter_realtime_velocity(env.ecs) {
        let delay = velocity.ms_per_cell();
        reactions.push(Reaction::new(ActionArgs::RealtimeVelocityMove(entity_id, *velocity), delay));
    }

    RULE_ACCEPT
}

pub fn realtime_velocity(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for entity_id in action.id_iter_position() {
        let entity = env.ecs.post_entity(action, entity_id);
        if let Some(0) = entity.copy_realtime_moves_remaining() {
            if entity.contains_destroy_when_stopped() {
                reactions.push(Reaction::new(ActionArgs::Destroy(entity_id), 0));
            } else {
                reactions.push(Reaction::new(ActionArgs::RealtimeVelocityStop(entity_id), 0));
            }
            continue;
        }
        if let Some(velocity) = entity.realtime_velocity() {
            reactions.push(Reaction::new(ActionArgs::RealtimeVelocityMove(entity_id, *velocity), 0));
        }
    }

    RULE_ACCEPT
}
