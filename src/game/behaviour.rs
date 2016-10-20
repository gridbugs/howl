use game::{MetaAction, actions, Level, EntityId, ReserveEntityId, EntityStore, LevelEntityRef,
           EntityWrapper};
use game::io::terminal_player_actor;

use behaviour;
use geometry::Direction;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Behaviour {
    BackAndForthForever,
    PlayerInput,
}

#[derive(Clone, Copy)]
pub struct BehaviourInput<'a> {
    pub id: EntityId,
    ids: &'a ReserveEntityId,
    level: &'a Level,
    entity: LevelEntityRef<'a>,
}

impl<'a> BehaviourInput<'a> {
    pub fn new(id: EntityId, ids: &'a ReserveEntityId, level: &'a Level) -> Self {
        BehaviourInput {
            id: id,
            ids: ids,
            level: level,
            entity: level.get(id).unwrap(),
        }
    }
}

pub struct Leaf(Box<for<'a> Fn(BehaviourInput<'a>) -> behaviour::LeafResolution<MetaAction>>);
pub struct Check(Box<for<'a> Fn(BehaviourInput<'a>) -> Option<behaviour::CheckResolution>>);

impl<'a> behaviour::LeafFnBox<BehaviourInput<'a>, MetaAction> for Leaf {
    fn call(&self, knowledge: BehaviourInput<'a>) -> behaviour::LeafResolution<MetaAction> {
        (self.0)(knowledge)
    }
}

impl<'a> behaviour::CheckFnBox<BehaviourInput<'a>> for Check {
    fn call(&self, knowledge: BehaviourInput<'a>) -> Option<behaviour::CheckResolution> {
        (self.0)(knowledge)
    }
}

pub type BehaviourGraph = behaviour::Graph<Leaf, Check>;

pub struct BehaviourContext {
    pub graph: BehaviourGraph,
    behaviours: HashMap<Behaviour, behaviour::NodeIndex>,
}

impl BehaviourContext {
    pub fn new() -> Self {
        let mut graph = behaviour::Graph::new();
        let mut behaviours = HashMap::new();

        let east = graph.add_leaf(Leaf(Box::new(|input: BehaviourInput| {
            let walk = MetaAction::Update(actions::walk(input.entity, Direction::East));
            behaviour::LeafResolution::Yield(walk)
        })));
        let west = graph.add_leaf(Leaf(Box::new(|input: BehaviourInput| {
            let walk = MetaAction::Update(actions::walk(input.entity, Direction::West));
            behaviour::LeafResolution::Yield(walk)
        })));
        let back_and_forth = graph.add_collection(behaviour::CollectionNode::All(vec![east, west]));
        let back_and_forth_forever =
            graph.add_collection(behaviour::CollectionNode::Forever(back_and_forth));

        let player_input_once = graph.add_leaf(Leaf(Box::new(|input: BehaviourInput| {
            let input_source = input.entity.input_source().expect("no input source");
            let meta_action = terminal_player_actor::act_retrying(&input_source,
                                                                  input.level,
                                                                  input.id,
                                                                  input.ids);
            behaviour::LeafResolution::Yield(meta_action)
        })));

        let player_input =
            graph.add_collection(behaviour::CollectionNode::Forever(player_input_once));

        behaviours.insert(Behaviour::BackAndForthForever, back_and_forth_forever);
        behaviours.insert(Behaviour::PlayerInput, player_input);

        BehaviourContext {
            graph: graph,
            behaviours: behaviours,
        }
    }

    pub fn get_node_index(&self, behaviour: Behaviour) -> behaviour::NodeIndex {
        *self.behaviours.get(&behaviour).expect("missing behaviour")
    }
}
