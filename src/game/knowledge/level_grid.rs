use game::{
    Entity,
    EntityId,
    EntityTable,
    SpacialHashCell,
};

use grid::StaticGrid;
use grid::Coord;

use std::collections::HashMap;

pub trait KnowledgeCell: Default {

    type MetaData;

    fn clear(&mut self);
    fn update(&mut self, entity: &Entity, turn_count: u64, meta: &Self::MetaData);
}

#[derive(Debug)]
struct KnowledgeGrid<C: KnowledgeCell>(StaticGrid<C>);

impl<C: KnowledgeCell> KnowledgeGrid<C> {
    fn new(width: usize, height: usize) -> Self {
        KnowledgeGrid(StaticGrid::new_default(width, height))
    }

    fn update<'a, I>(
        &mut self, entities: &EntityTable,
        grid: &StaticGrid<SpacialHashCell>,
        report_iter: I,
        turn_count: u64)
        where C::MetaData: 'a,
              I: Iterator<Item=(&'a Coord, &'a C::MetaData)>,
    {
        for (coord, meta) in report_iter {
            let sh_cell = &grid[coord];
            let mut kn_cell = &mut self.0[coord];
            kn_cell.clear();
            for entity in entities.id_set_iter(&sh_cell.entities) {
                kn_cell.update(entity, turn_count, meta);
            }
        }
    }
}

#[derive(Debug)]
pub struct LevelGridKnowledge<C: KnowledgeCell> {
    levels: HashMap<EntityId, KnowledgeGrid<C>>,
}

impl<C: KnowledgeCell> LevelGridKnowledge<C> {
    pub fn new() -> Self {
        LevelGridKnowledge {
            levels: HashMap::new(),
        }
    }

    pub fn update<'a, I>(
        &mut self, level_id: EntityId,
        entities: &EntityTable,
        grid: &StaticGrid<SpacialHashCell>,
        report_iter: I,
        turn_count: u64)
        where C::MetaData: 'a,
              I: Iterator<Item=(&'a Coord, &'a C::MetaData)>,
    {
        if !self.levels.contains_key(&level_id) {
            self.levels.insert(level_id, KnowledgeGrid::new(grid.width, grid.height));
        }

        self.levels.get_mut(&level_id).unwrap().update(entities, grid, report_iter, turn_count);
    }

    pub fn grid(&self, level_id: EntityId) -> Option<&StaticGrid<C>> {
        self.levels.get(&level_id).map(|g| &g.0)
    }
}

impl<C: KnowledgeCell> Clone for LevelGridKnowledge<C> {
    fn clone(&self) -> Self {
        panic!("Tried to clone knowledge.")
    }
}
