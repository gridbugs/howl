use game::*;
use ecs::*;

pub fn level_switch(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    if let Some(entity_id) = action.try_level_switch() {
        // the character tried to switch levels

        let entity = env.ecs.post_action_entity(entity_id, action);

        if let Some(position) = entity.position() {

            // is there actually a level switch here
            if let Some(exit_id) = env.spatial_hash.get(position).any_level_switch() {

                let level_switch = env.ecs.level_switch(exit_id)
                    .expect("Entity missing level_switch");

                reactions.push(Reaction::new(ActionArgs::LevelSwitch {
                    entity_id: entity_id,
                    exit_id: exit_id,
                    level_switch: level_switch
                }, 0));
            }
        }
    }

    RULE_ACCEPT
}

pub fn level_switch_auto(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, position) in action.position_profile().insertion_copy_iter() {

        if !env.ecs.contains_pc(entity_id) {
            // only the player may switch levels automatically
            continue;
        }

        if let Some(exit_id) = env.spatial_hash.get(position).any_level_switch() {

            if env.ecs.contains_level_switch_auto(exit_id) {

                let level_switch = env.ecs.level_switch(exit_id)
                    .expect("Entity missing level_switch");

                reactions.push(Reaction::new(ActionArgs::LevelSwitch {
                    entity_id: entity_id,
                    exit_id: exit_id,
                    level_switch: level_switch
                }, 0));
                break;
            }
        }
    }

    RULE_ACCEPT
}
