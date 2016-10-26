use std::result;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    UnknownNodeIndex,
    NodeStateMismatch,
    UnexpectedNodeType,
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

enum Resolution<'a, A> {
    Return(bool),
    Call(NodeIndex),
    Yield(A),
    StackSwitch(&'a mut Stack),
}

pub trait LeafFnBox<K, A> {
    fn call(&self, knowledge: K) -> LeafResolution<A>;
}

pub trait SwitchFnBox<K> {
    fn call(&self, knowledge: K) -> SwitchResolution;
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
    Leaf(NodeIndex),
    Collection {
        index: NodeIndex,
        state: CollectionState,
    },
    Switch {
        index: NodeIndex,
        state: SwitchState,
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
    yielding: bool,
}

impl Stack {
    fn new() -> Self {
        Stack { frames: Vec::new(), initialised: false }
    }

    fn push(&mut self, frame: StackFrame) {
        self.frames.push(frame);
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
            &Node::Leaf(_) => Ok(StackFrame::Leaf(index)),
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
                })
            }
        }
    }

    fn resolve_switch<'a>(&self, state: &'a mut SwitchState, resolution: SwitchResolution) -> Result<&'a mut Stack> {
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

    fn resolve_frame<'a, K, A>(&self, frame: &'a mut StackFrame, knowledge: K) -> Result<Resolution<'a, A>>
        where Leaf: LeafFnBox<K, A>,
              Switch: SwitchFnBox<K>,
    {
        match frame {
            &mut StackFrame::Leaf(index) => {
                let node = try!(self.node(index));
                let f = try!(node.leaf_fn());
                Ok(f.call(knowledge).to_resolution())
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
            &mut StackFrame::Switch { index, ref mut state } => {
                let node = try!(self.node(index));
                let f = try!(node.switch_fn());
                let switch_resolution = f.call(knowledge);
                let stack = try!(self.resolve_switch(state, switch_resolution));
                Ok(Resolution::StackSwitch(stack))
            }
        }
    }
}
