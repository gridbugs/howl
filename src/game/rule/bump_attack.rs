use game::*;
use ecs::*;

pub fn bump_attack(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (attacker_id, position) in action.position_profile().insertion_copy_iter() {

        if let Some(victim_id) = env.spatial_hash.get(position).any_bump_attackable() {

            let victim = env.ecs.entity(victim_id);
            let attacker = env.ecs.entity(attacker_id);

            if let Some(damage) = attacker.bump_attacker() {
                if victim.contains_hit_points() {
                    reactions.push(Reaction::new(ActionArgs::Damage(victim_id, damage), 0));
                }
                return RULE_REJECT;
            }
        }
    }

    RULE_ACCEPT
}
