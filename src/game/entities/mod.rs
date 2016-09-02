use game;
use game::{
    Entity,
    EntityId,
    StatusCounter,
    Level,
};
use game::entity::Component::*;
use game::components::{
    DoorState,
    Moonlight,
    Form,
};
use game::knowledge;

use geometry::Vector2;
use renderer::{
    tile,
    ComplexTile,
    SimpleTile,
};
use colour::ansi;
use terminal::style;

use std::cell::RefCell;

pub fn make_wall(x: isize, y: isize, level: EntityId) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Solid,
        Tile(ComplexTile::Wall {
            front: SimpleTile::Full { ch: '▄', fg: ansi::YELLOW, bg: ansi::GREY, style: style::NONE },
            back: SimpleTile::Foreground('█', ansi::GREY, style::NONE),
        }),
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
        entity.add(Tile(tile::foreground('-', ansi::WHITE, style::NONE)));
        entity.add(Door(DoorState::Open));
        entity.add(Opacity(0.0));
    } else {
        entity.add(Solid);
        entity.add(Tile(tile::full('+', ansi::WHITE, ansi::DARK_GREY, style::NONE)));
        entity.add(Door(DoorState::Closed));
        entity.add(Opacity(1.0));
    }

    entity
}

pub fn make_tree(x: isize, y: isize, level: EntityId) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Solid,
        Tile(tile::foreground('&', ansi::GREEN, style::NONE)),
        TileDepth(1),
        OnLevel(level),
        Opacity(0.4),
    ]
}

pub fn make_floor(x: isize, y: isize, level: EntityId) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Tile(tile::full('.', ansi::WHITE, ansi::DARK_GREY, style::NONE)),
        TileDepth(0),
        OnLevel(level),
    ]
}

pub fn make_floor_outside(x: isize, y: isize, level: EntityId, moonlight: Moonlight) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Tile(tile::full('.', ansi::WHITE, ansi::DARK_GREY, style::NONE)),
        TileDepth(0),
        OnLevel(level),
        MoonlightSlot(moonlight),
    ]
}

pub fn make_pc(x: isize, y: isize, level: EntityId) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Tile(tile::foreground('@', ansi::WHITE, style::BOLD)),
        TileDepth(2),
        PlayerActor,
        OnLevel(level),
        Collider,
        DoorOpener,
        VisionDistance(20),
        DrawableKnowledge(RefCell::new(knowledge::DrawableKnowledge::new())),
        FormSlot(Form::Human),
        BeastTransform(StatusCounter::new_max(6)),
    ]
}

pub fn make_bullet(x: isize, y: isize, level: EntityId) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Tile(tile::foreground('*', ansi::RED, style::NONE)),
        TileDepth(2),
        OnLevel(level),
        Collider,
        Bullet,
        DestroyOnCollision,
    ]
}

pub fn make_level(width: usize, height: usize) -> Entity {
    entity![
        LevelData(Box::new(Level::new(width, height)))
    ]
}
