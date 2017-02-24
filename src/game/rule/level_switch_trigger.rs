use game::*;
use ecs::*;

pub fn level_switch_trigger(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    if let Some(entity_id) = action.try_level_switch() {
        // the character tried to switch levels

        let entity = env.ecs.post_action_entity(entity_id, action);

        if let Some(position) = entity.position() {

            // is there actually a level switch here
            if let Some(trigger_id) = env.spatial_hash.get(position).any_level_switch_trigger() {

                let level_switch = env.ecs.level_switch_trigger(trigger_id)
                    .expect("Entity missing level_switch_trigger");

                reactions.push(Reaction::new(ActionArgs::LevelSwitch(trigger_id, level_switch), 0));
            }
        }
    }

    RULE_ACCEPT
}

pub fn level_switch_trigger_auto(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, position) in action.position().insertion_copy_iter() {

        if !env.ecs.contains_pc(entity_id) {
            // only the player may switch levels
            continue;
        }

        if let Some(trigger_id) = env.spatial_hash.get(position).any_level_switch_trigger() {

            if env.ecs.contains_level_switch_auto(trigger_id) {

                let level_switch = env.ecs.level_switch_trigger(trigger_id)
                    .expect("Entity missing level_switch_trigger");

                reactions.push(Reaction::new(ActionArgs::LevelSwitch(trigger_id, level_switch), 0));
                break;
            }
        }
    }

    RULE_ACCEPT
}
