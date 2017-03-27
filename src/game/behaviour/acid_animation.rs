use game::*;
use behaviour::LeafResolution;
use content_types::ActionArgs;

pub fn acid_animate<K: KnowledgeRenderer>() -> BehaviourLeaf<K> {
    BehaviourLeaf::new(move |_| {
        LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::AcidAnimate))
    })
}
