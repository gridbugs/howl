use game::knowledge::{KnowledgeCellCommon, LevelGridKnowledge, KnowledgeCellExtra};

use game::{ComponentType, IterEntityRef};

use grid::StaticGrid;

pub type SimpleNpcCell = KnowledgeCellCommon<SimpleNpcExtra>;

pub type SimpleNpcKnowledge = LevelGridKnowledge<StaticGrid<SimpleNpcCell>>;

#[derive(Debug)]
pub struct SimpleNpcExtra {
    pub solid: bool,
    pub player: bool,
}

impl Default for SimpleNpcExtra {
    fn default() -> Self {
        SimpleNpcExtra {
            solid: false,
            player: false,
        }
    }
}

impl KnowledgeCellExtra for SimpleNpcExtra {
    type MetaData = f64;

    fn clear(&mut self) {
        self.solid = false;
        self.player = false;
    }

    fn update<'a, E: IterEntityRef<'a>>(&mut self, entity: E, _: &Self::MetaData) {
        if entity.has(ComponentType::Solid) {
            self.solid = true;
        }
        if entity.has(ComponentType::PlayerCharacter) {
            self.player = true;
        }
    }
}
