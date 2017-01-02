use game::*;
use behaviour::LeafResolution;

pub fn move_clouds() -> BehaviourLeaf {
    BehaviourLeaf::new(move |input| {
        LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::MoveClouds(input.entity.id())))
    })
}
