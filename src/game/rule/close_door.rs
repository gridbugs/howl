use game::*;
use ecs::*;

pub fn close_door(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    if let Some(close) = action.close() {
        let entity = env.ecs.entity(close.entity_id);
        let position = entity.position().expect("Entity missing position");
        let target_position = position + close.direction.vector();

        if let Some(door_id) = env.spatial_hash.get(target_position).any_door() {
            reactions.push(Reaction::new(ActionArgs::CloseDoor(door_id), 0));
        }
    }

    RULE_ACCEPT
}
