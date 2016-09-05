use game::knowledge::{
    KnowledgeCell,
    LevelGridKnowledge,
};
use game::{
    Entity,
    ComponentType as CType,
    EntityWrapper,
    EntityRef,
    IterEntityRef,
};

use best::BestMap;
use renderer::ComplexTile;
use object_pool::ObjectPool;
use grid::StaticGrid;
use table::TableRefMut;

use std::collections::HashSet;

pub type DrawableKnowledge = LevelGridKnowledge<StaticGrid<DrawableCell>>;

#[derive(Debug)]
pub struct DrawableCell {
    pub component_types: HashSet<CType>,
    pub foreground: BestMap<isize, ComplexTile>,
    pub background: BestMap<isize, ComplexTile>,
    pub moonlight: bool,
    pub last_turn: u64,
    memory_pool: ObjectPool<Entity>,
}

impl Default for DrawableCell {
    fn default() -> Self {
        DrawableCell {
            component_types: HashSet::new(),
            foreground: BestMap::new(),
            background: BestMap::new(),
            moonlight: false,
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
        self.moonlight = false;
    }

    fn update<'a, E: EntityRef<'a> + IterEntityRef<'a>>(
        &mut self,
        entity: E,
        turn_count: u64,
        _: &Self::MetaData)
    {
        // update set of component types
        for component_type in entity.types() {
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
                if tile.opaque_bg() {
                    self.background.insert(depth, tile);
                }
            });
        });

        // update moonlight
        self.moonlight |= entity.has_moon();

        self.last_turn = turn_count;
    }

    fn last_updated(&self) -> u64 { self.last_turn }
    fn set_last_updated(&mut self, last_updated: u64) {
        self.last_turn = last_updated;
    }
}

impl DrawableCell {
    fn update_memory<'a, E: EntityRef<'a>>(memory: &mut Entity, entity: E) {
        entity.get(CType::Solid).map(|c| memory.add(c.clone()));
    }
}
