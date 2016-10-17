use game::{UpdateSummary, IdEntityRef, actions};

use behaviour;
use geometry::Direction;

pub struct Behaviours {
    pub back_and_forth: behaviour::NodeIndex,
    pub back_and_forth_forever: behaviour::NodeIndex,
}

pub struct BehaviourContext<E> {
    pub graph: behaviour::Graph<E, UpdateSummary>,
    pub behaviours: Behaviours,
}

impl<'a, E: 'a + IdEntityRef<'a>> BehaviourContext<E> {
    pub fn new() -> Self {
        let mut graph = behaviour::Graph::new();

        let east = graph.add_leaf(Box::new(|e: E| {
            let walk = actions::walk(e, Direction::East);
            behaviour::LeafResolution::Yield(walk)
        }));
        let west = graph.add_leaf(Box::new(|e: E| {
            let walk = actions::walk(e, Direction::West);
            behaviour::LeafResolution::Yield(walk)
        }));
        let back_and_forth = graph.add_collection(
            behaviour::CollectionNode::All(vec![east, west]));
        let back_and_forth_forever = graph.add_collection(
            behaviour::CollectionNode::Forever(back_and_forth));

        let behaviours = Behaviours {
            back_and_forth: back_and_forth,
            back_and_forth_forever: back_and_forth_forever,
        };

        BehaviourContext {
            graph: graph,
            behaviours: behaviours,
        }
    }
}
