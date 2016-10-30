use game::{Speed, StatusCounter};
use game::components::{DoorState, Form};
use game::knowledge::{DrawableKnowledge, SimpleNpcKnowledge};
use game::behaviour::Behaviour;

use table::{ToIndex, ToType};
use geometry::{Vector2, Direction};
use tile::ComplexTile;
use behaviour;
use terminal;

use std::cell::RefCell;
use std::collections::HashSet;

impl ToIndex for ComponentType {
    fn num_indices() -> usize {
        NUM_COMPONENTS
    }

    fn to_index(&self) -> usize {
        *self as usize
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ComponentType {
    NullComponent, // 00
    Position, // 01
    Solid, // 02
    Collider, // 03
    DestroyOnCollision, // 04
    Tile, // 05
    TileDepth, // 06
    Door, // 07
    DoorOpener, // 08
    Opacity, // 09
    VisionDistance, // 10
    DrawableKnowledge, // 11
    Bullet, // 12
    AxisVelocity, // 13
    BeastTransform, // 14
    HumanTransform, // 15
    FormSlot, // 16
    Outside, // 17
    Moon, // 18
    SimpleNpcKnowledge, // 19
    PlayerCharacter, // 20
    WalkSpeed, // 21
    Behaviour, // 22
    BehaviourState, // 23
    InputSource, // 24
    TargetSet, // 25
}
pub const NUM_COMPONENTS: usize = 26;

#[derive(Clone)]
pub enum Component {
    NullComponent,
    Position(Vector2<isize>),
    Solid,
    Collider,
    DestroyOnCollision,
    Tile(ComplexTile),
    TileDepth(isize),
    Door(DoorState),
    DoorOpener,
    Opacity(f64),
    VisionDistance(usize),
    DrawableKnowledge(RefCell<DrawableKnowledge>),
    Bullet,
    AxisVelocity {
        direction: Direction,
        speed: Speed,
    },
    BeastTransform(StatusCounter),
    HumanTransform(StatusCounter),
    FormSlot(Form),
    Outside,
    Moon,
    SimpleNpcKnowledge(RefCell<SimpleNpcKnowledge>),
    PlayerCharacter,
    WalkSpeed(u64),
    Behaviour(Behaviour),
    BehaviourState(RefCell<behaviour::State>),
    InputSource(terminal::InputSource),
    TargetSet(RefCell<HashSet<Vector2<isize>>>),
}

impl ToType<ComponentType> for Component {
    fn to_type(&self) -> ComponentType {
        match *self {
            Component::NullComponent => ComponentType::NullComponent,
            Component::Position(_) => ComponentType::Position,
            Component::Solid => ComponentType::Solid,
            Component::Collider => ComponentType::Collider,
            Component::DestroyOnCollision => ComponentType::DestroyOnCollision,
            Component::Tile(_) => ComponentType::Tile,
            Component::TileDepth(_) => ComponentType::TileDepth,
            Component::Door(_) => ComponentType::Door,
            Component::DoorOpener => ComponentType::DoorOpener,
            Component::Opacity(_) => ComponentType::Opacity,
            Component::VisionDistance(_) => ComponentType::VisionDistance,
            Component::DrawableKnowledge(_) => ComponentType::DrawableKnowledge,
            Component::Bullet => ComponentType::Bullet,
            Component::AxisVelocity { direction: _, speed: _ } => ComponentType::AxisVelocity,
            Component::BeastTransform(_) => ComponentType::BeastTransform,
            Component::HumanTransform(_) => ComponentType::HumanTransform,
            Component::FormSlot(_) => ComponentType::FormSlot,
            Component::Outside => ComponentType::Outside,
            Component::Moon => ComponentType::Moon,
            Component::SimpleNpcKnowledge(_) => ComponentType::SimpleNpcKnowledge,
            Component::PlayerCharacter => ComponentType::PlayerCharacter,
            Component::WalkSpeed(_) => ComponentType::WalkSpeed,
            Component::Behaviour(_) => ComponentType::Behaviour,
            Component::BehaviourState(_) => ComponentType::BehaviourState,
            Component::InputSource(_) => ComponentType::InputSource,
            Component::TargetSet(_) => ComponentType::TargetSet,
        }
    }
}
