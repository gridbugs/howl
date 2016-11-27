use game::Level;
use util::BidirectionalList;

pub struct LevelTable {
    levels: BidirectionalList<Level>,
}

impl LevelTable {
    pub fn new() -> Self {
        LevelTable {
            levels: BidirectionalList::new(),
        }
    }

    pub fn level(&self, level_id: isize) -> &Level {
        self.levels.get_with_default(level_id)
    }

    pub fn level_mut(&mut self, level_id: isize) -> &mut Level {
        self.levels.get_mut_with_default(level_id)
    }
}
