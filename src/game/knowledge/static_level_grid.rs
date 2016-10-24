use game::{LevelId, Level, SpatialHashCell, IterEntityRef, EntityStore};
use grid::{Grid, Coord, StaticGrid, DefaultGrid, IterGrid};
use clear::Clear;
use std::collections::HashMap;

pub trait KnowledgeCellData {
    type VisionMetadata;
    fn update<'a, E: IterEntityRef<'a>>(&mut self, entity: E, vision_meta: &Self::VisionMetadata);
}

pub struct KnowledgeCell<Data> {
    last_updated_turn: u64,
    data: Data,
}

pub struct KnowledgeGrid<Data> {
    grid: StaticGrid<KnowledgeCell<Data>>,
}

pub struct LevelGridKnowledge<Data> {
    levels: HashMap<LevelId, KnowledgeGrid<Data>>,
}

pub struct KnowledgeGridIter<'a, Data: 'a>(<StaticGrid<KnowledgeCell<Data>> as IterGrid<'a>>::Iter);

impl<'a, Data> Iterator for KnowledgeGridIter<'a, Data> {
    type Item = &'a Data;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|cell| &cell.data)
    }
}

impl<Data> KnowledgeCell<Data> {
    pub fn data(&self) -> &Data { &self.data }
    pub fn last_updated_turn(&self) -> u64 { self.last_updated_turn }
}

impl<Data: Default> Default for KnowledgeCell<Data> {
    fn default() -> Self {
        KnowledgeCell {
            last_updated_turn: 0,
            data: Default::default(),
        }
    }
}

impl<Data: KnowledgeCellData + Clear + Default> KnowledgeGrid<Data> {
    pub fn new(width: usize, height: usize) -> Self {
        KnowledgeGrid { grid: StaticGrid::new_default(width, height) }
    }

    pub fn inner(&self) -> &StaticGrid<KnowledgeCell<Data>> {
        &self.grid
    }

    pub fn iter(&self) -> KnowledgeGridIter<Data> {
        KnowledgeGridIter(self.grid.iter())
    }

    pub fn update<'a, SpatialHash, ReportIter>(&mut self,
                                               level: &Level,
                                               grid: &SpatialHash,
                                               iter: ReportIter,
                                               turn_count: u64)
                                               -> bool
        where Data::VisionMetadata: 'a,
              SpatialHash: Grid<Item = SpatialHashCell>,
              ReportIter: Iterator<Item = (&'a Coord, &'a Data::VisionMetadata)>
    {
        let mut changed = false;
        for (coord, meta) in iter {
            let sh_cell = grid.get_unsafe(*coord);
            let mut kn_cell = self.grid.get_mut_unsafe(*coord);

            // If the last update to the cell was before the last
            // time the cell was observed, we can skip updating
            // knowledge for that cell.

            if sh_cell.last_updated >= kn_cell.last_updated_turn {
                changed = true;
                kn_cell.data.clear();
                for (_, maybe_entity) in level.id_set_iter(&sh_cell.entities) {
                    let entity = maybe_entity.expect("invalid entity id");
                    kn_cell.data.update(entity, meta);
                }
            }
            kn_cell.last_updated_turn = turn_count;
        }

        changed
    }
}

impl<Data: KnowledgeCellData + Clear + Default> LevelGridKnowledge<Data> {
    pub fn new() -> Self {
        LevelGridKnowledge { levels: HashMap::new() }
    }

    pub fn grid(&self, level_id: LevelId) -> Option<&KnowledgeGrid<Data>> {
        self.levels.get(&level_id)
    }

    pub fn update<'a, SpatialHash, ReportIter>(&mut self,
                                               level: &Level,
                                               grid: &SpatialHash,
                                               iter: ReportIter,
                                               turn_count: u64)
                                               -> bool
        where Data::VisionMetadata: 'a,
              SpatialHash: Grid<Item = SpatialHashCell>,
              ReportIter: Iterator<Item = (&'a Coord, &'a Data::VisionMetadata)>
    {
        self.levels
            .entry(level.id())
            .or_insert_with(|| KnowledgeGrid::new(grid.width(), grid.height()))
            .update(level, grid, iter, turn_count)
    }
}

impl<Data> Clone for LevelGridKnowledge<Data> {
    fn clone(&self) -> Self { panic!() }
}
