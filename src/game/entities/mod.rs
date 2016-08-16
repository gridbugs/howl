use game;
use game::entity::{Entity, EntityId};
use game::entity::Component::*;
use game::components::{
    Level,
    DoorState,
};
use game::knowledge;

use geometry::Vector2;
use renderer::tile::Tile;
use colour::ansi;

use std::cell::RefCell;

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
        Opacity(1.0),
    ]
}

pub fn make_door(x: isize, y: isize, level: EntityId, state: DoorState) -> Entity {
    let mut entity = entity![
        Position(Vector2::new(x, y)),
        TileDepth(1),
        OnLevel(level),
    ];

    if state == DoorState::Open {
        entity.add(TransparentTile(Tile::new('-', ansi::WHITE)));
        entity.add(Door(DoorState::Open));
        entity.add(Opacity(0.0));
    } else {
        entity.add(Solid);
        entity.add(SolidTile {
            tile: Tile::new('+', ansi::WHITE),
            background: ansi::DARK_GREY,
        });
        entity.add(Door(DoorState::Closed));
        entity.add(Opacity(1.0));
    }

    entity
}

pub fn make_tree(x: isize, y: isize, level: EntityId) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Solid,
        TransparentTile(Tile::new('&', ansi::GREEN)),
        TileDepth(1),
        OnLevel(level),
        Opacity(0.4),
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
        VisionDistance(4),
        DefaultKnowledge(RefCell::new(knowledge::DefaultKnowledge::new())),
    ]
}

pub fn make_level(width: usize, height: usize) -> Entity {
    entity![
        LevelData(Level::new(width, height))
    ]
}
