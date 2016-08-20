use game::{
    Entity,
    EntityId,
    EntityTable,
    SpacialHashCell,
};

use grid::{
    Grid,
    DefaultGrid,
    Coord,
};

use std::collections::HashMap;

pub trait KnowledgeCell: Default {

    type MetaData;

    fn clear(&mut self);
    fn update(&mut self, entity: &Entity, turn_count: u64, meta: &Self::MetaData);
}

#[derive(Debug)]
struct KnowledgeGrid<G>
    where G: DefaultGrid,
          G::Item: KnowledgeCell
{
    grid: G,
}

impl<G> KnowledgeGrid<G>
    where G: DefaultGrid,
          G::Item: KnowledgeCell,
{
    fn new(width: usize, height: usize) -> Self {
        KnowledgeGrid {
            grid: G::new_default(width, height),
        }
    }

    fn update<'a, I, S>(
        &mut self, entities: &EntityTable,
        grid: &S,
        report_iter: I,
        turn_count: u64)
        where <<G as Grid>::Item as KnowledgeCell>::MetaData: 'a,
              I: Iterator<Item=(&'a Coord, &'a <<G as Grid>::Item as KnowledgeCell>::MetaData)>,
              S: Grid<Item=SpacialHashCell>,
    {
        for (coord, meta) in report_iter {
            let sh_cell = &grid.get(*coord).unwrap();
            let mut kn_cell = &mut self.grid.get_mut(*coord).unwrap();
            kn_cell.clear();
            for entity in entities.id_set_iter(&sh_cell.entities) {
                kn_cell.update(entity, turn_count, meta);
            }
        }
    }
}

#[derive(Debug)]
pub struct LevelGridKnowledge<G>
    where G: DefaultGrid,
          G::Item: KnowledgeCell
{
    levels: HashMap<EntityId, KnowledgeGrid<G>>,
}

impl<G> LevelGridKnowledge<G>
    where G: DefaultGrid,
          G::Item: KnowledgeCell
{
    pub fn new() -> Self {
        LevelGridKnowledge {
            levels: HashMap::new(),
        }
    }

    pub fn update<'a, I, S>(
        &mut self, level_id: EntityId,
        entities: &EntityTable,
        grid: &S,
        report_iter: I,
        turn_count: u64)
        where <<G as Grid>::Item as KnowledgeCell>::MetaData: 'a,
              I: Iterator<Item=(&'a Coord, &'a <<G as Grid>::Item as KnowledgeCell>::MetaData)>,
              S: Grid<Item=SpacialHashCell>,
    {
        if !self.levels.contains_key(&level_id) {
            self.levels.insert(level_id, KnowledgeGrid::new(grid.width(), grid.height()));
        }

        self.levels.get_mut(&level_id).unwrap().update(entities, grid, report_iter, turn_count);
    }

    pub fn grid(&self, level_id: EntityId) -> Option<&G> {
        self.levels.get(&level_id).map(|g| &g.grid)
    }
}

impl<G> Clone for LevelGridKnowledge<G>
    where G: DefaultGrid,
          G::Item: KnowledgeCell
{
    fn clone(&self) -> Self {
        panic!("Tried to clone knowledge.")
    }
}
