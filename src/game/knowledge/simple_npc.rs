use game::knowledge::{KnowledgeCell, LevelGridKnowledge, KnowledgeCellData};
use game::{ComponentType, EntityRef};

use search::TraverseCost;
use clear::Clear;

pub type SimpleNpcCell = KnowledgeCell<SimpleNpcExtra>;
pub type SimpleNpcKnowledge = LevelGridKnowledge<SimpleNpcExtra>;

impl TraverseCost for SimpleNpcCell {
    fn traverse_cost(&self) -> Option<f64> {
        if self.data().solid {
            None
        } else {
            Some(1.0)
        }
    }
}

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

impl Clear for SimpleNpcExtra {
    fn clear(&mut self) {
        self.solid = false;
        self.player = false;
    }
}

impl KnowledgeCellData for SimpleNpcExtra {
    type VisionMetadata = f64;

    fn update<'a, E: EntityRef<'a>>(&mut self, entity: E, _: &Self::VisionMetadata) {
        if entity.has(ComponentType::Solid) {
            self.solid = true;
        }
        if entity.has(ComponentType::PlayerCharacter) {
            self.player = true;
        }
    }
}
