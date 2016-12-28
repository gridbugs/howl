use game::Level;

pub type LevelId = usize;

pub struct LevelTable {
    levels: Vec<Level>,
}

impl LevelTable {
    pub fn new() -> Self {
        LevelTable {
            levels: Vec::new(),
        }
    }

    pub fn add_level(&mut self, level: Level) -> LevelId {
        let index = self.levels.len();
        self.levels.push(level);
        index
    }

    pub fn level(&self, level_id: LevelId) -> &Level {
        self.levels.get(level_id).expect("No such level")
    }

    pub fn level_mut(&mut self, level_id: LevelId) -> &mut Level {
        self.levels.get_mut(level_id).expect("No such level")
    }
}
