use ecs;
use ecs::entity::Entity;
use ecs::entity::Component::*;
use ecs::components;
use geometry::vector2::Vector2;
use renderer::tile::Tile;
use colour::ansi;

pub fn make_wall(x: isize, y: isize) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Solid,
        SolidTile {
            tile: Tile::new('#', ansi::WHITE),
            background: ansi::DARK_GREY
        },
        TileDepth(1)
    ]
}

pub fn make_floor(x: isize, y: isize) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        SolidTile {
            tile: Tile::new('.', ansi::WHITE),
            background: ansi::DARK_GREY
        },
        TileDepth(0)
    ]
}

pub fn make_pc(x: isize, y: isize) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        TransparentTile(Tile::new('@', ansi::WHITE)),
        TileDepth(2),
        PlayerActor,
    ]
}

pub fn make_level(width: usize, height: usize) -> Entity {
    entity![
        Level(components::level::Level::new(width, height))
    ]
}
