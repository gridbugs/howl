use game::*;
use game::behaviour::player_input::*;
use game::behaviour::observation::*;
use game::behaviour::search::*;
use game::behaviour::clouds::*;

use behaviour::{LeafResolution, CollectionNode};

pub struct BehaviourNodes {
    pub null: BehaviourNodeIndex,
    pub player_input: BehaviourNodeIndex,
    pub simple_npc: BehaviourNodeIndex,
    pub clouds: BehaviourNodeIndex,
}

pub struct BehaviourCtx {
    pub graph: BehaviourGraph,
    pub nodes: BehaviourNodes,
}

impl BehaviourNodes {
    pub fn index(&self, behaviour_type: BehaviourType) -> BehaviourNodeIndex {
        match behaviour_type {
            BehaviourType::Null => self.null,
            BehaviourType::PlayerInput => self.player_input,
            BehaviourType::SimpleNpc => self.simple_npc,
            BehaviourType::Clouds => self.clouds,
        }
    }
}

impl BehaviourCtx {
    pub fn new(input_source: InputSourceRef) -> Self {
        let mut graph = BehaviourGraph::new();

        let null_leaf = graph.add_leaf(BehaviourLeaf::new(|_| LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Null))));

        let player_input_leaf = graph.add_leaf(player_input(input_source));

        let simple_npc_update_path_leaf = graph.add_leaf(simple_npc_update_path());
        let follow_path_step_leaf = graph.add_leaf(follow_path_step());
        let follow_path_loop = graph.add_collection(CollectionNode::Forever(follow_path_step_leaf));
        let simple_npc_loop = graph.add_collection(CollectionNode::All(vec![
                                                                       simple_npc_update_path_leaf,
                                                                       follow_path_loop]));

        let simple_npc = graph.add_switch(simple_npc_shadowcast(simple_npc_loop));

        let move_clouds = graph.add_leaf(move_clouds());

        let nodes = BehaviourNodes {
            null: graph.add_collection(CollectionNode::Forever(null_leaf)),
            player_input: graph.add_collection(CollectionNode::Forever(player_input_leaf)),
            simple_npc: graph.add_collection(CollectionNode::Forever(simple_npc)),
            clouds: graph.add_collection(CollectionNode::Forever(move_clouds)),
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
