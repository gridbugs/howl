use game::*;
use game::data::*;
use behaviour::LeafResolution;

pub fn car_chace_axis<K: KnowledgeRenderer>() -> BehaviourLeaf<K> {
    BehaviourLeaf::new(move |input| {
        LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::ChangeSpeed(input.entity.id(), ChangeSpeed::Accelerate)))
    })
}
