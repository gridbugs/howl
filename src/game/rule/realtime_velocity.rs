use game::*;
use ecs::*;

pub fn realtime_velocity_start(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, velocity) in action.realtime_velocity_positive_iter(env.ecs) {
        if env.ecs.realtime_velocity(entity_id).is_none() {
            let delay = velocity.ms_per_cell();
            reactions.push(Reaction::new(ActionArgs::RealtimeVelocityMove(entity_id, *velocity), delay));
        }
    }

    RULE_ACCEPT
}

pub fn realtime_velocity(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, _position) in action.position_profile().insertion_copy_iter() {
        let entity = env.ecs.entity(entity_id);
        if let Some(velocity) = entity.realtime_velocity() {
            reactions.push(Reaction::new(ActionArgs::RealtimeVelocityMove(entity_id, *velocity), 0));
        }
    }

    RULE_ACCEPT
}
