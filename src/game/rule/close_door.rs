use game::*;
use ecs::*;

pub struct CloseDoor;

impl Rule for CloseDoor {
    fn check(&self, env: RuleEnv, action: &EcsAction, resolution: &mut RuleResolution) -> Result<()> {

        if let Some(close) = action.close() {
            let entity = env.ecs.entity(close.entity_id);
            let position = entity.position().ok_or(Error::MissingComponent)?;
            let target_position = position + close.direction.vector();

            if let Some(door_id) = env.spatial_hash.get(target_position).any_door() {
                resolution.add_reaction(Reaction::new(ActionArgs::CloseDoor(door_id), 0));
            }
        }

        Ok(())
    }
}
