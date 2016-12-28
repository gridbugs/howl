use game::*;
use behaviour::*;

pub fn simple_npc_shadowcast(child: NodeIndex) -> BehaviourSwitch {

    let shadowcast = Shadowcast::new();

    BehaviourSwitch::new_returning(move |input| {

        let eye = input.entity.position().unwrap();
        let vision_distance = input.entity.vision_distance().unwrap();
        let mut knowledge = input.entity.simple_npc_knowledge_borrow_mut().unwrap();
        let level_knowledge = knowledge.level_mut_or_insert_size(input.level_id,
                                                                 input.spatial_hash.width(),
                                                                 input.spatial_hash.height());

        shadowcast.observe(eye, input.spatial_hash, vision_distance,
                           level_knowledge, input.action_env);

        if level_knowledge.last_target_update() == input.action_env.id {
            // the targets have changed
            SwitchResolution::Reset(child)
        } else {
            SwitchResolution::Select(child)
        }
    })
}
