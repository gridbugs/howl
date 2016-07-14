use ecs;
use ecs::entity::Entity;
use ecs::entity::Component::*;
use geometry::vector2::Vector2;
use renderer::tile::Tile;
use colour::ansi;
use game;

pub fn make_wall(pos: Vector2<isize>) -> Entity {
    entity![
        Position(pos),
        Solid,
        SolidTile {
            tile: Tile::new('#', ansi::GREY),
            background: ansi::RED
        },
        TileDepth(1)
    ]
}

pub fn make_floor(pos: Vector2<isize>) -> Entity {
    entity![
        Position(pos),
        SolidTile {
            tile: Tile::new('.', ansi::DARK_GREY),
            background: ansi::GREEN
        },
        TileDepth(0)
    ]
}

pub fn make_pc(pos: Vector2<isize>) -> Entity {
    entity![
        Position(pos),
        TransparentTile(Tile::new('@', ansi::WHITE)),
        TileDepth(2)
    ]
}

pub fn make_level(width: usize, height: usize) -> Entity {
    entity![
        Level(game::level::Level::new(width, height))
    ]
}
