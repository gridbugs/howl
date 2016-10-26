#[cfg(test)]
mod tests;

mod behaviour;
pub use self::behaviour::{Graph, NodeIndex, State, LeafResolution, CheckResolution, LeafFnBox,
                          CheckFnBox, CollectionNode, Result, Error};

mod behaviour_graph;
