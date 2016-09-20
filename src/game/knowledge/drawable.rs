use game::knowledge::{KnowledgeCellExtra, KnowledgeCellCommon, LevelGridKnowledge};
use game::{EntityWrapper, EntityRef};

use best::BestMap;
use tile::ComplexTile;
use grid::StaticGrid;

pub type DrawableCell = KnowledgeCellCommon<DrawableExtra>;

pub type DrawableKnowledge = LevelGridKnowledge<StaticGrid<DrawableCell>>;

#[derive(Debug)]
pub struct DrawableExtra {
    pub foreground: BestMap<isize, ComplexTile>,
    pub background: BestMap<isize, ComplexTile>,
    pub moonlight: bool,
}

impl Default for DrawableExtra {
    fn default() -> Self {
        DrawableExtra {
            foreground: BestMap::new(),
            background: BestMap::new(),
            moonlight: false,
        }
    }
}

impl KnowledgeCellExtra for DrawableExtra {
    // visibility of cell (from 0.0 to 1.0)
    type MetaData = f64;

    fn clear(&mut self) {
        self.foreground.clear();
        self.background.clear();
        self.moonlight = false;
    }

    fn update<'a, E: EntityRef<'a>>(&mut self, entity: E, _: &Self::MetaData) {
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
