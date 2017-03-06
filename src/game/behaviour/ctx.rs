use game::*;
use game::behaviour::player_input::*;
use game::behaviour::observation::*;
use game::behaviour::search::*;
use game::behaviour::acid_animation::*;
use game::behaviour::physics::*;
use game::behaviour::axis::*;

use behaviour::{LeafResolution, CollectionNode};

pub struct BehaviourNodes {
    pub null: BehaviourNodeIndex,
    pub player_input: BehaviourNodeIndex,
    pub simple_npc: BehaviourNodeIndex,
    pub acid_animate: BehaviourNodeIndex,
    pub physics: BehaviourNodeIndex,
    pub car: BehaviourNodeIndex,
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
            BehaviourType::SimpleNpc => self.simple_npc,
            BehaviourType::AcidAnimate => self.acid_animate,
            BehaviourType::Physics => self.physics,
            BehaviourType::Car => self.car,
        }
    }
}

impl<K: KnowledgeRenderer> BehaviourCtx<K> {
    pub fn new<I: 'static + InputSource + Clone>(input_source: I) -> Self {
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

        let acid_animate_leaf = graph.add_leaf(acid_animate());
        let physics_leaf = graph.add_leaf(physics());

        let car_leaf = graph.add_leaf(car_chace_axis());
        let car_loop = graph.add_collection(CollectionNode::Forever(car_leaf));
        let car = graph.add_switch(simple_npc_shadowcast(car_loop));

        let nodes = BehaviourNodes {
            null: graph.add_collection(CollectionNode::Forever(null_leaf)),
            player_input: graph.add_collection(CollectionNode::Forever(player_input_leaf)),
            simple_npc: graph.add_collection(CollectionNode::Forever(simple_npc)),
            acid_animate: graph.add_collection(CollectionNode::Forever(acid_animate_leaf)),
            physics: graph.add_collection(CollectionNode::Forever(physics_leaf)),
            car: graph.add_collection(CollectionNode::Forever(car)),
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
