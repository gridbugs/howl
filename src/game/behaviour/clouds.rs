use game::*;
use behaviour::LeafResolution;

pub fn move_clouds<K: KnowledgeRenderer>() -> BehaviourLeaf<K> {
    BehaviourLeaf::new(move |input| {
        LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::MoveClouds(input.entity.id())))
    })
}
