use game::{
    Entity,
    EntityId,
    EntityTable,
    SpacialHashCell,
};
use game::vision::DefaultVisibilityReport;

use grid::StaticGrid;

use std::collections::HashMap;

#[derive(Debug)]
struct KnowledgeCell {
    // TODO
}

impl Default for KnowledgeCell {
    fn default() -> Self {
        KnowledgeCell {}
    }
}

impl KnowledgeCell {
    fn clear(&mut self) {
        // TODO
    }

    fn update(&mut self, _: &Entity, _: f64) {
        // TODO
    }
}

#[derive(Debug)]
struct KnowledgeGrid {
    grid: StaticGrid<KnowledgeCell>,
}

impl KnowledgeGrid {
    fn new(width: usize, height: usize) -> Self {
        KnowledgeGrid {
            grid: StaticGrid::new_default(width, height),
        }
    }

    fn update(&mut self, entities: &EntityTable,
              grid: &StaticGrid<SpacialHashCell>,
              report: &DefaultVisibilityReport)
    {
        for (coord, meta) in report.iter() {
            let sh_cell = &grid[coord];
            let mut kn_cell = &mut self.grid[coord];
            kn_cell.clear();
            for entity in entities.id_set_iter(&sh_cell.entities) {
                kn_cell.update(entity, *meta);
            }
        }
    }
}

#[derive(Debug)]
pub struct DefaultKnowledge {
    levels: HashMap<EntityId, KnowledgeGrid>,
}

impl DefaultKnowledge {
    pub fn new() -> Self {
        DefaultKnowledge {
            levels: HashMap::new(),
        }
    }

    pub fn update(&mut self, level_id: EntityId,
                  entities: &EntityTable,
                  grid: &StaticGrid<SpacialHashCell>,
                  report: &DefaultVisibilityReport)
    {
        if !self.levels.contains_key(&level_id) {
            self.levels.insert(level_id, KnowledgeGrid::new(grid.width, grid.height));
        }

        self.levels.get_mut(&level_id).unwrap().update(entities, grid, report);
    }
}

impl Clone for DefaultKnowledge {
    fn clone(&self) -> Self {
        panic!("Tried to clone knowledge.")
    }
}
