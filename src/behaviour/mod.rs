#[cfg(test)]
mod tests;

mod behaviour;
pub use self::behaviour::{Graph, NodeIndex, State, LeafResolution, CheckResolution, LeafFn,
                          CheckFn, CollectionNode, Result, Error};
