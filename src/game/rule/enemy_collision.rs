use game::*;
use ecs::*;

pub fn enemy_collision(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, position) in action.position_profile().insertion_copy_iter() {
        if !env.ecs.contains_enemy(entity_id) {
            continue;
        }

        if env.spatial_hash.get(position).enemy() {
            reactions.push(Reaction::new(ActionArgs::Null, 0));
            return RULE_REJECT;
        }

        if env.spatial_hash.get(position).pc() {
            // TODO damage the player here
            reactions.push(Reaction::new(ActionArgs::Null, 0));
            return RULE_REJECT;
        }
    }

    RULE_ACCEPT
}

pub fn pc_collision(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, position) in action.position_profile().insertion_copy_iter() {
        if !env.ecs.contains_pc(entity_id) {
            continue;
        }

        if env.spatial_hash.get(position).enemy() {

            if env.ecs.contains_realtime_velocity(entity_id) {
                reactions.push(Reaction::new(ActionArgs::RealtimeVelocityStop(entity_id), 0));
            }

            reactions.push(Reaction::new(ActionArgs::Null, 0));
            return RULE_REJECT;
        }
    }

    RULE_ACCEPT
}
