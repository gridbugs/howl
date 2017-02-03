use game::*;
use ecs::*;

pub fn death(_env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, hit_points) in action.hit_points().insertion_copy_iter() {
        if !hit_points.is_positive() {
            reactions.push(Reaction::new(ActionArgs::Die(entity_id), 0));
        }
    }

    RULE_ACCEPT
}
