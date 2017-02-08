use game::*;
use ecs::*;

pub fn level_switch_trigger(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, position) in action.position().insertion_copy_iter() {

        if !env.ecs.contains_pc(entity_id) {
            // only the player may switch levels
            continue;
        }

        if let Some(trigger_id) = env.spatial_hash.get(position).any_level_switch_trigger() {

            let level_switch = env.ecs.level_switch_trigger(trigger_id)
                .expect("Entity missing level_switch_trigger");

            reactions.push(Reaction::new(ActionArgs::LevelSwitch(level_switch), 0));
            break;
        }
    }

    RULE_ACCEPT
}
