use behaviour::behaviour::*;

struct Leaf(Box<Fn(isize) -> LeafResolution<&'static str>>);
struct Switch {
    call: Box<Fn(isize) -> SwitchResolution>,
    return_to: Box<Fn(bool) -> Option<bool>>,
}

impl<'a> LeafFn<isize, &'static str> for Leaf {
    fn call(&self, knowledge: isize) -> LeafResolution<&'static str> {
        (self.0)(knowledge)
    }
}

impl<'a> SwitchFn<isize> for Switch {
    fn call(&self, knowledge: isize) -> SwitchResolution {
        (self.call)(knowledge)
    }

    fn return_to(&self, value: bool) -> Option<bool> {
        (self.return_to)(value)
    }
}

type TestGraph = Graph<Leaf, Switch>;

fn action(s: &'static str) -> Leaf {
    Leaf(Box::new(move |_| LeafResolution::Yield(s)))
}

fn create_a() -> (TestGraph, NodeIndex) {
    let mut graph = Graph::new();

    let hello = graph.add_leaf(action("hello"));
    let world = graph.add_leaf(action("world"));

    let all = graph.add_collection(CollectionNode::All(vec![hello, world]));

    let forever = graph.add_collection(CollectionNode::Forever(all));

    (graph, forever)
}

#[test]
fn forever() {
    let (graph, root) = create_a();
    let mut state = State::new();
    state.initialise(&graph, root).unwrap();

    assert_eq!(state.run(&graph, 0).unwrap(), "hello");
    state.declare_return(true).unwrap();
    assert_eq!(state.run(&graph, 0).unwrap(), "world");
    state.declare_return(true).unwrap();
    assert_eq!(state.run(&graph, 0).unwrap(), "hello");
    state.declare_return(true).unwrap();
}
