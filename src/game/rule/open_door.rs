use game::*;
use ecs::*;

pub fn open_door(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, position) in action.position().insertion_copy_iter() {

        if let Some(door_id) = env.spatial_hash.get(position).any_door() {

            let door = env.ecs.entity(door_id);
            if door.door_state().ok_or(Error::MissingComponent)?.is_closed() {
                let entity = env.ecs.entity(entity_id);
                if entity.contains_door_opener() {
                    reactions.push(Reaction::new(ActionArgs::OpenDoor(door_id), 0));
                    return RULE_REJECT;
                }
            }
        }
    }

    RULE_ACCEPT
}
