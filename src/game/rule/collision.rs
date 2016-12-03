use game::*;
use ecs::*;

pub struct Collision;

impl Rule for Collision {
    fn check(&self, env: RuleEnv, action: &EcsAction, resolution: &mut RuleResolution) -> Result<()> {

        for (entity_id, position) in action.insertions.position_iter() {

            if !env.spatial_hash.get(position).solid() {
                continue;
            }

            let entity = env.ecs.post_insertion_entity(entity_id, &action.insertions);

            if entity.contains_collider() {
                resolution.reject();
            }
            if entity.contains_destroy_on_collision() {
                resolution.reject();
                resolution.add_reaction(Reaction::new(ActionArgs::Destroy(entity_id), 0));
            }
        }

        Ok(())
    }
}
