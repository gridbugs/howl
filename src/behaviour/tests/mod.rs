use behaviour::*;

type TestGraph = Graph<isize, &'static str>;
type TestLeafFn = LeafFn<isize, &'static str>;
type TestCheckFn = CheckFn<isize>;

fn action(s: &'static str) -> TestLeafFn {
    Box::new(move |_| LeafResolution::Yield(s))
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

    assert_eq!(state.run_to_action(&graph, &0).unwrap(), "hello");
    state.report_action_result(true).unwrap();
    assert_eq!(state.run_to_action(&graph, &0).unwrap(), "world");
    state.report_action_result(true).unwrap();
    assert_eq!(state.run_to_action(&graph, &0).unwrap(), "hello");
    state.report_action_result(true).unwrap();
}
