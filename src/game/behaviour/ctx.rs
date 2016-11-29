use game::{BehaviourGraph, BehaviourNodeIndex, BehaviourType};
use game::behaviour::player_input::*;
use frontends::ansi;
use behaviour::*;

pub struct BehaviourNodes {
    pub ansi_player_input: BehaviourNodeIndex,
}

pub struct BehaviourCtx {
    pub graph: BehaviourGraph,
    pub nodes: BehaviourNodes,
}

impl BehaviourNodes {
    pub fn index(&self, behaviour_type: BehaviourType) -> BehaviourNodeIndex {
        match behaviour_type {
            BehaviourType::AnsiPlayerInput => self.ansi_player_input,
        }
    }
}

impl BehaviourCtx {
    pub fn new(input_source: ansi::InputSource) -> Self {
        let mut graph = BehaviourGraph::new();

        let ansi_player_input_leaf = graph.add_leaf(ansi_player_input(input_source));

        let nodes = BehaviourNodes {
            ansi_player_input: graph.add_collection(CollectionNode::Forever(ansi_player_input_leaf)),
        };

        BehaviourCtx {
            graph: graph,
            nodes: nodes,
        }
    }

    pub fn graph(&self) -> &BehaviourGraph {
        &self.graph
    }

    pub fn nodes(&self) -> &BehaviourNodes {
        &self.nodes
    }
}
