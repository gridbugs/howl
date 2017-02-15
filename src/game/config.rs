use std::path;
use game::*;

#[derive(Debug, Deserialize)]
pub struct GameConfig {
    pub graphics: GraphicsConfig,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            graphics: GraphicsConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GraphicsConfig {
    pub scale: usize,
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        GraphicsConfig {
            scale: 1,
        }
    }
}

impl GameConfig {
    pub fn from_file<P: AsRef<path::Path>>(path: P) -> Option<Self> {
        game_file::read_toml(path).ok()
    }
}
