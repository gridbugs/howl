use std::result;

#[derive(Debug)]
pub enum Error {
    UnknownNodeIndex,
    StateAlreadyInitialised,
    NoStack,
    EmptyStack,
    ReturnToLeaf,
    InvalidStack,
    Yielding,
    NotYielding,
    UnexpectedNodeType,
    NodeStateMismatch,
}

pub type Result<T> = result::Result<T, Error>;

pub type NodeIndex = usize;

pub enum LeafResolution<A> {
    Return(bool),
    Yield(A),
    ReturnFromInterrupt,
}

pub enum CheckResolution {
    Restart,
    Interrupt(NodeIndex),
    ReturnFromInterrupt,
}

enum Resolution<A> {
    Return(bool),
    Call(NodeIndex),
    Yield(A),
    PopStack,
}

enum CheckResolutionInternal {
    SeverStack(usize),
    PushStack(NodeIndex),
    PopStack,
}

pub type LeafFn<K, A> = Box<Fn(K) -> LeafResolution<A>>;
pub type CheckFn<K> = Box<Fn(K) -> Option<CheckResolution>>;

enum Node<K, A> {
    Leaf(Box<Fn(K) -> LeafResolution<A>>),
    Check {
        condition: CheckFn<K>,
        child: NodeIndex,
    },
    Collection(CollectionNode),
}

pub enum CollectionNode {
    Forever(NodeIndex),
    All(Vec<NodeIndex>),
}

struct ArrayTraverse {
    index: usize,
    length: usize,
    value: bool,
}

enum CollectionState {
    Forever,
    All(ArrayTraverse),
}

pub struct Graph<K, A> {
    nodes: Vec<Node<K, A>>,
}

enum StackFrame {
    Leaf(NodeIndex),
    Check {
        index: NodeIndex,
        value: Option<bool>,
    },
    Collection {
        index: NodeIndex,
        state: CollectionState,
    },
}

type Stack = Vec<StackFrame>;

pub struct State {
    stacks: Vec<Stack>,
    yielding: bool,
}

impl<A> LeafResolution<A> {
    fn to_resolution(self) -> Resolution<A> {
        match self {
            LeafResolution::Return(value) => Resolution::Return(value),
            LeafResolution::Yield(action) => Resolution::Yield(action),
            LeafResolution::ReturnFromInterrupt => Resolution::PopStack,
        }
    }
}

impl CheckResolution {
    fn to_internal(self, frame_index: usize) -> CheckResolutionInternal {
        match self {
            CheckResolution::Restart => CheckResolutionInternal::SeverStack(frame_index),
            CheckResolution::Interrupt(index) => CheckResolutionInternal::PushStack(index),
            CheckResolution::ReturnFromInterrupt => CheckResolutionInternal::PopStack,
        }
    }
}

impl ArrayTraverse {
    fn new(length: usize, value: bool) -> Self {
        ArrayTraverse {
            index: 0,
            length: length,
            value: value,
        }
    }

    fn next_index(&mut self) -> Option<usize> {
        if self.index < self.length {
            let index = self.index;
            self.index += 1;
            Some(index)
        } else {
            None
        }
    }
}

impl CollectionNode {
    fn create_state(&self) -> CollectionState {
        match self {
            &CollectionNode::Forever(_) => CollectionState::Forever,
            &CollectionNode::All(ref v) => CollectionState::All(ArrayTraverse::new(v.len(), false)),
        }
    }

    fn next_index(&self, state: &mut CollectionState) -> Result<Option<NodeIndex>> {
        match (self, state) {
            (&CollectionNode::Forever(index), &mut CollectionState::Forever) => Ok(Some(index)),
            (&CollectionNode::All(ref indices), &mut CollectionState::All(ref mut at)) => {
                Ok(at.next_index().map(|i| indices[i]))
            }
            _ => Err(Error::NodeStateMismatch),
        }
    }
}

impl CollectionState {
    fn handle_return(&mut self, value: bool) {
        match self {
            &mut CollectionState::All(ref mut at) => at.value = value,
            _ => {}
        }
    }

    fn return_value(&self) -> bool {
        match self {
            &CollectionState::All(ref at) => at.value,
            _ => panic!("impossible return"),
        }
    }
}

