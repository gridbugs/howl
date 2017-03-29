use game::*;
use ecs_content::Entity;
use engine_defs::*;

use behaviour::SwitchResolution;

pub fn simple_npc_shadowcast<K: KnowledgeRenderer>(child: BehaviourNodeIndex) -> BehaviourSwitch<K> {

    let shadowcast = Shadowcast::new();

    BehaviourSwitch::new_returning(move |input| {

        let eye = input.entity.copy_position().unwrap();
        let vision_distance = input.entity.copy_vision_distance().unwrap();
        let mut knowledge = input.entity.borrow_mut_simple_npc_knowledge().unwrap();
        let level_knowledge = knowledge.level_mut_or_insert_size(input.level_id,
                                                                 input.spatial_hash.width(),
                                                                 input.spatial_hash.height());

        shadowcast.observe(eye, input.spatial_hash, vision_distance,
                           level_knowledge, ActionEnv::new(input.entity.ecs(), input.action_id));

        if level_knowledge.last_target_update() == input.action_id {
            // the targets have changed
            SwitchResolution::Reset(child)
        } else {
            SwitchResolution::Select(child)
        }
    })
}
