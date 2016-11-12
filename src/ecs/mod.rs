mod ecs;
pub use self::ecs::{EcsTable, EcsCtx, EcsMap, EcsSet, EntityId, ComponentType, component_type, ComponentTypeSet,
                    EntityRef, EntityRefMut};

#[cfg(test)]
mod tests;
