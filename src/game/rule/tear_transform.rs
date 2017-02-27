use game::*;
use ecs::*;

pub fn tear_move_transform(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, new_position) in action.position_profile().insertion_copy_iter() {

        let entity = env.ecs.post_action_entity(entity_id, action);

        if let Some(transformation_state) = entity.transformation_state() {

            let transformation_type = entity.transformation_type().expect("Expected transformation_type component");

            if env.spatial_hash.get(new_position).tear() {
                if transformation_state == TransformationState::Real {
                    reactions.push(Reaction::new(transformation_type.to_action_args(entity_id), 0));
                }
            } else {
                if transformation_state == TransformationState::Other {
                    reactions.push(Reaction::new(transformation_type.to_action_args(entity_id), 0));
                }
            }
        }
    }

    RULE_ACCEPT
}

pub fn tear_transform(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for entity_id in action.tear_profile().insertion_iter() {
        if let Some(position) = env.ecs.position(entity_id) {
            let cell = env.spatial_hash.get(position);
            for transformer_id in cell.transformation_state_iter() {
                let transformer = env.ecs.entity(transformer_id);
                let transformation_state = transformer.transformation_state()
                    .expect("Entity missing transformation_state");

                if transformation_state == TransformationState::Real {
                    let transformation_type = transformer.transformation_type()
                        .expect("Entity missing transformation_type");
                    let action_args = transformation_type.to_action_args(transformer_id);
                    reactions.push(Reaction::new(action_args, 0));
                }
            }
        }
    }

    for entity_id in action.tear_profile().removal_iter() {
        if let Some(position) = env.ecs.position(entity_id) {
            let cell = env.spatial_hash.get(position);
            for transformer_id in cell.transformation_state_iter() {
                let transformer = env.ecs.entity(transformer_id);
                let transformation_state = transformer.transformation_state()
                    .expect("Entity missing transformation_state");

                if transformation_state == TransformationState::Other {
                    let transformation_type = transformer.transformation_type()
                        .expect("Entity missing transformation_type");
                    let action_args = transformation_type.to_action_args(transformer_id);
                    reactions.push(Reaction::new(action_args, 0));
                }
            }
        }
    }

    RULE_ACCEPT
}
