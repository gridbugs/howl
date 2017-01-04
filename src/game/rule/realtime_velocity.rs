use game::*;
use ecs::*;

pub struct RealtimeVelocityStart;
impl Rule for RealtimeVelocityStart {
    fn check(&self, env: RuleEnv, action: &EcsAction, resolution: &mut RuleResolution) -> Result<()> {

        for (entity_id, velocity) in action.realtime_velocity_positive_iter(env.ecs) {
            if env.ecs.realtime_velocity(entity_id).is_none() {
                let delay = velocity.ms_per_cell();
                resolution.add_reaction(Reaction::new(ActionArgs::RealtimeVelocityMove(entity_id, *velocity), delay));
            }
        }

        Ok(())
    }
}

pub struct RealtimeVelocity;

impl Rule for RealtimeVelocity {
    fn check(&self, env: RuleEnv, action: &EcsAction, resolution: &mut RuleResolution) -> Result<()> {

        for (entity_id, _position) in action.position().insertion_copy_iter() {
            let entity = env.ecs.entity(entity_id);
            if let Some(velocity) = entity.realtime_velocity() {
                resolution.add_reaction(Reaction::new(ActionArgs::RealtimeVelocityMove(entity_id, *velocity), 0));
            }
        }

        Ok(())
    }
}
