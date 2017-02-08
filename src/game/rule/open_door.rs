use game::*;
use ecs::*;

pub fn open_door(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, position) in action.position().insertion_copy_iter() {

        if let Some(door_id) = env.spatial_hash.get(position).any_door() {

            let door = env.ecs.entity(door_id);
            let door_state = door.door_state().expect("Entity missing door_state");
            if door_state.is_closed() {
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
