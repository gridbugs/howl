use game::*;
use game::behaviour::player_input::*;
use game::behaviour::observation::*;
use game::behaviour::physics::*;
use game::behaviour::car::*;
use game::behaviour::bike::*;
use game::behaviour::zombie::*;

use engine_defs::*;
use control::*;
use content_types::BehaviourType;
use behaviour::{LeafResolution, CollectionNode};
use content_types::ActionArgs;

pub struct BehaviourNodes {
    pub null: BehaviourNodeIndex,
    pub player_input: BehaviourNodeIndex,
    pub zombie: BehaviourNodeIndex,
    pub physics: BehaviourNodeIndex,
    pub car: BehaviourNodeIndex,
    pub bike: BehaviourNodeIndex,
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
            BehaviourType::Zombie => self.zombie,
            BehaviourType::Physics => self.physics,
            BehaviourType::Car => self.car,
            BehaviourType::Bike => self.bike,
        }
    }
}

impl<K: KnowledgeRenderer> BehaviourCtx<K> {
    pub fn new<I: 'static + InputSource + Clone>(input_source: I) -> Self {
        let mut graph = BehaviourGraph::new();

        let null_leaf = graph.add_leaf(BehaviourLeaf::new(|_| LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Null))));

        let player_input_leaf = graph.add_leaf(player_input(input_source));

        let zombie_leaf = graph.add_leaf(zombie_step());
        let zombie_loop = graph.add_collection(CollectionNode::Forever(zombie_leaf));
        let zombie = graph.add_switch(simple_npc_shadowcast(zombie_loop));

        let physics_leaf = graph.add_leaf(physics());

        let car_leaf = graph.add_leaf(car_chace());
        let car_loop = graph.add_collection(CollectionNode::Forever(car_leaf));
        let car = graph.add_switch(simple_npc_shadowcast(car_loop));

        let bike_leaf = graph.add_leaf(bike_chace());
        let bike_loop = graph.add_collection(CollectionNode::Forever(bike_leaf));
        let bike = graph.add_switch(simple_npc_shadowcast(bike_loop));

        let nodes = BehaviourNodes {
            null: graph.add_collection(CollectionNode::Forever(null_leaf)),
            player_input: graph.add_collection(CollectionNode::Forever(player_input_leaf)),
            zombie: graph.add_collection(CollectionNode::Forever(zombie)),
            physics: graph.add_collection(CollectionNode::Forever(physics_leaf)),
            car: graph.add_collection(CollectionNode::Forever(car)),
            bike: graph.add_collection(CollectionNode::Forever(bike)),
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