impl StackFrame {
    fn handle_return(&mut self, return_value: bool) -> Result<()> {
        match self {
            &mut StackFrame::Leaf(_) => return Err(Error::ReturnToLeaf),
            &mut StackFrame::Check { ref mut value, .. } => {
                *value = Some(return_value);
            }
            &mut StackFrame::Collection { ref mut state, .. } => {
                state.handle_return(return_value);
            }
        }

        Ok(())
    }
}

impl<K, A> Node<K, A> {
    fn leaf_fn(&self) -> Result<&LeafFn<K, A>> {
        if let &Node::Leaf(ref f) = self {
            Ok(f)
        } else {
            Err(Error::UnexpectedNodeType)
        }
    }

    fn check_condition(&self) -> Result<&CheckFn<K>> {
        if let &Node::Check { ref condition, .. } = self {
            Ok(condition)
        } else {
            Err(Error::UnexpectedNodeType)
        }
    }

    fn check_child(&self) -> Result<NodeIndex> {
        if let &Node::Check { child, .. } = self {
            Ok(child)
        } else {
            Err(Error::UnexpectedNodeType)
        }
    }

    fn collection_node(&self) -> Result<&CollectionNode> {
        if let &Node::Collection(ref n) = self {
            Ok(n)
        } else {
            Err(Error::UnexpectedNodeType)
        }
    }
}

impl<K, A> Graph<K, A> {
    pub fn new() -> Self {
        Graph { nodes: Vec::new() }
    }

    fn add_node(&mut self, node: Node<K, A>) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(node);
        return index;
    }

    pub fn add_leaf(&mut self, leaf: LeafFn<K, A>) -> NodeIndex {
        self.add_node(Node::Leaf(leaf))
    }

    pub fn add_check(&mut self, child: NodeIndex, condition: CheckFn<K>) -> NodeIndex {
        self.add_node(Node::Check {
            condition: condition,
            child: child,
        })
    }

    pub fn add_collection(&mut self, collection: CollectionNode) -> NodeIndex {
        self.add_node(Node::Collection(collection))
    }

    fn node(&self, index: NodeIndex) -> Result<&Node<K, A>> {
        if index < self.nodes.len() {
            Ok(&self.nodes[index])
        } else {
            Err(Error::UnknownNodeIndex)
        }
    }

    fn create_stack_frame(&self, index: NodeIndex) -> Result<StackFrame> {
        match try!(self.node(index)) {
            &Node::Leaf(_) => Ok(StackFrame::Leaf(index)),
            &Node::Check { .. } => {
                Ok(StackFrame::Check {
                    index: index,
                    value: None,
                })
            }
            &Node::Collection(ref c) => {
                Ok(StackFrame::Collection {
                    index: index,
                    state: c.create_state(),
                })
            }
        }
    }

    fn resolve_frame(&self, frame: &mut StackFrame, knowledge: K) -> Result<Resolution<A>> {
        match frame {
            &mut StackFrame::Leaf(index) => {
                let node = try!(self.node(index));
                let f = try!(node.leaf_fn());
                Ok(f(knowledge).to_resolution())
            }
            &mut StackFrame::Check { index, value } => {
                let node = try!(self.node(index));
                let child = try!(node.check_child());
                if let Some(return_value) = value {
                    Ok(Resolution::Return(return_value))
                } else {
                    Ok(Resolution::Call(child))
                }
            }
            &mut StackFrame::Collection { index, ref mut state } => {
                let node = try!(self.node(index));
                let collection_node = try!(node.collection_node());
                if let Some(child_index) = try!(collection_node.next_index(state)) {
                    Ok(Resolution::Call(child_index))
                } else {
                    Ok(Resolution::Return(state.return_value()))
                }
            }
        }
    }
}

impl State {
    pub fn new() -> Self {
        State {
            stacks: Vec::new(),
            yielding: false,
        }
    }

    pub fn is_clear(&self) -> bool {
        self.stacks.is_empty()
    }

    pub fn clear(&mut self) {
        self.stacks.clear();
        self.yielding = false;
    }

    pub fn is_yielding(&self) -> bool {
        self.yielding
    }

