use game::*;
use ecs::*;

pub fn death(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, hit_points) in action.hit_points_profile().insertion_copy_iter() {
        let current_hit_points = env.ecs.hit_points(entity_id).expect("Missing hit_points component");
        if !hit_points.is_positive() && current_hit_points.is_positive() {
            if env.ecs.contains_bloodstain_on_death(entity_id) {
                reactions.push(Reaction::new(ActionArgs::BecomeBloodstain(entity_id), 0));
            } else {
                reactions.push(Reaction::new(ActionArgs::Die(entity_id), 0));
            }
        }
    }

    RULE_ACCEPT
}
