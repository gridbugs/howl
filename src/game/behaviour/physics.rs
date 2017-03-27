use game::*;
use behaviour::LeafResolution;
use content_types::ActionArgs;

pub fn physics<K: KnowledgeRenderer>() -> BehaviourLeaf<K> {
    BehaviourLeaf::new(move |_| {
        LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Physics))
    })
}
