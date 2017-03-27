use game::*;
use ecs_content::*;

pub fn bump_attack(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (attacker_id, position) in action.copy_iter_position() {

        if let Some(victim_id) = env.spatial_hash.get(position).any_bump_attackable() {

            let victim = env.ecs.entity(victim_id);
            let attacker = env.ecs.entity(attacker_id);

            if attacker.contains_can_run_over() && victim.contains_can_be_run_over() {
                // they are run over instead
                continue;
            }

            if let Some(damage) = attacker.copy_bump_attacker() {
                if victim.contains_complex_damage() {
                    reactions.push(Reaction::new(ActionArgs::Bump(victim_id, attacker_id), 0));
                    reactions.push(Reaction::new(ActionArgs::ComplexDamage(victim_id, damage), 0));
                } else if victim.contains_hit_points() {
                    reactions.push(Reaction::new(ActionArgs::Damage(victim_id, damage), 0));
                }
                if attacker.realtime_velocity().is_some() {
                    reactions.push(Reaction::new(ActionArgs::RealtimeVelocityStop(attacker_id), 0));
                }
                return RULE_REJECT;
            }
        }
    }

    RULE_ACCEPT
}
