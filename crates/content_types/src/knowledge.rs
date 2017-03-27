use std::collections::BTreeMap;
use dimension_constructor::TwoDimensionalCons;
use engine_defs::LevelId;

#[derive(Serialize, Deserialize)]
pub struct GameKnowledge<K> {
    levels: BTreeMap<LevelId, K>,
}

impl<K> GameKnowledge<K> {
    pub fn new() -> Self {
        GameKnowledge {
            levels: BTreeMap::new(),
        }
    }

    pub fn level(&self, level_id: LevelId) -> &K {
        self.levels.get(&level_id).expect("No such level")
    }

    pub fn level_mut(&mut self, level_id: LevelId) -> &mut K {
        self.levels.get_mut(&level_id).expect("No such level")
    }
}

impl<K: Default> Default for GameKnowledge<K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: TwoDimensionalCons> GameKnowledge<K> {
    pub fn level_mut_or_insert_size(&mut self, level_id: LevelId,
                                    width: usize, height: usize) -> &mut K {
        self.levels.entry(level_id).or_insert_with(|| K::new(width, height))
    }
}
