use game::*;
use ecs::*;

pub fn enemy_collision(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, position) in action.position_profile().insertion_copy_iter() {
        if !env.ecs.contains_enemy(entity_id) {
            continue;
        }

        if let Some(enemy_id) = env.spatial_hash.get(position).any_enemy() {
            if env.ecs.contains_can_run_over(entity_id) && env.ecs.contains_can_be_run_over(enemy_id) {
                continue;
            }

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

        if let Some(enemy_id) = env.spatial_hash.get(position).any_enemy() {

            if env.ecs.contains_can_run_over(entity_id) && env.ecs.contains_can_be_run_over(enemy_id) {
                continue;
            }

            if env.ecs.contains_realtime_velocity(entity_id) {
                reactions.push(Reaction::new(ActionArgs::RealtimeVelocityStop(entity_id), 0));
            }

            reactions.push(Reaction::new(ActionArgs::Null, 0));
            return RULE_REJECT;
        }
    }

    RULE_ACCEPT
}
