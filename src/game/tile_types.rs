#[derive(Clone, Copy)]
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
}
