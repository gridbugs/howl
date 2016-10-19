use game::{IdEntityRef, MetaAction, actions};

use behaviour;
use geometry::Direction;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Behaviour {
    BackAndForthForever,
}

pub struct BehaviourContext<E> {
    pub graph: behaviour::Graph<E, MetaAction>,
    behaviours: HashMap<Behaviour, behaviour::NodeIndex>,
}

impl<'a, E: 'a + IdEntityRef<'a>> BehaviourContext<E> {
    pub fn new() -> Self {
        let mut graph = behaviour::Graph::new();
        let mut behaviours = HashMap::new();

        let east = graph.add_leaf(Box::new(|e: E| {
            let walk = MetaAction::Update(actions::walk(e, Direction::East));
            behaviour::LeafResolution::Yield(walk)
        }));
        let west = graph.add_leaf(Box::new(|e: E| {
            let walk = MetaAction::Update(actions::walk(e, Direction::West));
            behaviour::LeafResolution::Yield(walk)
        }));
        let back_and_forth = graph.add_collection(behaviour::CollectionNode::All(vec![east, west]));
        let back_and_forth_forever =
            graph.add_collection(behaviour::CollectionNode::Forever(back_and_forth));

        behaviours.insert(Behaviour::BackAndForthForever, back_and_forth_forever);

        BehaviourContext {
            graph: graph,
            behaviours: behaviours,
        }
    }

    pub fn get_node_index(&self, behaviour: Behaviour) -> behaviour::NodeIndex {
        *self.behaviours.get(&behaviour).expect("missing behaviour")
    }
}
