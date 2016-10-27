use std::result;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    UnknownNodeIndex,
    NodeStateMismatch,
    UnexpectedNodeType,
    StackEmpty,
    StackNotEmpty,
    StackInitialised,
    RootStackReturned,
    UnexpectedResolutionType,
    Yielding,
    NotYielding,
}

pub type Result<T> = result::Result<T, Error>;

pub type NodeIndex = usize;

pub enum LeafResolution<A> {
    Return(bool),
    Yield(A),
}

pub enum SwitchResolution {
    Select(NodeIndex),
    Reset(NodeIndex),
}

enum StepResolution<A> {
    Yield {
        action: A,
        state: YieldState,
    },
    Call(NodeIndex),
    Return(bool),
}

enum RunResolution<A> {
    Yield {
        action: A,
        state: YieldState,
    },
    Return(bool),
}

enum Resolution<'a, A> {
    Return(bool),
    Call(NodeIndex),
    Yield(A),
    StackSwitch(&'a mut Stack),
}

// Pointer to stack frame currently yielding.
// Using a pointer prevents having to traverse the state when
// handling returns from yielded actions.
struct YieldState {
    frame: *mut StackFrame,
}

pub trait LeafFn<K, A> {
    fn call(&self, knowledge: K) -> LeafResolution<A>;
}

pub trait SwitchFn<K> {
    fn call(&self, knowledge: K) -> SwitchResolution;
    fn return_to(&self, value: bool) -> Option<bool>;
}

enum Node<Leaf, Switch> {
    Leaf(Leaf),
    Collection(CollectionNode),
    Switch(Switch),
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

struct SwitchState {
    stacks: HashMap<NodeIndex, Stack>,
}

enum StackFrame {
    Leaf {
        index: NodeIndex,
        value: Option<bool>,
    },
    Collection {
        index: NodeIndex,
        state: CollectionState,
    },
    Switch {
        index: NodeIndex,
        state: SwitchState,
        value: Option<bool>,
    },
}

struct Stack {
    frames: Vec<StackFrame>,
    initialised: bool,
}

pub struct Graph<Leaf, Switch> {
    nodes: Vec<Node<Leaf, Switch>>,
}

pub struct State {
    root_stack: Stack,
    yield_state: Option<YieldState>,
}

impl YieldState {
    fn new(frame: &mut StackFrame) -> Self {
        YieldState { frame: frame }
    }
}

impl<A> RunResolution<A> {
    fn to_step_resolution(self) -> StepResolution<A> {
        match self {
            RunResolution::Return(value) => StepResolution::Return(value),
            RunResolution::Yield { action, state } => {
                StepResolution::Yield {
                    action: action,
                    state: state,
                }
            }
        }
    }
}

impl Stack {
    fn new() -> Self {
        Stack {
            frames: Vec::new(),
            initialised: false,
        }
    }

    fn push(&mut self, frame: StackFrame) {
        self.frames.push(frame);
    }

    fn pop(&mut self) -> Option<StackFrame> {
        self.frames.pop()
    }

    fn last_mut(&mut self) -> Option<&mut StackFrame> {
        self.frames.last_mut()
    }

    fn initialise(&mut self, frame: StackFrame) {
        self.push(frame);
        self.initialised = true;
    }

    fn is_initialised(&self) -> bool {
        self.initialised
    }

    fn reset(&mut self) {
        self.frames.clear();
        self.initialised = false;
    }

