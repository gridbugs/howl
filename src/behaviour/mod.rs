#[cfg(test)]
mod tests;

mod old_behaviour;
pub use self::old_behaviour::{Graph, NodeIndex, State, LeafResolution, CheckResolution, LeafFnBox,
                              CheckFnBox, CollectionNode, Result, Error};

pub mod behaviour;
