mod generated;

// general ecs types/modules
pub use self::generated::{EntityId, EcsTable, EcsCtx, EcsAction, EcsActionProperties,
                          ComponentTypeSet, ComponentTypeSetIter, ActionPropertyType,
                          ActionPropertyTypeSetIter, EntityRef, EntityRefMut, component_type,
                          action_property_type};

#[cfg(test)]
mod tests;
