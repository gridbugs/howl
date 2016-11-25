use std::result;

#[derive(Debug)]
pub enum Error {
    UnknownNodeIndex,
    NodeStateMismatch,
    UnexpectedNodeType,
    StackEmpty,
    StackNotEmpty,
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

pub enum SwitchReturn {
    Return(bool),
    Select(NodeIndex),
}

enum StepResolution<A> {
    Yield(A),
    Call(NodeIndex),
    Return(bool),
}

enum RunResolution<A> {
    Yield(A),
    Return(bool),
}

enum Resolution<A> {
    Return(bool),
    Call(NodeIndex),
    Yield(A),
}

pub trait LeafFn<K, A> {
    fn call(&self, knowledge: K) -> LeafResolution<A>;
}

pub trait SwitchFn<K> {
    fn call(&self, knowledge: K) -> SwitchResolution;
    fn return_to(&self, value: bool) -> SwitchReturn;
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

#[derive(Clone)]
struct ArrayTraverse {
    index: usize,
    length: usize,
    value: bool,
}

#[derive(Clone)]
enum CollectionState {
    Forever,
    All(ArrayTraverse),
}

#[derive(Clone)]
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
        child: Option<NodeIndex>,
        value: Option<bool>,
    },
}

struct StackTruncate {
    len: usize,
    index: NodeIndex,
}

pub struct Graph<Leaf, Switch> {
    nodes: Vec<Node<Leaf, Switch>>,
}

#[derive(Clone)]
pub struct State {
    stack: Vec<StackFrame>,
    yielding: bool,
}

impl StackTruncate {
    fn new(len: usize, index: NodeIndex) -> Self {
        StackTruncate {
            len: len,
            index: index,
        }
    }
}

impl SwitchResolution {
    fn to_index(&self) -> NodeIndex {
        match self {
            &SwitchResolution::Select(index) => index,
            &SwitchResolution::Reset(index) => index,
        }
    }
}

impl<A> RunResolution<A> {
    fn to_step_resolution(self) -> StepResolution<A> {
        match self {
            RunResolution::Return(value) => StepResolution::Return(value),
            RunResolution::Yield(action) => StepResolution::Yield(action),
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

impl<A> LeafResolution<A> {
    fn to_resolution<'a>(self) -> Resolution<A> {
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
                    child: None,
                    value: None,
                })
            }
        }
    }

    fn resolve_frame<K, A>(&self, frame: &mut StackFrame, knowledge: K) -> Result<Resolution<A>>
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
            &mut StackFrame::Switch { index, ref mut child, value } => {
                let switch_fn = try!(try!(self.node(index)).switch_fn());
                Ok(if let Some(value) = value {
                    match switch_fn.return_to(value) {
                        SwitchReturn::Return(return_value) => Resolution::Return(return_value),
                        SwitchReturn::Select(index) => Resolution::Call(index),
                    }
                } else {
                    let index = switch_fn.call(knowledge).to_index();
                    *child = Some(index);
                    Resolution::Call(index)
                })
            }
        }
    }
}
impl State {
    pub fn new() -> Self {
        State {
            stack: Vec::new(),
            yielding: false,
        }
    }

    pub fn is_initialised(&self) -> bool {
        !self.stack.is_empty()
    }

    pub fn initialise<Leaf, Switch>(&mut self,
                                    graph: &Graph<Leaf, Switch>,
                                    index: NodeIndex)
                                    -> Result<()> {

        if !self.stack.is_empty() {
            return Err(Error::StackNotEmpty);
        }

        let frame = try!(graph.create_stack_frame(index));
        self.stack.push(frame);

        Ok(())
    }

    fn current_frame(&mut self) -> Result<&mut StackFrame> {
        if let Some(frame) = self.stack.last_mut() {
            Ok(frame)
        } else {
            Err(Error::StackEmpty)
        }
    }

    fn apply_resolution<K, A, Leaf, Switch>(stack: &mut Vec<StackFrame>,
                                            graph: &Graph<Leaf, Switch>,
                                            resolution: Resolution<A>)
                                            -> Result<Option<A>>
        where Leaf: LeafFn<K, A>
    {
        match resolution {
            Resolution::Call(index) => {
                stack.push(try!(graph.create_stack_frame(index)));
            }
            Resolution::Return(value) => {
                if stack.pop().is_none() {
                    return Err(Error::StackEmpty);
                }

                if let Some(frame) = stack.last_mut() {
                    try!(frame.handle_return(value));
                }
            }
            Resolution::Yield(action) => return Ok(Some(action)),
        }

        Ok(None)
    }

    fn resolve_switches<K, Leaf, Switch>(stack: &mut Vec<StackFrame>,
                                         graph: &Graph<Leaf, Switch>,
                                         knowledge: K)
                                         -> Result<Option<StackTruncate>>
        where K: Copy,
              Switch: SwitchFn<K>
    {
        let mut i = 0;
        for frame in stack {
            if let &mut StackFrame::Switch { index, ref mut child, .. } = frame {
                if let Some(current) = child.as_mut() {
                    let switch = try!(try!(graph.node(index)).switch_fn());
                    let resolution = switch.call(knowledge);
                    match resolution {
                        SwitchResolution::Select(child_index) => {
                            if child_index != *current {
                                *current = child_index;
                                return Ok(Some(StackTruncate::new(i + 1, child_index)));
                            }
                        }
                        SwitchResolution::Reset(child_index) => {
                            *current = child_index;
                            return Ok(Some(StackTruncate::new(i + 1, child_index)));
                        }
                    }
                }
            }

            i += 1;
        }

        Ok(None)
    }

    pub fn run<K, A, Leaf, Switch>(&mut self,
                                   graph: &Graph<Leaf, Switch>,
                                   knowledge: K)
                                   -> Result<A>
        where K: Copy,
              Leaf: LeafFn<K, A>,
              Switch: SwitchFn<K>
    {

        if self.yielding {
            return Err(Error::Yielding);
        }

        if let Some(truncate) = try!(Self::resolve_switches(&mut self.stack, graph, knowledge)) {
            let frame = try!(graph.create_stack_frame(truncate.index));
            self.stack.truncate(truncate.len);
            self.stack.push(frame);
        }

        loop {
            let resolution = {
                let frame = try!(self.current_frame());
                try!(graph.resolve_frame(frame, knowledge))
            };

            if let Some(action) = try!(Self::apply_resolution(&mut self.stack, graph, resolution)) {
                self.yielding = true;
                return Ok(action);
            }
        }
    }

    pub fn declare_return(&mut self, value: bool) -> Result<()> {
        if !self.yielding {
            return Err(Error::NotYielding);
        }

        self.yielding = false;

        if let Some(frame) = self.stack.last_mut() {
            frame.handle_return(value)
        } else {
            Err(Error::StackEmpty)
        }
    }
}