    fn is_empty(&self) -> bool {
        self.frames.is_empty()
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

impl SwitchState {
    fn new() -> Self {
        SwitchState { stacks: HashMap::new() }
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

impl<A> LeafResolution<A> {
    fn to_resolution<'a>(self) -> Resolution<'a, A> {
        match self {
            LeafResolution::Return(value) => Resolution::Return(value),
            LeafResolution::Yield(action) => Resolution::Yield(action),
        }
    }
}

impl StackFrame {
    fn handle_return(&mut self, return_value: bool) -> Result<()> {
        match self {
            &mut StackFrame::Leaf { ref mut value, .. } => {
                *value = Some(return_value);
            }
            &mut StackFrame::Collection { ref mut state, .. } => {
                state.handle_return(return_value);
            }
            &mut StackFrame::Switch { ref mut value, .. } => {
                *value = Some(return_value);
            }
        }

        Ok(())
    }
}

impl<Leaf, Switch> Node<Leaf, Switch> {
    fn leaf_fn(&self) -> Result<&Leaf> {
        if let &Node::Leaf(ref f) = self {
            Ok(f)
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

    fn switch_fn(&self) -> Result<&Switch> {
        if let &Node::Switch(ref f) = self {
            Ok(f)
        } else {
            Err(Error::UnexpectedNodeType)
        }
    }
}



impl<Leaf, Switch> Graph<Leaf, Switch> {
    pub fn new() -> Self {
        Graph { nodes: Vec::new() }
    }

    fn add_node(&mut self, node: Node<Leaf, Switch>) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(node);
        return index;
    }

    pub fn add_leaf(&mut self, leaf: Leaf) -> NodeIndex {
        self.add_node(Node::Leaf(leaf))
    }

    pub fn add_collection(&mut self, collection: CollectionNode) -> NodeIndex {
        self.add_node(Node::Collection(collection))
    }

    pub fn add_switch(&mut self, switch: Switch) -> NodeIndex {
        self.add_node(Node::Switch(switch))
    }

    fn node(&self, index: NodeIndex) -> Result<&Node<Leaf, Switch>> {
        if index < self.nodes.len() {
            Ok(&self.nodes[index])
        } else {
            Err(Error::UnknownNodeIndex)
        }
    }

    fn create_stack_frame(&self, index: NodeIndex) -> Result<StackFrame> {
        match try!(self.node(index)) {
            &Node::Leaf(_) => {
                Ok(StackFrame::Leaf {
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
            &Node::Switch(_) => {
                Ok(StackFrame::Switch {
                    index: index,
                    state: SwitchState::new(),
                    value: None,
                })
            }
        }
    }

    fn resolve_switch<'a>(&self,
                          state: &'a mut SwitchState,
                          resolution: SwitchResolution)
                          -> Result<&'a mut Stack> {
        match resolution {
            SwitchResolution::Select(index) => {
                let mut stack = state.stacks.entry(index).or_insert_with(Stack::new);

                if !stack.is_initialised() {
                    stack.initialise(try!(self.create_stack_frame(index)));
                }

                Ok(stack)
            }
            SwitchResolution::Reset(index) => {
                let mut stack = state.stacks.entry(index).or_insert_with(Stack::new);

                stack.reset();
                stack.initialise(try!(self.create_stack_frame(index)));

                Ok(stack)
            }
        }
    }

    fn apply_switch<'a, K>(&self,
                           index: NodeIndex,
                           state: &'a mut SwitchState,
                           knowledge: K)
                           -> Result<&'a mut Stack>
        where Switch: SwitchFn<K>
    {
        let node = try!(self.node(index));
        let f = try!(node.switch_fn());
        let switch_resolution = f.call(knowledge);
        self.resolve_switch(state, switch_resolution)
    }

    fn resolve_frame<'a, K, A>(&self,
                               frame: &'a mut StackFrame,
                               knowledge: K)
                               -> Result<Resolution<'a, A>>
        where Leaf: LeafFn<K, A>,
              Switch: SwitchFn<K>
    {
        match frame {
            &mut StackFrame::Leaf { index, value } => {
                if let Some(value) = value {
                    Ok(Resolution::Return(value))
                } else {
                    let node = try!(self.node(index));
                    let f = try!(node.leaf_fn());
                    Ok(f.call(knowledge).to_resolution())
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
            &mut StackFrame::Switch { index, ref mut state, value } => {
                if let Some(value) = value {
                    Ok(Resolution::Return(value))
                } else {
                    let stack = try!(self.apply_switch(index, state, knowledge));
                    Ok(Resolution::StackSwitch(stack))
                }
            }
        }
    }
}

impl State {
    pub fn new() -> Self {
        State {
            root_stack: Stack::new(),
            yield_state: None,
        }
    }

    fn is_yielding(&self) -> bool {
        self.yield_state.is_some()
    }

    pub fn initialise<Leaf, Switch>(&mut self,
                                    graph: &Graph<Leaf, Switch>,
                                    index: NodeIndex)
                                    -> Result<()> {

        if !self.root_stack.is_empty() {
            return Err(Error::StackNotEmpty);
        }

        if self.root_stack.is_initialised() {
            return Err(Error::StackInitialised);
        }

        let frame = try!(graph.create_stack_frame(index));
        self.root_stack.push(frame);

        Ok(())
    }

    pub fn run<K, A, Leaf, Switch>(&mut self,
                                   graph: &Graph<Leaf, Switch>,
                                   knowledge: K)
                                   -> Result<A>
        where K: Copy,
              Leaf: LeafFn<K, A>,
              Switch: SwitchFn<K>
    {

        if self.is_yielding() {
            return Err(Error::Yielding);
        }

        let resolution = try!(Self::run_on_stack(&mut self.root_stack, graph, knowledge));
        if let RunResolution::Yield { action, state } = resolution {
            self.yield_state = Some(state);
            Ok(action)
        } else {
            Err(Error::RootStackReturned)
        }
    }

    pub fn declare_return(&mut self, value: bool) -> Result<()> {
        if let Some(ref mut yield_state) = self.yield_state {

            unsafe {
                try!((*yield_state.frame).handle_return(value));
            }

        } else {
            return Err(Error::NotYielding);
        }

        self.yield_state = None;
        Ok(())
    }

    fn run_on_stack<K, A, Leaf, Switch>(stack: &mut Stack,
                                        graph: &Graph<Leaf, Switch>,
                                        knowledge: K)
                                        -> Result<RunResolution<A>>
        where K: Copy,
              Leaf: LeafFn<K, A>,
              Switch: SwitchFn<K>
    {
        loop {
            match try!(Self::step_on_stack(stack, graph, knowledge)) {
                StepResolution::Yield { action, state } => {
                    return Ok(RunResolution::Yield {
                        action: action,
                        state: state,
                    });
                }
                StepResolution::Call(index) => {
                    let frame = try!(graph.create_stack_frame(index));
                    stack.push(frame);
                }
                StepResolution::Return(value) => {
                    if stack.pop().is_none() {
                        return Err(Error::StackEmpty);
                    }

                    if let Some(frame) = stack.last_mut() {
                        try!(frame.handle_return(value));
                        continue;
                    }

                    // stack is empty
                    // reset stack so it can be re-initialised by parent
                    stack.reset();

                    // return control to parent
                    return Ok(RunResolution::Return(value));
                }
            }
        }
    }

    fn step_on_stack<K, A, Leaf, Switch>(stack: &mut Stack,
                                         graph: &Graph<Leaf, Switch>,
                                         knowledge: K)
                                         -> Result<StepResolution<A>>
        where K: Copy,
              Leaf: LeafFn<K, A>,
              Switch: SwitchFn<K>
    {
        if let Some(frame) = stack.last_mut() {

            let yield_state = YieldState::new(frame);

            match try!(graph.resolve_frame(frame, knowledge)) {
                Resolution::StackSwitch(child_stack) => {
                    let resolution = try!(Self::run_on_stack(child_stack, graph, knowledge));
                    Ok(resolution.to_step_resolution())
                }
                Resolution::Yield(action) => {
                    Ok(StepResolution::Yield {
                        action: action,
                        state: yield_state,
                    })
                }
                Resolution::Call(index) => Ok(StepResolution::Call(index)),
                Resolution::Return(value) => Ok(StepResolution::Return(value)),
            }
        } else {
            Err(Error::StackEmpty)
        }
    }
}
