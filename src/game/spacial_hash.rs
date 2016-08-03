use ecs::entity::{EntityId, ComponentType};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct SpacialHashMap {
    pub hash_map: HashMap<(isize, isize), SpacialHashCell>,
}

#[derive(Debug, Clone)]
pub struct SpacialHashCell {
    pub entities: HashSet<EntityId>,
    pub components: HashMap<ComponentType, usize>,
}

impl SpacialHashMap {
    pub fn new() -> Self {
        SpacialHashMap {
            hash_map: HashMap::new(),
        }
    }
}
