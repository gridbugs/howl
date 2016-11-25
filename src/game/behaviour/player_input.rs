use game::{BehaviourLeaf, ActionArgs};
use behaviour::LeafResolution;
use direction::Direction;
use frontends::ansi;

pub fn ansi_player_input(input_source: ansi::InputSource) -> BehaviourLeaf {
    BehaviourLeaf::new(move |input| {
        input_source.get_event(); // TODO
        LeafResolution::Yield(ActionArgs::Walk(input.entity.id(), Direction::North))
    })
}
