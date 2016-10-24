use game::{MetaAction, actions, Level, EntityId, ReserveEntityId, EntityStore, LevelEntityRef,
           EntityWrapper};
use game::io::terminal_player_actor;
use game::knowledge::SimpleNpcCell;

use vision::{VisionSystem, DefaultVisibilityReport, Shadowcast, VisibilityReport};
use behaviour;
use geometry::{Direction, Vector2};
use grid::{Coord, StaticGrid, IterGrid};

use debug;

use std::collections::HashMap;
use std::cell::{RefCell, RefMut};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Behaviour {
    BackAndForthForever,
    PlayerInput,
    FollowPlayer,
    Observer,
}

#[derive(Clone, Copy)]
pub struct BehaviourInput<'a> {
    entity_id: EntityId,
    ids: &'a ReserveEntityId,
    level: &'a Level,
    turn: u64,
    entity: LevelEntityRef<'a>,
}

impl<'a> BehaviourInput<'a> {
    pub fn new(entity_id: EntityId, ids: &'a ReserveEntityId, level: &'a Level, turn: u64) -> Self {
        BehaviourInput {
            entity_id: entity_id,
            ids: ids,
            level: level,
            turn: turn,
            entity: level.get(entity_id).unwrap(),
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

fn follow_player_node_choose_target(grid: &StaticGrid<SimpleNpcCell>, targets: RefMut<Vec<Coord>>) {
    debug_println!("choose target");
}

fn see_player_interrupt(handler: behaviour::NodeIndex) -> Check {
    Check(Box::new(move |input: BehaviourInput| {
        let knowledge = input.entity.simple_npc_knowledge().unwrap();
        let knowledge_grid = knowledge.grid(input.level.id()).unwrap().inner();
        for cell in knowledge_grid.iter() {
            // if the player character is visible
            if cell.data().player && cell.last_updated_turn() == input.turn {
                debug_println!("can see player");
                return Some(behaviour::CheckResolution::Interrupt(handler))
            }
        }

        None
    }))
}

fn follow_player_node(child: behaviour::NodeIndex) -> Check {
    let targets: RefCell<Vec<Coord>> = RefCell::new(Vec::new());
    Check(Box::new(move |input: BehaviourInput| {
        let mut targets = targets.borrow_mut();
        targets.clear();

        let knowledge = input.entity.simple_npc_knowledge().unwrap();
        let knowledge_grid = knowledge.grid(input.level.id()).unwrap().inner();

        follow_player_node_choose_target(knowledge_grid, targets);

        None
    }))
}

fn observe_node() -> Check {
    let _visibility_report = RefCell::new(DefaultVisibilityReport::new());
    let vision_system = Shadowcast::new();
    Check(Box::new(move |input: BehaviourInput| {

        let grid = input.level.spatial_hash().grid();
        let eye = input.entity.position().unwrap();
        let info = input.entity.vision_distance().unwrap();

        let mut visibility_report = _visibility_report.borrow_mut();
        visibility_report.clear();
        vision_system.detect_visible_area(eye, grid, info, visibility_report);

        let visibility_report = _visibility_report.borrow();
        let mut knowledge = input.entity.simple_npc_knowledge_mut().unwrap();
        knowledge.update(input.level, grid, visibility_report.iter(), input.turn);

        None
    }))
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
                                                                  input.entity_id,
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
