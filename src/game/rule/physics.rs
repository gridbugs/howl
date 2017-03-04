use game::*;
use ecs::*;

use game::data::*;

const SPEED_CELLS_PER_SEC: f64 = 20.0;

pub fn physics(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    if !action.contains_physics() {
        return RULE_ACCEPT;
    }

    for (entity_id, speed) in env.ecs.current_speed_iter() {
        if speed != 0 {
            if let Some(facing) = env.ecs.facing(entity_id) {
                let velocity = RealtimeVelocity::new(facing.vector(), SPEED_CELLS_PER_SEC);
                reactions.push(Reaction::new(ActionArgs::RealtimeVelocityStart(entity_id, velocity, speed), 0));
            }
        }
    }

    RULE_ACCEPT
}
