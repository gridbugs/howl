use game::*;
use behaviour::LeafResolution;
use search::{GridSearchCfg, GridSearchCtx};

pub fn follow_path_step<K: KnowledgeRenderer>() -> BehaviourLeaf<K> {
    BehaviourLeaf::new(move |input| {
        let mut path_traverse = input.entity.path_traverse_borrow_mut().unwrap();
        let action = if let Some(direction) = path_traverse.next_direction() {
            MetaAction::ActionArgs(ActionArgs::Walk(input.entity.id(), direction))
        } else {
            MetaAction::ActionArgs(ActionArgs::Null)
        };
        LeafResolution::Yield(action)
    })
}

pub fn simple_npc_update_path<K: KnowledgeRenderer>() -> BehaviourLeaf<K> {
    let search_ctx = GridSearchCtx::new();
    let search_cfg = GridSearchCfg::cardinal_directions();

    BehaviourLeaf::new(move |input| {
        let position = input.entity.position().unwrap();
        let knowledge = input.entity.simple_npc_knowledge_borrow().unwrap();
        let level_knowledge = knowledge.level(input.level_id);
        let mut path_traverse = input.entity.path_traverse_borrow_mut().unwrap();

        let result = search_ctx.search_predicate(
            level_knowledge.grid(), position,
            |info| level_knowledge.contains_target(info.coord),
            &search_cfg, path_traverse.path_mut());

        if result.is_err() {
            return LeafResolution::Return(false);
        }

        path_traverse.reset();
        LeafResolution::Return(true)
    })
}
