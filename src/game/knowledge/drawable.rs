use game::knowledge::{
    KnowledgeCellExtra,
    KnowledgeCellCommon,
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
use tile::ComplexTile;
use object_pool::ObjectPool;
use grid::StaticGrid;
use table::TableRefMut;

pub type DrawableCell =
    KnowledgeCellCommon<DrawableExtra>;

pub type DrawableKnowledge =
    LevelGridKnowledge<StaticGrid<DrawableCell>>;

#[derive(Debug)]
pub struct DrawableExtra {
    pub foreground: BestMap<isize, ComplexTile>,
    pub background: BestMap<isize, ComplexTile>,
    pub moonlight: bool,
    memory_pool: ObjectPool<Entity>,
}

impl Default for DrawableExtra {
    fn default() -> Self {
        DrawableExtra {
            foreground: BestMap::new(),
            background: BestMap::new(),
            moonlight: false,
            memory_pool: ObjectPool::new(),
        }
    }
}

impl KnowledgeCellExtra for DrawableExtra {

    // visibility of cell (from 0.0 to 1.0)
    type MetaData = f64;

    fn clear(&mut self) {
        self.memory_pool.clear();
        self.foreground.clear();
        self.background.clear();
        self.moonlight = false;
    }

    fn update<'a, E: IterEntityRef<'a>>(
        &mut self,
        entity: E,
        _: &Self::MetaData)
    {
        // add entity memory containing clones of components
        {
            let mut memory = self.memory_pool.alloc();
            update_memory(memory, entity);
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
    }
}

fn update_memory<'a, E: EntityRef<'a>>(memory: &mut Entity, entity: E) {
    entity.get(CType::Solid).map(|c| memory.add(c.clone()));
}
