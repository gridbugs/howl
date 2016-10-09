use std::result;

const MAX_STACK_DEPTH: usize = 1024;

#[derive(Debug)]
pub enum Error {
    InvalidNodeIndex,
    InvalidNodeState,
    StackEmpty,
    StackOverflow,
    Yielding,
    NotYielding,
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BehaviourOutcome {
    Succeed,
    Fail,
}

pub struct BehaviourGraph<Knowledge, Action> {
    nodes: Vec<BehaviourNode<Knowledge, Action>>,
}

#[derive(Clone, Copy)]
pub struct BehaviourNodeIndex(usize);

pub enum BehaviourNodeLeaf<Knowledge, Action> {
    Action(Box<Fn(&Knowledge) -> Action>),
    Predicate(Box<Fn(&Knowledge) -> BehaviourOutcome>),
    Succeed,
    Fail,
}

pub enum BehaviourNodeInternal {
    Identity(BehaviourNodeIndex),
    Forever(BehaviourNodeIndex),
    All(Vec<BehaviourNodeIndex>),
    Sequence(Vec<BehaviourNodeIndex>),
    Selector(Vec<BehaviourNodeIndex>),
}

enum BehaviourNode<Knowledge, Action>  {
    Leaf(BehaviourNodeLeaf<Knowledge, Action>),
    Internal(BehaviourNodeInternal),
}

enum BehaviourNodeState {
    IdentityState { resolved: bool, outcome: BehaviourOutcome },
    AllState { index: usize, outcome: BehaviourOutcome },
    SequenceState { index: usize, outcome: BehaviourOutcome },
    SelectorState { index: usize, outcome: BehaviourOutcome },
}

struct BehaviourStackFrame {
    node: BehaviourNodeIndex,
    state: Option<BehaviourNodeState>,
}

pub struct BehaviourState {
    stack: Vec<BehaviourStackFrame>,
    yielding: bool,
}

enum BehaviourNodeStackResolution {
    Push(BehaviourNodeIndex),
    Pop(BehaviourOutcome),
}

enum BehaviourNodeResolution<Action> {
    Stack(BehaviourNodeStackResolution),
    YieldAction(Action),
}

impl BehaviourNodeState {
    fn handle_return(&mut self, value: BehaviourOutcome) {
        use self::BehaviourNodeState::*;

        match self {
            &mut IdentityState { ref mut outcome, .. } |
                &mut AllState { ref mut outcome, .. } |
                &mut SequenceState { ref mut outcome, .. } |
                &mut SelectorState { ref mut outcome, .. }
            => {
                *outcome = value;
            }
        }
    }
}

impl<K, A> BehaviourNode<K, A> {
    fn new_state(&self) -> Option<BehaviourNodeState> {
        match self {
            &BehaviourNode::Internal(ref node) => node.new_state(),
            &BehaviourNode::Leaf(..) => None,
        }
    }

    fn resolve(&self, state: Option<&mut BehaviourNodeState>, knowledge: &K) -> Result<BehaviourNodeResolution<A>> {
        match self {
            &BehaviourNode::Internal(ref node) => Ok(BehaviourNodeResolution::Stack(try!(node.resolve(state)))),
            &BehaviourNode::Leaf(ref node) => Ok(node.resolve(knowledge)),
        }
    }
}

impl BehaviourNodeInternal {
    fn new_state(&self) -> Option<BehaviourNodeState> {
        use self::BehaviourNodeState::*;
        use self::BehaviourNodeInternal::*;
        use self::BehaviourOutcome::*;

        match self {
            &All(..) => Some(AllState { index: 0, outcome: Succeed }),
            &Identity(..) => Some(IdentityState { resolved: false, outcome: Succeed }),
            &Sequence(..) => Some(SequenceState { index: 0, outcome: Succeed }),
            &Selector(..) => Some(SelectorState { index: 0, outcome: Fail }),
            _ => None,
        }
    }

