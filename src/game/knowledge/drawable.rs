use game::knowledge::{
    KnowledgeCell,
    LevelGridKnowledge,
};
use game::{
    Entity,
    ComponentType as CType,
};

use best::BestMap;
use renderer::Tile;
use colour::ansi::AnsiColour;
use object_pool::ObjectPool;

use std::collections::HashSet;

pub type DrawableKnowledge = LevelGridKnowledge<DrawableCell>;

#[derive(Debug)]
pub struct DrawableCell {
    pub component_types: HashSet<CType>,
    pub foreground: BestMap<isize, Tile>,
    pub background: BestMap<isize, AnsiColour>,
    pub last_turn: u64,
    memory_pool: ObjectPool<Entity>,
}

impl Default for DrawableCell {
    fn default() -> Self {
        DrawableCell {
            component_types: HashSet::new(),
            foreground: BestMap::new(),
            background: BestMap::new(),
            last_turn: 0,
            memory_pool: ObjectPool::new(),
        }
    }
}

impl KnowledgeCell for DrawableCell {

    type MetaData = f64;

    fn clear(&mut self) {
        self.component_types.clear();
        self.memory_pool.clear();
        self.foreground.clear();
        self.background.clear();
    }

    fn update(&mut self, entity: &Entity, turn_count: u64, _: &Self::MetaData) {
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

        self.last_turn = turn_count;
    }
}

impl DrawableCell {
    fn update_memory(memory: &mut Entity, entity: &Entity) {
        entity.get(CType::Solid).map(|c| memory.add(c.clone()));
    }
}
