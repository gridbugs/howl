use behaviour::*;
use behaviour::BehaviourNodeLeaf::*;
use behaviour::BehaviourNodeInternal::*;

type TestBehaviourGraph = BehaviourGraph<&'static str, &'static str>;
type TestBehaviourNodeLeaf = BehaviourNodeLeaf<&'static str, &'static str>;

fn action(s: &'static str) -> TestBehaviourNodeLeaf {
    Action(Box::new(move |_| s))
}

fn create_a() -> (TestBehaviourGraph, BehaviourNodeIndex) {
    let mut bg: TestBehaviourGraph = BehaviourGraph::new();

    let hello = bg.add_leaf(action("hello"));
    let world = bg.add_leaf(action("world"));

    let all = bg.add_internal(All(vec![hello, world]));
    let root = bg.add_internal(Forever(all));

    (bg, root)
}

#[test]
fn forever() {
    let (bg, root) = create_a();

    let mut state = BehaviourState::new();

    state.start(&bg, root).unwrap();

    assert_eq!(state.run(&bg, &"").unwrap(), "hello");
    state.give_outcome(BehaviourOutcome::Succeed).unwrap();
    assert_eq!(state.run(&bg, &"").unwrap(), "world");
    state.give_outcome(BehaviourOutcome::Succeed).unwrap();
    assert_eq!(state.run(&bg, &"").unwrap(), "hello");
    state.give_outcome(BehaviourOutcome::Succeed).unwrap();
}
