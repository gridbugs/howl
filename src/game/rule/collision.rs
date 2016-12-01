use game::*;
use ecs::*;

pub struct Collision;

impl Rule for Collision {
    fn check(&self, env: RuleEnv, action: &EcsAction, resolution: &mut RuleResolution) -> Result<()> {

        for (entity_id, position) in action.insertions.position_iter() {

            if !env.spatial_hash.get(position).solid() {
                continue;
            }

            let entity = env.ecs.entity(entity_id);

            if entity.contains_collider() {
                resolution.reject();
            }
        }

        Ok(())
    }
}
