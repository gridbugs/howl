use ecs::entity::{EntityId, Entity};
use std::collections::HashMap;

#[derive(Debug)]
pub struct EcsContext {
    next_entity_id: EntityId,
    entities: HashMap<EntityId, Entity>,
}

impl EcsContext {
    pub fn new() -> EcsContext {
        EcsContext {
            next_entity_id: 0,
            entities: HashMap::new(),
        }
    }
}
