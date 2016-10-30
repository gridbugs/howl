use game::{MetaAction, actions, Level, EntityId, ReserveEntityId, EntityStore, LevelEntityRef,
           EntityWrapper};
use game::io::terminal_player_actor;

use vision::{VisionSystem, DefaultVisibilityReport, Shadowcast};
use behaviour::*;
use geometry::Direction;
use grid::{Grid, IterGrid};
use search::{WeightedGridSearchContext, Config};

use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use std::mem;
use std::ops::DerefMut;

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

pub struct Leaf(Box<Fn(BehaviourInput) -> LeafResolution<MetaAction>>);
pub struct Switch {
    call: Box<Fn(BehaviourInput) -> SwitchResolution>,
    return_to: Box<Fn(bool) -> SwitchReturn>,
}

impl Leaf {
    fn new<F: 'static + Fn(BehaviourInput) -> LeafResolution<MetaAction>>(f: F) -> Self {
        Leaf(Box::new(f))
    }
}

impl Switch {
    fn new_returning<F: 'static + Fn(BehaviourInput) -> SwitchResolution>(f: F) -> Self {
        Switch {
            call: Box::new(f),
            return_to: Box::new(|value| SwitchReturn::Return(value)),
        }
    }
}

impl<'a> LeafFn<BehaviourInput<'a>, MetaAction> for Leaf {
    fn call(&self, knowledge: BehaviourInput<'a>) -> LeafResolution<MetaAction> {
        (self.0)(knowledge)
    }
}

impl<'a> SwitchFn<BehaviourInput<'a>> for Switch {
    fn call(&self, knowledge: BehaviourInput<'a>) -> SwitchResolution {
        (self.call)(knowledge)
    }

    fn return_to(&self, value: bool) -> SwitchReturn {
        (self.return_to)(value)
    }
}

pub type BehaviourGraph = Graph<Leaf, Switch>;

pub struct BehaviourContext {
    pub graph: BehaviourGraph,
    behaviours: HashMap<Behaviour, NodeIndex>,
}

fn observe_node(child: NodeIndex) -> Switch {
    let _visibility_report = RefCell::new(DefaultVisibilityReport::new());
    let vision_system = Shadowcast::new();
    Switch::new_returning(move |input: BehaviourInput| {

        let grid = input.level.spatial_hash().grid();
        let eye = input.entity.position().unwrap();
        let info = input.entity.vision_distance().unwrap();

        let mut visibility_report = _visibility_report.borrow_mut();
        visibility_report.clear();
        vision_system.detect_visible_area(eye, grid, info, visibility_report);

        let visibility_report = _visibility_report.borrow();
        let mut knowledge = input.entity.simple_npc_knowledge_mut().unwrap();
        knowledge.update(input.level, grid, visibility_report.iter(), input.turn);

        SwitchResolution::Select(child)
    })
}

fn choose_target_node(child: NodeIndex) -> Switch {
    let targets = RefCell::new(HashSet::new());
    Switch::new_returning(move |input: BehaviourInput| {
        let mut targets = targets.borrow_mut();
        targets.clear();

        let mut turn = 0;

        let knowledge = input.entity.simple_npc_knowledge().unwrap();
        let knowledge_grid = knowledge.grid(input.level.id()).unwrap().inner();

        for (coord, cell) in izip!(knowledge_grid.coord_iter(), knowledge_grid.iter()) {
            if cell.data().player {
                let last_updated_turn = cell.last_updated_turn();
                if turn == last_updated_turn {
                    targets.insert(coord);
                } else if turn < last_updated_turn {
                    turn = last_updated_turn;
                    targets.clear();
                    targets.insert(coord);
                }
            }
        }

        let mut target_set = input.entity.target_set_mut().unwrap();
        if *target_set != *targets {
            mem::swap(target_set.deref_mut(), targets.deref_mut());
            SwitchResolution::Reset(child)
        } else {
            SwitchResolution::Select(child)
        }
    })
}

fn update_path_node() -> Leaf {
    let _search_context = RefCell::new(WeightedGridSearchContext::new());
    Leaf::new(move |input: BehaviourInput| {
        let mut search_context = _search_context.borrow_mut();
        let target_set = input.entity.target_set().unwrap();

        let position = input.entity.position().unwrap();

        let knowledge = input.entity.simple_npc_knowledge().unwrap();
        let knowledge_grid = knowledge.grid(input.level.id()).unwrap().inner();

        let config = Config::new_all_directions();

        let mut path_traverse = input.entity.path_traverse_mut().unwrap();

        // TODO optimized search function for sets
        let err = search_context.search_predicate(knowledge_grid,
                                                  position,
                                                  |info| target_set.contains(&info.coord),
                                                  &config,
                                                  path_traverse.path_mut());

        if err.is_err() {
            return LeafResolution::Return(false);
        }

        path_traverse.reset();

        LeafResolution::Return(true)
    })
}

fn follow_path_step_node() -> Leaf {
    Leaf::new(|input: BehaviourInput| {

        let mut path_traverse = input.entity.path_traverse_mut().unwrap();
        let direction = path_traverse.next_direction().unwrap();
        let action = MetaAction::Update(actions::walk(input.entity, direction));
        LeafResolution::Yield(action)
    })
}

impl BehaviourContext {
    pub fn new() -> Self {
        let mut graph = Graph::new();
        let mut behaviours = HashMap::new();

        let east = graph.add_leaf(Leaf::new(|input: BehaviourInput| {
            let walk = MetaAction::Update(actions::walk(input.entity, Direction::East));
            LeafResolution::Yield(walk)
        }));
        let west = graph.add_leaf(Leaf::new(|input: BehaviourInput| {
            let walk = MetaAction::Update(actions::walk(input.entity, Direction::West));
            LeafResolution::Yield(walk)
        }));

        let follow_path_step = graph.add_leaf(follow_path_step_node());
        let follow_path = graph.add_collection(CollectionNode::Forever(follow_path_step));

        let back_and_forth = graph.add_collection(CollectionNode::All(vec![east, west]));
        let back_and_forth_forever = graph.add_collection(CollectionNode::Forever(back_and_forth));
        let update_path = graph.add_leaf(update_path_node());

        let back_and_forth_forever =
            graph.add_collection(CollectionNode::All(vec![update_path, follow_path, back_and_forth_forever]));

        let back_and_forth_forever = graph.add_switch(choose_target_node(back_and_forth_forever));
        let back_and_forth_forever = graph.add_switch(observe_node(back_and_forth_forever));

        let player_input_once = graph.add_leaf(Leaf::new(|input: BehaviourInput| {
            let input_source = input.entity.input_source().expect("no input source");
            let meta_action = terminal_player_actor::act_retrying(&input_source,
                                                                  input.level,
                                                                  input.entity_id,
                                                                  input.ids);
            LeafResolution::Yield(meta_action)
        }));

        let player_input = graph.add_collection(CollectionNode::Forever(player_input_once));

        behaviours.insert(Behaviour::BackAndForthForever, back_and_forth_forever);
        behaviours.insert(Behaviour::PlayerInput, player_input);

        BehaviourContext {
            graph: graph,
            behaviours: behaviours,
        }
    }

    pub fn get_node_index(&self, behaviour: Behaviour) -> NodeIndex {
        *self.behaviours.get(&behaviour).expect("missing behaviour")
    }
}
