use game::*;
use ecs_content::*;

use content_types::*;
use math::Direction;

const SPEED_CELLS_PER_SEC: f64 = 20.0;

pub fn physics(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    if !action.contains_property_physics() {
        return RULE_ACCEPT;
    }

    for (entity_id, direction) in env.ecs.copy_iter_steering() {
        match direction {
            SteerDirection::Up => {
                reactions.push(Reaction::new(ActionArgs::Walk(entity_id, Direction::North), 0));
            }
            SteerDirection::Down => {
                reactions.push(Reaction::new(ActionArgs::Walk(entity_id, Direction::South), 0));
            }
        }
        reactions.push(Reaction::new(ActionArgs::RemoveSteer(entity_id), 0));
    }

    for (entity_id, speed) in env.ecs.copy_iter_current_speed() {
        if speed != 0 {
            if let Some(facing) = env.ecs.get_copy_facing(entity_id) {
                let velocity = RealtimeVelocity::new(facing.vector(), SPEED_CELLS_PER_SEC);
                reactions.push(Reaction::new(ActionArgs::RealtimeVelocityStart(entity_id, velocity, speed), 0));
            }
        }
    }

    RULE_ACCEPT
}
