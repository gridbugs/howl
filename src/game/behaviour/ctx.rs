use game::*;
use game::behaviour::player_input::*;
use game::behaviour::observation::*;

use engine_defs::*;
use control::*;
use content_types::BehaviourType;
use behaviour::{LeafResolution, CollectionNode};
use content_types::ActionArgs;

pub struct BehaviourNodes {
    pub null: BehaviourNodeIndex,
    pub player_input: BehaviourNodeIndex,
}

pub struct BehaviourCtx<K: KnowledgeRenderer> {
    pub graph: BehaviourGraph<K>,
    pub nodes: BehaviourNodes,
}

impl BehaviourNodes {
    pub fn index(&self, behaviour_type: BehaviourType) -> BehaviourNodeIndex {
        match behaviour_type {
            BehaviourType::Null => self.null,
            BehaviourType::PlayerInput => self.player_input,
        }
    }
}

impl<K: KnowledgeRenderer> BehaviourCtx<K> {
    pub fn new<I: 'static + InputSource + Clone>(input_source: I) -> Self {
        let mut graph = BehaviourGraph::new();

        let null_leaf = graph.add_leaf(BehaviourLeaf::new(|_| LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Null))));

        let player_input_leaf = graph.add_leaf(player_input(input_source));

        let nodes = BehaviourNodes {
            null: graph.add_collection(CollectionNode::Forever(null_leaf)),
            player_input: graph.add_collection(CollectionNode::Forever(player_input_leaf)),
        };

        BehaviourCtx {
            graph: graph,
            nodes: nodes,
        }
    }

    pub fn graph(&self) -> &BehaviourGraph<K> {
        &self.graph
    }

    pub fn nodes(&self) -> &BehaviourNodes {
        &self.nodes
    }
}
