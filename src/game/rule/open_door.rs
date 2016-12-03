use game::*;
use ecs::*;

pub struct OpenDoor;

impl Rule for OpenDoor {
    fn check(&self, env: RuleEnv, action: &EcsAction, resolution: &mut RuleResolution) -> Result<()> {

        for (entity_id, position) in action.insertions.position_iter() {

            if let Some(door_id) = env.spatial_hash.get(position).any_door() {

                let door = env.ecs.entity(door_id);
                if door.door_state().ok_or(Error::MissingComponent)?.is_closed() {
                    let entity = env.ecs.entity(entity_id);
                    if entity.contains_door_opener() {
                        resolution.reject();
                        resolution.add_reaction(Reaction::new(ActionArgs::OpenDoor(door_id), 0));
                    }
                }
            }
        }

        Ok(())
    }
}
