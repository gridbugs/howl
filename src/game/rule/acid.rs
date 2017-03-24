use game::*;
use ecs::*;

pub fn acid(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, position) in action.copy_iter_position() {

        if env.spatial_hash.get(position).is_acid() {

            let entity = env.ecs.entity(entity_id);

            if entity.contains_tyre_health() {
                reactions.push(Reaction::new(ActionArgs::AcidDamage(entity_id), 0));
            }
        }
    }

    RULE_ACCEPT
}
