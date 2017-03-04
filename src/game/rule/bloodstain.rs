use game::*;
use ecs::*;

pub fn run_over(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (attacker_id, position) in action.position_profile().insertion_copy_iter() {

        if let Some(victim_id) = env.spatial_hash.get(position).any_enemy() {

            let victim = env.ecs.entity(victim_id);
            let attacker = env.ecs.entity(attacker_id);

            if attacker.contains_can_run_over() && victim.contains_can_be_run_over() {
                reactions.push(Reaction::new(ActionArgs::Die(victim_id), 0));
            }
        }
    }

    RULE_ACCEPT
}
