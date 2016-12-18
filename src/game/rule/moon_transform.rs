use game::*;
use ecs::*;

pub struct MoonTransform;

impl Rule for MoonTransform {
    fn check(&self, env: RuleEnv, action: &EcsAction, resolution: &mut RuleResolution) -> Result<()> {

        for entity_id in action.moon().insertion_iter() {
            if let Some(position) = env.ecs.position(entity_id) {
                let cell = env.spatial_hash.get(position);
                for transformer_id in cell.transform_on_moon_change_iter() {
                    let transformer = env.ecs.entity(transformer_id);
                    let transformation_state = transformer.transformation_state().ok_or(Error::MissingComponent)?;
                    if transformation_state == TransformationState::Real {
                        let transformation_type = transformer.transformation_type().ok_or(Error::MissingComponent)?;
                        let action_args = transformation_type.to_action_args(transformer_id);
                        resolution.add_reaction(Reaction::new(action_args, 0));
                    }
                }
            }
        }

        for entity_id in action.moon().removal_iter() {
            if let Some(position) = env.ecs.position(entity_id) {
                let cell = env.spatial_hash.get(position);
                for transformer_id in cell.transform_on_moon_change_iter() {
                    let transformer = env.ecs.entity(transformer_id);
                    let transformation_state = transformer.transformation_state().ok_or(Error::MissingComponent)?;
                    if transformation_state == TransformationState::Other {
                        let transformation_type = transformer.transformation_type().ok_or(Error::MissingComponent)?;
                        let action_args = transformation_type.to_action_args(transformer_id);
                        resolution.add_reaction(Reaction::new(action_args, 0));
                    }
                }
            }
        }

        Ok(())
    }
}
