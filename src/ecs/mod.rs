mod ecs;
pub use self::ecs::{EcsTable, EcsCtx, EcsMap, EcsSet, EntityId, ComponentType, component_type, ComponentTypeSet};

#[cfg(test)]
mod tests;
