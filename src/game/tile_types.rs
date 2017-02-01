#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Tree,
    DeadTree,
    Floor,
    Ground,
    OpenDoor,
    ClosedDoor,
    Bullet,
    Player,
    TerrorPillar,
    TerrorFly,
    Book,
}

impl TileType {
    pub fn opaque_bg(self) -> bool {
        match self {
            TileType::Wall
                | TileType::Floor
                | TileType::Ground
                | TileType::ClosedDoor
                => true,
            _ => false,
        }
    }

    pub fn has_front_variant(self) -> bool {
        match self {
            TileType::Wall => true,
            _ => false,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let tile = match s {
            "Wall" => TileType::Wall,
            "Tree" => TileType::Tree,
            "DeadTree" => TileType::DeadTree,
            "Floor" => TileType::Floor,
            "Ground" => TileType::Ground,
            "OpenDoor" => TileType::OpenDoor,
            "ClosedDoor" => TileType::ClosedDoor,
            "Bullet" => TileType::Bullet,
            "Player" => TileType::Player,
            "TerrorPillar" => TileType::TerrorPillar,
            "TerrorFly" => TileType::TerrorFly,
            "Book" => TileType::Book,
            _ => return None,
        };

        Some(tile)
    }
}
