use game::knowledge::{
    KnowledgeCellCommon,
    LevelGridKnowledge,
};

use game::{
    Entity,
    ComponentType as CType,
};

use object_pool::ObjectPool;
use grid::StaticGrid;

use std::collections::HashSet;

pub type SimpleNpcCell =
    KnowledgeCellCommon<SimpleNpcExtra>;

pub type SimpleNpcKnowledge =
    LevelGridKnowledge<StaticGrid<SimpleNpcCell>>;

#[derive(Debug)]
pub struct SimpleNpcExtra {
    pub component_types: HashSet<CType>,
    memory_pool: ObjectPool<Entity>,
}
