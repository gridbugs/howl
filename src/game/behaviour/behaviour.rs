use std::cell::RefCell;

use game::*;
use ecs::*;
use behaviour::{State, NodeIndex, Graph, LeafFn, SwitchFn, SwitchReturn, SwitchResolution, LeafResolution};


pub type BehaviourState = State;
pub type BehaviourNodeIndex = NodeIndex;

pub struct BehaviourInput<'a, R: 'a + KnowledgeRenderer> {
    pub entity: EntityRef<'a>,
    pub spatial_hash: &'a SpatialHashTable,
    pub level_id: LevelId,
    pub action_env: ActionEnv<'a>,
    pub renderer: &'a RefCell<R>,
    pub rng: &'a GameRng,
    pub language: &'a Box<Language>,
}

impl<'a, R: KnowledgeRenderer> Clone for BehaviourInput<'a, R> {
    fn clone(&self) -> Self {
        BehaviourInput {
            entity: self.entity,
            spatial_hash: self.spatial_hash,
            level_id: self.level_id,
            action_env: self.action_env,
            renderer: self.renderer,
            rng: self.rng,
            language: self.language,
        }
    }
}

impl<'a, R: KnowledgeRenderer> Copy for BehaviourInput<'a, R> {}


pub struct BehaviourLeaf<R: KnowledgeRenderer>(Box<Fn(BehaviourInput<R>) -> LeafResolution<MetaAction>>);

pub struct BehaviourSwitch<R: KnowledgeRenderer> {
    call: Box<Fn(BehaviourInput<R>) -> SwitchResolution>,
    return_to: Box<Fn(bool) -> SwitchReturn>,
}

pub type BehaviourGraph<K> = Graph<BehaviourLeaf<K>, BehaviourSwitch<K>>;

impl<'a, R: KnowledgeRenderer> LeafFn<BehaviourInput<'a, R>, MetaAction> for BehaviourLeaf<R> {
    fn call(&self, input: BehaviourInput<'a, R>) -> LeafResolution<MetaAction> {
        (self.0)(input)
    }
}

impl<'a, R: KnowledgeRenderer> SwitchFn<BehaviourInput<'a, R>> for BehaviourSwitch<R> {
    fn call(&self, input: BehaviourInput<'a, R>) -> SwitchResolution {
        (self.call)(input)
    }

    fn return_to(&self, value: bool) -> SwitchReturn {
        (self.return_to)(value)
    }
}

impl<R: KnowledgeRenderer> BehaviourLeaf<R> {
    pub fn new<F: 'static + Fn(BehaviourInput<R>) -> LeafResolution<MetaAction>>(f: F) -> Self {
        BehaviourLeaf(Box::new(f))
    }
}

impl<R: KnowledgeRenderer> BehaviourSwitch<R> {
    pub fn new_returning<F: 'static + Fn(BehaviourInput<R>) -> SwitchResolution>(f: F) -> Self {
        BehaviourSwitch {
            call: Box::new(f),
            return_to: Box::new(|value| SwitchReturn::Return(value)),
        }
    }
}
