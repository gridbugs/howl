mod generated;
pub use self::generated::{EcsTable, EcsCtx, EntityMap, EntitySet, EntityId, ComponentType, component_type, ComponentTypeSet,
                          EntityRef, EntityRefMut};

#[cfg(test)]
mod tests;
