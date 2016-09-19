use game::{
    IterEntityRef,
    SpatialHashCell,
    LevelId,
    EntityStore,
    Level,
};

use grid::{
    Grid,
    DefaultGrid,
    Coord,
};

use std::collections::HashMap;

pub trait KnowledgeCell: Default  + KnowledgeCellExtra {
    fn last_updated_turn(&self) -> u64;
    fn set_last_updated_turn(&mut self, last_updated: u64);
}

pub trait KnowledgeCellExtra: Default {
    type MetaData;

    fn clear(&mut self);
    fn update<'a, E: IterEntityRef<'a>>(
        &mut self,
        entity: E,
        meta: &Self::MetaData);
}

#[derive(Debug)]
pub struct KnowledgeCellCommon<Extra: KnowledgeCellExtra> {
    extra: Extra,
    turn_count: u64,
}

impl<Extra: KnowledgeCellExtra> Default for KnowledgeCellCommon<Extra> {
    fn default() -> Self {
        KnowledgeCellCommon {
            turn_count: 0,
            extra: Extra::default(),
        }
    }
}

impl<Extra: KnowledgeCellExtra> KnowledgeCellExtra for KnowledgeCellCommon<Extra> {
    type MetaData = Extra::MetaData;

    fn clear(&mut self) {
        self.extra.clear();
    }

    fn update<'a, E: IterEntityRef<'a>>(
        &mut self,
        entity: E,
        meta: &Self::MetaData)
    {
        self.extra.update(entity, meta);
    }
}

impl<Extra: KnowledgeCellExtra> KnowledgeCell for KnowledgeCellCommon<Extra> {
    fn last_updated_turn(&self) -> u64 {
        self.turn_count
    }

    fn set_last_updated_turn(&mut self, last_updated: u64) {
        self.turn_count = last_updated;
    }
}

impl<Extra: KnowledgeCellExtra> KnowledgeCellCommon<Extra> {
    pub fn extra(&self) -> &Extra {
        &self.extra
    }
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
        &mut self, entities: &Level,
        grid: &S,
        report_iter: I,
        turn_count: u64) -> bool
        where <<G as Grid>::Item as KnowledgeCellExtra>::MetaData: 'a,
              I: Iterator<Item=(&'a Coord, &'a <<G as Grid>::Item as KnowledgeCellExtra>::MetaData)>,
              S: Grid<Item=SpatialHashCell>,
    {
        let mut changed = false;
        for (coord, meta) in report_iter {
            let sh_cell = &grid.get_unsafe(*coord);
            let mut kn_cell = &mut self.grid.get_mut_unsafe(*coord);

            // If the last update to the cell was before the last
            // time the cell was observed, we can skip updating
            // knowledge for that cell.

            if sh_cell.last_updated < kn_cell.last_updated_turn() {
                kn_cell.set_last_updated_turn(turn_count);
            } else {
                changed = true;
                kn_cell.clear();
                for (_, entity) in entities.id_set_iter(&sh_cell.entities) {
                    kn_cell.update(entity.unwrap(), meta);
                    kn_cell.set_last_updated_turn(turn_count);
                }
            }
        }

        changed
    }
}

#[derive(Debug)]
pub struct LevelGridKnowledge<G>
    where G: DefaultGrid,
          G::Item: KnowledgeCell
{
    levels: HashMap<LevelId, KnowledgeGrid<G>>,
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
        &mut self,
        entities: &Level,
        grid: &S,
        report_iter: I,
        turn_count: u64) -> bool
        where <<G as Grid>::Item as KnowledgeCellExtra>::MetaData: 'a,
              I: Iterator<Item=(&'a Coord, &'a <<G as Grid>::Item as KnowledgeCellExtra>::MetaData)>,
              S: Grid<Item=SpatialHashCell>,
    {
        let level_id = entities.id();
        if !self.levels.contains_key(&level_id) {
            self.levels.insert(level_id, KnowledgeGrid::new(grid.width(), grid.height()));
        }

        self.levels.get_mut(&level_id).unwrap().update(entities, grid, report_iter, turn_count)
    }

    pub fn grid(&self, level_id: LevelId) -> Option<&G> {
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