    fn resolve(&self, state: Option<&mut BehaviourNodeState>) -> Result<BehaviourNodeStackResolution> {
        use self::BehaviourNodeState::*;
        use self::BehaviourNodeInternal::*;
        use self::BehaviourOutcome::*;
        use self::BehaviourNodeStackResolution::*;

        match self {
            &Forever(idx) => Ok(Push(idx)),
            &Identity(idx) => {
                if let Some(&mut IdentityState { ref mut resolved, outcome }) = state {
                    if *resolved {
                        Ok(Pop(outcome))
                    } else {
                        *resolved = true;
                        Ok(Push(idx))
                    }
                } else {
                    Err(Error::InvalidNodeState)
                }
            }
            &All(ref indices) => {
                if let Some(&mut AllState { ref mut index, outcome }) = state {
                    if *index < indices.len() {
                        let node_idx = indices[*index];
                        *index += 1;

                        Ok(Push(node_idx))
                    } else {
                        Ok(Pop(outcome))
                    }
                } else {
                    Err(Error::InvalidNodeState)
                }
            }
            &Sequence(ref indices) => {
                if let Some(&mut SequenceState { ref mut index, outcome }) = state {
                    if outcome == Fail || *index == indices.len() {
                        Ok(Pop(outcome))
                    } else {
                        let node_idx = indices[*index];
                        *index += 1;

                        Ok(Push(node_idx))
                    }
                } else {
                    Err(Error::InvalidNodeState)
                }
            }
            &Selector(ref indices) => {
                if let Some(&mut SelectorState { ref mut index, outcome }) = state {
                    if outcome == Succeed || *index == indices.len() {
                        Ok(Pop(outcome))
                    } else {
                        let node_idx = indices[*index];
                        *index += 1;

                        Ok(Push(node_idx))
                    }
                } else {
                    Err(Error::InvalidNodeState)
                }
            }
        }
    }
}

impl<K, A> BehaviourNodeLeaf<K, A> {
    fn resolve(&self, knowledge: &K) -> BehaviourNodeResolution<A> {
        use self::BehaviourNodeResolution::*;
        use self::BehaviourNodeStackResolution::*;
        use self::BehaviourOutcome::*;

        match self {
            &BehaviourNodeLeaf::Succeed => Stack(Pop(Succeed)),
            &BehaviourNodeLeaf::Fail => Stack(Pop(Fail)),
            &BehaviourNodeLeaf::Predicate(ref p) => Stack(Pop(p(knowledge))),
            &BehaviourNodeLeaf::Action(ref afn) => YieldAction(afn(knowledge)),
        }
    }
}

impl BehaviourStackFrame {
    fn new(node: BehaviourNodeIndex, state: Option<BehaviourNodeState>) -> Self {
        BehaviourStackFrame {
            node: node,
            state: state,
        }
    }
}

impl<K, A> BehaviourGraph<K, A> {
    pub fn new() -> Self {
        BehaviourGraph {
            nodes: Vec::new(),
        }
    }

    pub fn add_leaf(&mut self, node: BehaviourNodeLeaf<K, A>) -> BehaviourNodeIndex {
        let index = self.nodes.len();
        self.nodes.push(BehaviourNode::Leaf(node));

        BehaviourNodeIndex(index)
    }

    pub fn add_internal(&mut self, node: BehaviourNodeInternal) -> BehaviourNodeIndex {
        let index = self.nodes.len();
        self.nodes.push(BehaviourNode::Internal(node));

        BehaviourNodeIndex(index)
    }

    fn node(&self, index: BehaviourNodeIndex) -> Result<&BehaviourNode<K, A>> {
        if index.0 < self.nodes.len() {
            Ok(&self.nodes[index.0])
        } else {
            Err(Error::InvalidNodeIndex)
        }
    }
}

impl BehaviourState {
    pub fn new() -> Self {
        BehaviourState {
            stack: Vec::new(),
            yielding: false,
        }
    }

    pub fn start<K, A>(&mut self, graph: &BehaviourGraph<K, A>, root: BehaviourNodeIndex) -> Result<()> {

        let root_node = try!(graph.node(root));
        let frame_state = root_node.new_state();
        let frame = BehaviourStackFrame::new(root, frame_state);
        self.stack.push(frame);

        Ok(())
    }

    pub fn run<K, A>(&mut self, graph: &BehaviourGraph<K, A>, knowledge: &K) -> Result<A> {
        use self::BehaviourNodeStackResolution::*;
        use self::BehaviourNodeResolution::*;

        if self.yielding {
            return Err(Error::Yielding);
        }

        loop {
            let resolution = if let Some(frame) = self.stack.last_mut() {
                let node = try!(graph.node(frame.node));
                try!(node.resolve(frame.state.as_mut(), knowledge))
            } else {
                return Err(Error::StackEmpty);
            };

            match resolution {
                Stack(Push(node_index)) => {
                    let child_node = try!(graph.node(node_index));
                    let child_state = child_node.new_state();
                    let child_frame = BehaviourStackFrame::new(node_index, child_state);
                    self.stack.push(child_frame);
                }
                Stack(Pop(outcome)) => {
                    self.stack.pop();
                    if let Some(mut frame) = self.stack.last_mut() {
                        if let &mut Some(ref mut state) = &mut frame.state {
                            state.handle_return(outcome);
                        }
                    }
                }
                YieldAction(action) => {
                    self.yielding = true;
                    return Ok(action);
                }
            }
        }
    }

    pub fn is_yielding(&self) -> bool { self.yielding }

    pub fn give_outcome(&mut self, outcome: BehaviourOutcome) -> Result<()> {
        if !self.yielding {
            return Err(Error::NotYielding);
        }

        self.yielding = false;

        self.stack.pop();

        if let Some(mut frame) = self.stack.last_mut() {
            if let &mut Some(ref mut state) = &mut frame.state {
                state.handle_return(outcome);
            }
        }

        Ok(())
    }
}
