use game::{
    Entity,
    EntityId,
    EntityTable,
    SpacialHashCell,
    ComponentType as CType,
};
use game::vision::DefaultVisibilityReport;

use grid::StaticGrid;
use best::BestMap;
use renderer::Tile;
use colour::ansi::AnsiColour;
use object_pool::ObjectPool;

use std::collections::{
    HashMap,
    HashSet,
};

#[derive(Debug)]
struct KnowledgeCell {
    component_types: HashSet<CType>,
    memory_pool: ObjectPool<Entity>,
    foreground: BestMap<isize, Tile>,
    background: BestMap<isize, AnsiColour>,
}

impl Default for KnowledgeCell {
    fn default() -> Self {
        KnowledgeCell {
            component_types: HashSet::new(),
            memory_pool: ObjectPool::new(),
            foreground: BestMap::new(),
            background: BestMap::new(),
        }
    }
}

impl KnowledgeCell {
    fn clear(&mut self) {
        self.component_types.clear();
        self.memory_pool.clear();
        self.foreground.clear();
        self.background.clear();
    }

    fn update(&mut self, entity: &Entity, _: f64) {
        // update set of component types
        for component_type in entity.slots.keys() {
            self.component_types.insert(*component_type);
        }

        // add entity memory containing clones of components
        {
            let mut memory = self.memory_pool.alloc();
            Self::update_memory(memory, entity);
        }

        // update tiles
        entity.tile_depth().map(|depth| {
            entity.tile().map(|tile| {
                self.foreground.insert(depth, tile);
            });
            entity.background().map(|background| {
                self.background.insert(depth, background);
            });
        });
    }

    fn update_memory(memory: &mut Entity, entity: &Entity) {
        entity.get(CType::Solid).map(|c| memory.add(c.clone()));
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
