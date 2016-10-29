#[cfg(test)]
mod tests;

mod behaviour;
pub use self::behaviour::{Graph, NodeIndex, State, LeafResolution, SwitchResolution, SwitchReturn,
                          LeafFn, SwitchFn, CollectionNode, Result, Error};
