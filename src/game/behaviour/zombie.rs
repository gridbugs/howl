use game::*;
use ecs_content::Entity;

use behaviour::LeafResolution;
use math::Direction;

pub fn zombie_step<K: KnowledgeRenderer>() -> BehaviourLeaf<K> {
    BehaviourLeaf::new(move |input| {
        let position = input.entity.copy_position().unwrap();
        let knowledge = input.entity.borrow_simple_npc_knowledge().unwrap();
        let level_knowledge = knowledge.level(input.level_id);

        let action = if let Some(target) = level_knowledge.any_target() {
            if position == target {
                ActionArgs::Null
            } else {
                let delta = target - position;
                if delta.x.abs() > delta.y.abs() {
                    if delta.x > 0 {
                        ActionArgs::Walk(input.entity.id(), Direction::East)
                    } else {
                        ActionArgs::Walk(input.entity.id(), Direction::West)
                    }
                } else {
                    if delta.y > 0 {
                        ActionArgs::Walk(input.entity.id(), Direction::South)
                    } else {
                        ActionArgs::Walk(input.entity.id(), Direction::North)
                    }
                }
            }
        } else {
            ActionArgs::Null
        };

        LeafResolution::Yield(MetaAction::ActionArgs(action))
    })
}