    pub fn initialise<K, A>(&mut self, graph: &Graph<K, A>, index: NodeIndex) -> Result<()> {
        if !self.is_clear() {
            return Err(Error::StateAlreadyInitialised);
        }

        self.push_stack(graph, index)
    }

    fn push_stack<K, A>(&mut self, graph: &Graph<K, A>, index: NodeIndex) -> Result<()> {
        self.stacks.push(vec![try!(graph.create_stack_frame(index))]);

        Ok(())
    }

    fn pop_stack(&mut self) -> Result<()> {
        try!(self.stacks.pop().ok_or(Error::InvalidStack));
        Ok(())
    }

    fn current_stack_mut(&mut self) -> Result<&mut Stack> {
        self.stacks.last_mut().ok_or(Error::NoStack)
    }

    fn current_stack(&self) -> Result<&Stack> {
        self.stacks.last().ok_or(Error::NoStack)
    }

    fn current_frame_mut(&mut self) -> Result<&mut StackFrame> {
        let mut stack = try!(self.current_stack_mut());
        stack.last_mut().ok_or(Error::EmptyStack)
    }

    fn apply_return(&mut self, value: bool) -> Result<()> {
        {
            let mut stack = try!(self.current_stack_mut());

            if stack.pop().is_none() {
                return Err(Error::EmptyStack);
            }

            if let Some(frame) = stack.last_mut() {
                return frame.handle_return(value);
            }
        }

        // if we're here, it means the top-most frame of the
        // current stack just returned

        self.pop_stack()
    }

    fn apply_resolution<K, A>(&mut self,
                              graph: &Graph<K, A>,
                              resolution: Resolution<A>)
                              -> Result<Option<A>> {

        match resolution {
            Resolution::Return(value) => {
                try!(self.apply_return(value));
            }
            Resolution::Call(index) => {
                let frame = try!(graph.create_stack_frame(index));
                let mut stack = try!(self.current_stack_mut());
                stack.push(frame);
            }
            Resolution::Yield(action) => {
                return Ok(Some(action));
            }
            Resolution::PopStack => {
                try!(self.pop_stack());
            }
       }

        Ok(None)
    }

    fn apply_check_resolution<K, A>(&mut self,
                                    graph: &Graph<K, A>,
                                    resolution: CheckResolutionInternal)
                                    -> Result<()> {

        match resolution {
            CheckResolutionInternal::SeverStack(frame_index) => {
                let mut stack = try!(self.current_stack_mut());
                stack.truncate(frame_index + 1);
            }
            CheckResolutionInternal::PushStack(index) => {
                try!(self.push_stack(graph, index));
            }
            CheckResolutionInternal::PopStack => {
                try!(self.pop_stack());
            }
        }

        Ok(())
    }

    pub fn run_to_action<K: Copy, A>(&mut self, graph: &Graph<K, A>, knowledge: K) -> Result<A> {

        if self.is_yielding() {
            return Err(Error::Yielding);
        }

        if let Some(resolution) = try!(self.check(graph, knowledge)) {
            try!(self.apply_check_resolution(graph, resolution));
        }

        loop {
            let resolution = {
                let mut frame = try!(self.current_frame_mut());
                try!(graph.resolve_frame(frame, knowledge))
            };

            if let Some(action) = try!(self.apply_resolution(graph, resolution)) {
                self.yielding = true;
                return Ok(action);
            }
        }
    }

    pub fn report_action_result(&mut self, value: bool) -> Result<()> {
        if !self.is_yielding() {
            return Err(Error::NotYielding);
        }

        self.yielding = false;

        self.apply_return(value)
    }

    fn check<K: Copy, A>(&mut self, graph: &Graph<K, A>, knowledge: K) -> Result<Option<CheckResolutionInternal>> {

        let mut i = 0; // track current stack index

        for frame in try!(self.current_stack()) {
            if let &StackFrame::Check { index, .. } = frame {
                let node = try!(graph.node(index));
                let condition = try!(node.check_condition());
                if let Some(resolution) = condition(knowledge) {
                    return Ok(Some(resolution.to_internal(i)));
                }
            }

            i += 1;
        }

        Ok(None)
    }
}
