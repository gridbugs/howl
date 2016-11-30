use math::Coord;
use game::{SpatialHashCell, Turn};
use util::BidirectionalList;

/// Trait implemented by representations of knowledge about a level
pub trait LevelKnowledge {
    /// Updates a cell of the knowledge representation, returnig true iff the
    /// knowledge of the cell changed as a result of the update.
    fn update_cell(&mut self, coord: Coord, world_cell: &SpatialHashCell, accuracy: f64, turn: Turn) -> bool;
}

pub struct GameKnowledge<K: LevelKnowledge + Default> {
    levels: BidirectionalList<K>,
}

impl<K: LevelKnowledge + Default> GameKnowledge<K> {
    pub fn new() -> Self {
        GameKnowledge {
            levels: BidirectionalList::new(),
        }
    }

    pub fn level(&self, level_id: isize) -> &K {
        self.levels.get_with_default(level_id)
    }

    pub fn level_mut(&mut self, level_id: isize) -> &mut K {
        self.levels.get_mut_with_default(level_id)
    }
}

impl<K: LevelKnowledge + Default> Default for GameKnowledge<K> {
    fn default() -> Self {
        Self::new()
    }
}
