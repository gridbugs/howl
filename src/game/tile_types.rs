#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    Van,
    Zombie,
    Wreck0,
    Wreck1,
    Wreck2,
    Bullet,
    Road0,
    Road1,
    Dirt0,
    Dirt1,
    Acid0,
    Acid1,
}

impl TileType {
    pub fn opaque_bg(self) -> bool {
        match self {
            TileType::Road0 |
            TileType::Road1 |
            TileType::Dirt0 |
            TileType::Dirt1 |
            TileType::Acid0 |
            TileType::Acid1 => true,
            _ => false,
        }
    }

    pub fn has_front_variant(self) -> bool { false }

    pub fn from_str(s: &str) -> Option<Self> {
        let tile = match s {
            "Van" => TileType::Van,
            "Zombie" => TileType::Zombie,
            "Wreck0" => TileType::Wreck0,
            "Wreck1" => TileType::Wreck1,
            "Wreck2" => TileType::Wreck2,
            "Bullet" => TileType::Bullet,
            "Road0" => TileType::Road0,
            "Road1" => TileType::Road1,
            "Dirt0" => TileType::Dirt0,
            "Dirt1" => TileType::Dirt1,
            "Acid0" => TileType::Acid0,
            "Acid1" => TileType::Acid1,
            _ => return None,
        };

        Some(tile)
    }
}
