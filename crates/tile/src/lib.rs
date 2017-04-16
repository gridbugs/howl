#[macro_use]
extern crate serde_derive;
extern crate serde;

pub const NUM_TILE_CHANNELS: usize = 3;

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    StoneFloor,
    Wall,
    Player,
    Rain,
}

impl TileType {
    pub fn opaque_bg(self) -> bool {
        match self {
            TileType::StoneFloor |
            TileType::Wall => true,
            _ => false,
        }
    }

    pub fn has_front_variant(self) -> bool {
        self == TileType::Wall
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let tile = match s {
            "StoneFloor" => TileType::StoneFloor,
            "Wall" => TileType::Wall,
            "Player" => TileType::Player,
            "Rain" => TileType::Rain,
            _ => return None,
        };

        Some(tile)
    }
}
