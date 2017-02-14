use std::iter::IntoIterator;
use game::*;

pub type LevelId = usize;

pub struct LevelTable {
    levels: Vec<Level>,
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct SerializableLevelTable {
    levels: Vec<SerializableLevel>,
}

impl From<LevelTable> for SerializableLevelTable {
    fn from(orig_levels: LevelTable) -> Self {

        let mut serializable_levels = Vec::new();

        for level in orig_levels.levels.into_iter() {
            serializable_levels.push(SerializableLevel::from(level));
        }

        SerializableLevelTable {
            levels: serializable_levels,
        }
    }
}

impl From<SerializableLevelTable> for LevelTable {
    fn from(orig_levels: SerializableLevelTable) -> Self {

        let mut levels = Vec::new();

        for serialiable_level in orig_levels.levels.into_iter() {
            levels.push(Level::from(serialiable_level));
        }

        LevelTable {
            levels: levels,
        }
    }
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
