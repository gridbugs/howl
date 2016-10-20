use game;
use game::{Entity, StatusCounter};
use game::Component::*;
use game::ActorType::*;
use game::components::{DoorState, Form};
use game::knowledge;
use game::actors::SimpleNpcAiState;
use game::behaviour;

use geometry::Vector2;
use tile;
use tile::{ComplexTile, SimpleTile};
use colour::ansi;
use terminal::style;
use table::TableRefMut;

use std::cell::RefCell;

pub fn make_wall(x: isize, y: isize) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Solid,
        Tile(ComplexTile::Wall {
            front: SimpleTile::Full { ch: '▄', fg: ansi::YELLOW, bg: ansi::GREY, style: style::NONE },
            back: SimpleTile::Foreground('█', ansi::GREY, style::NONE),
        }),
        TileDepth(1),
        Opacity(1.0),
    ]
}

pub fn make_door(x: isize, y: isize, state: DoorState) -> Entity {
    let mut entity = entity![
        Position(Vector2::new(x, y)),
        TileDepth(1),
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

pub fn make_tree(x: isize, y: isize) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Solid,
        Tile(tile::foreground('&', ansi::GREEN, style::NONE)),
        TileDepth(1),
        Opacity(0.4),
    ]
}

pub fn make_floor(x: isize, y: isize) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Tile(tile::full('.', ansi::WHITE, ansi::DARK_GREY, style::NONE)),
        TileDepth(0),
    ]
}

pub fn make_floor_outside(x: isize, y: isize, moonlight: bool) -> Entity {
    let mut entity = entity![
        Position(Vector2::new(x, y)),
        Tile(tile::full('.', ansi::WHITE, ansi::DARK_GREY, style::NONE)),
        TileDepth(0),
        Outside,
    ];

    if moonlight {
        entity.add(Moon);
    }

    entity
}

pub fn make_pc(x: isize, y: isize) -> Entity {
    entity![
        PlayerCharacter,
        Position(Vector2::new(x, y)),
        Tile(tile::foreground('@', ansi::WHITE, style::BOLD)),
        TileDepth(2),
        Actor(Player),
        Collider,
        DoorOpener,
        VisionDistance(20),
        DrawableKnowledge(RefCell::new(knowledge::DrawableKnowledge::new())),
        FormSlot(Form::Human),
        BeastTransform(StatusCounter::new_max(60)),
        WalkSpeed(6),
        Behaviour(behaviour::Behaviour::PlayerInput),
    ]
}

pub fn make_dog(x: isize, y: isize) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Tile(tile::foreground('d', ansi::YELLOW, style::BOLD)),
        TileDepth(2),
        VisionDistance(20),
        Actor(SimpleNpc),
        Collider,
        SimpleNpcKnowledge(RefCell::new(knowledge::SimpleNpcKnowledge::new())),
        WalkSpeed(1),
        SimpleNpcAi(RefCell::new(SimpleNpcAiState::new())),
        Behaviour(behaviour::Behaviour::BackAndForthForever),
    ]
}

pub fn make_bullet(x: isize, y: isize) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Tile(tile::foreground('*', ansi::RED, style::NONE)),
        TileDepth(2),
        Bullet,
        DestroyOnCollision,
    ]
}
