use game::ActionArgs;
use behaviour::*;
use ecs::*;


pub type BehaviourState = State;
pub type BehaviourNodeIndex = NodeIndex;

#[derive(Clone, Copy)]
pub struct BehaviourInput<'a> {
    pub entity: EntityRef<'a>,
}

pub struct BehaviourLeaf(Box<Fn(BehaviourInput) -> LeafResolution<ActionArgs>>);

pub struct BehaviourSwitch {
    call: Box<Fn(BehaviourInput) -> SwitchResolution>,
    return_to: Box<Fn(bool) -> SwitchReturn>,
}

pub type BehaviourGraph = Graph<BehaviourLeaf, BehaviourSwitch>;

impl<'a> LeafFn<BehaviourInput<'a>, ActionArgs> for BehaviourLeaf {
    fn call(&self, input: BehaviourInput<'a>) -> LeafResolution<ActionArgs> {
        (self.0)(input)
    }
}

impl<'a> SwitchFn<BehaviourInput<'a>> for BehaviourSwitch {
    fn call(&self, input: BehaviourInput<'a>) -> SwitchResolution {
        (self.call)(input)
    }

    fn return_to(&self, value: bool) -> SwitchReturn {
        (self.return_to)(value)
    }
}

impl BehaviourLeaf {
    pub fn new<F: 'static + Fn(BehaviourInput) -> LeafResolution<ActionArgs>>(f: F) -> Self {
        BehaviourLeaf(Box::new(f))
    }
}

impl BehaviourSwitch {
    pub fn new_returning<F: 'static + Fn(BehaviourInput) -> SwitchResolution>(f: F) -> Self {
        BehaviourSwitch {
            call: Box::new(f),
            return_to: Box::new(|value| SwitchReturn::Return(value)),
        }
    }
}
