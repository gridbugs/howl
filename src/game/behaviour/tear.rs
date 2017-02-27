use game::*;
use behaviour::LeafResolution;

pub fn move_tear<K: KnowledgeRenderer>() -> BehaviourLeaf<K> {
    BehaviourLeaf::new(move |input| {
        LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::MoveTear(input.entity.id())))
    })
}
