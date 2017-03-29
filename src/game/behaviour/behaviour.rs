use rand::StdRng;
use engine_defs::*;

use game::*;
use message::*;
use ecs_core::EntityId;
use ecs_content::*;
use spatial_hash::*;
use control::ControlMap;
use behaviour::{Graph, LeafFn, SwitchFn, SwitchReturn, SwitchResolution, LeafResolution};

pub struct BehaviourInput<'a, R: 'a + KnowledgeRenderer> {
    pub entity_id: EntityId,
    pub ecs: &'a mut EcsCtx,
    pub spatial_hash: &'a mut SpatialHashTable,
    pub level_id: LevelId,
    pub action_id: u64,
    pub renderer: &'a mut R,
    pub rng: &'a mut StdRng,
    pub language: &'a Box<Language>,
    pub control_map: &'a ControlMap,
}

pub struct BehaviourLeaf<R: KnowledgeRenderer>(Box<Fn(&mut BehaviourInput<R>) -> LeafResolution<MetaAction>>);

pub struct BehaviourSwitch<R: KnowledgeRenderer> {
    call: Box<Fn(&mut BehaviourInput<R>) -> SwitchResolution>,
    return_to: Box<Fn(bool) -> SwitchReturn>,
}

pub type BehaviourGraph<K> = Graph<BehaviourLeaf<K>, BehaviourSwitch<K>>;

impl<'a, R: KnowledgeRenderer> LeafFn<BehaviourInput<'a, R>, MetaAction> for BehaviourLeaf<R> {
    fn call(&self, input: &mut BehaviourInput<'a, R>) -> LeafResolution<MetaAction> {
        (self.0)(input)
    }
}

impl<'a, R: KnowledgeRenderer> SwitchFn<BehaviourInput<'a, R>> for BehaviourSwitch<R> {
    fn call(&self, input: &mut BehaviourInput<'a, R>) -> SwitchResolution {
        (self.call)(input)
    }

    fn return_to(&self, value: bool) -> SwitchReturn {
        (self.return_to)(value)
    }
}

impl<R: KnowledgeRenderer> BehaviourLeaf<R> {
    pub fn new<F: 'static + Fn(&mut BehaviourInput<R>) -> LeafResolution<MetaAction>>(f: F) -> Self {
        BehaviourLeaf(Box::new(f))
    }
}

impl<R: KnowledgeRenderer> BehaviourSwitch<R> {
    pub fn new_returning<F: 'static + Fn(&mut BehaviourInput<R>) -> SwitchResolution>(f: F) -> Self {
        BehaviourSwitch {
            call: Box::new(f),
            return_to: Box::new(|value| SwitchReturn::Return(value)),
        }
    }
}
