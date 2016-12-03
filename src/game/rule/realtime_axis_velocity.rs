use game::*;
use ecs::*;

pub struct RealtimeAxisVelocityStart;
impl Rule for RealtimeAxisVelocityStart {
    fn check(&self, _env: RuleEnv, action: &EcsAction, resolution: &mut RuleResolution) -> Result<()> {

        for (entity_id, velocity) in action.insertions.realtime_axis_velocity_iter() {
            let delay = velocity.speed.ms_per_cell();
            resolution.add_reaction(Reaction::new(ActionArgs::RealtimeAxisVelocityMove(entity_id, *velocity), delay));
        }

        Ok(())
    }
}

pub struct RealtimeAxisVelocity;

impl Rule for RealtimeAxisVelocity {
    fn check(&self, env: RuleEnv, action: &EcsAction, resolution: &mut RuleResolution) -> Result<()> {

        for (entity_id, _position) in action.insertions.position_iter() {
            let entity = env.ecs.entity(entity_id);
            if let Some(velocity) = entity.realtime_axis_velocity() {
                resolution.add_reaction(Reaction::new(ActionArgs::RealtimeAxisVelocityMove(entity_id, *velocity), 0));
            }
        }

        Ok(())
    }
}
