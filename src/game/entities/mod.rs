use game;
use game::entity::{Entity, EntityId};
use game::entity::Component::*;
use game::components;
use geometry::vector2::Vector2;
use renderer::tile::Tile;
use colour::ansi;

pub fn make_wall(x: isize, y: isize, level: EntityId) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Solid,
        SolidTile {
            tile: Tile::new('#', ansi::WHITE),
            background: ansi::DARK_GREY
        },
        TileDepth(1),
        OnLevel(level),
    ]
}

pub fn make_tree(x: isize, y: isize, level: EntityId) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Solid,
        TransparentTile(Tile::new('&', ansi::GREEN)),
        TileDepth(1),
        OnLevel(level),
    ]
}

pub fn make_floor(x: isize, y: isize, level: EntityId) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        SolidTile {
            tile: Tile::new('.', ansi::WHITE),
            background: ansi::DARK_GREY
        },
        TileDepth(0),
        OnLevel(level),
    ]
}

pub fn make_pc(x: isize, y: isize, level: EntityId) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        TransparentTile(Tile::new('@', ansi::WHITE)),
        TileDepth(2),
        PlayerActor,
        OnLevel(level),
        Collider,
    ]
}

pub fn make_level(width: usize, height: usize) -> Entity {
    entity![
        LevelData(components::level::Level::new(width, height))
    ]
}
