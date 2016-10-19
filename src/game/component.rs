use game::{Speed, StatusCounter, ActorType};
use game::components::{DoorState, Form};
use game::knowledge::{DrawableKnowledge, SimpleNpcKnowledge};
use game::actors::SimpleNpcAiState;
use game::behaviour::Behaviour;

use table::{ToIndex, ToType};
use geometry::{Vector2, Direction};
use tile::ComplexTile;
use behaviour;

use std::cell::RefCell;

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
    Actor, // 07
    Door, // 08
    DoorOpener, // 09
    Opacity, // 10
    VisionDistance, // 11
    DrawableKnowledge, // 12
    Bullet, // 13
    AxisVelocity, // 14
    BeastTransform, // 15
    HumanTransform, // 16
    FormSlot, // 17
    Outside, // 18
    Moon, // 19
    SimpleNpcKnowledge, // 20
    PlayerCharacter, // 21
    WalkSpeed, // 22
    SimpleNpcAi, // 23
    Behaviour, // 24
    BehaviourState, // 25
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
    Actor(ActorType),
    Door(DoorState),
    DoorOpener,
    Opacity(f64),
    VisionDistance(usize),
    DrawableKnowledge(RefCell<DrawableKnowledge>),
    Bullet,
    AxisVelocity { direction: Direction, speed: Speed },
    BeastTransform(StatusCounter),
    HumanTransform(StatusCounter),
    FormSlot(Form),
    Outside,
    Moon,
    SimpleNpcKnowledge(RefCell<SimpleNpcKnowledge>),
    PlayerCharacter,
    WalkSpeed(u64),
    SimpleNpcAi(RefCell<SimpleNpcAiState>),
    Behaviour(Behaviour),
    BehaviourState(behaviour::State),
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
            Component::Actor(_) => ComponentType::Actor,
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
            Component::SimpleNpcAi(_) => ComponentType::SimpleNpcAi,
            Component::Behaviour(_) => ComponentType::Behaviour,
            Component::BehaviourState(_) => ComponentType::BehaviourState,
        }
    }
}
