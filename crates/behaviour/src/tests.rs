use behaviour::*;

struct Leaf(Box<Fn(&isize) -> LeafResolution<&'static str>>);
struct Switch {
    call: Box<Fn(&isize) -> SwitchResolution>,
    return_to: Box<Fn(bool) -> SwitchReturn>,
}

impl<'a> LeafFn<isize, &'static str> for Leaf {
    fn call(&self, knowledge: &mut isize) -> LeafResolution<&'static str> {
        (self.0)(knowledge)
    }
}

impl<'a> SwitchFn<isize> for Switch {
    fn call(&self, knowledge: &mut isize) -> SwitchResolution {
        (self.call)(knowledge)
    }

    fn return_to(&self, value: bool) -> SwitchReturn {
        (self.return_to)(value)
    }
}

type TestGraph = Graph<Leaf, Switch>;

fn action(s: &'static str) -> Leaf {
    Leaf(Box::new(move |_| LeafResolution::Yield(s)))
}

fn hello_world(graph: &mut TestGraph) -> NodeIndex {
    let hello = graph.add_leaf(action("hello"));
    let world = graph.add_leaf(action("world"));

    let all = graph.add_collection(CollectionNode::All(vec![hello, world]));

    graph.add_collection(CollectionNode::Forever(all))
}

fn one_two_three(graph: &mut TestGraph) -> NodeIndex {
    let one = graph.add_leaf(action("one"));
    let two = graph.add_leaf(action("two"));
    let three = graph.add_leaf(action("three"));

    let all = graph.add_collection(CollectionNode::All(vec![one, two, three]));

    graph.add_collection(CollectionNode::Forever(all))
}

#[test]
fn forever() {
    let mut graph = Graph::new();
    let root = hello_world(&mut graph);
    let mut state = State::new();
    state.initialise(&graph, root).unwrap();

    assert_eq!(state.run(&graph, &mut 0).unwrap(), "hello");
    state.declare_return(true).unwrap();
    assert_eq!(state.run(&graph, &mut 0).unwrap(), "world");
    state.declare_return(true).unwrap();
    assert_eq!(state.run(&graph, &mut 0).unwrap(), "hello");
    state.declare_return(true).unwrap();
}

#[test]
fn switch_select() {
    let mut graph = Graph::new();
    let hello_world = hello_world(&mut graph);
    let one_two_three = one_two_three(&mut graph);

    let switch = Switch {
        call: Box::new(move |k| {
            if k % 2 == 0 {
                SwitchResolution::Select(hello_world)
            } else {
                SwitchResolution::Select(one_two_three)
            }
        }),
        return_to: Box::new(|x| SwitchReturn::Return(x)),
    };
    let switch = graph.add_switch(switch);

    let mut state = State::new();
    state.initialise(&graph, switch).unwrap();

    assert_eq!(state.run(&graph, &mut 0).unwrap(), "hello");
    state.declare_return(true).unwrap();
    assert_eq!(state.run(&graph, &mut 0).unwrap(), "world");
    state.declare_return(true).unwrap();
    assert_eq!(state.run(&graph, &mut 1).unwrap(), "one");
    state.declare_return(true).unwrap();
    assert_eq!(state.run(&graph, &mut 1).unwrap(), "two");
    state.declare_return(true).unwrap();
    assert_eq!(state.run(&graph, &mut 0).unwrap(), "hello");
    state.declare_return(true).unwrap();
    assert_eq!(state.run(&graph, &mut 0).unwrap(), "world");
    state.declare_return(true).unwrap();
    assert_eq!(state.run(&graph, &mut 1).unwrap(), "one");
    state.declare_return(true).unwrap();
}
