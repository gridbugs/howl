use game::{
    Speed,
    StatusCounter,
    Level,
};
use game::components::{
    DoorState,
    Moonlight,
    Form,
};
use game::knowledge::DrawableKnowledge;

use table::{
    Table,
    TableId,
    ToType,
    TableTable
};

use geometry::{
    Vector2,
    Direction,
};
use renderer::ComplexTile;

use std::collections::HashSet;
use std::collections::hash_set;
use std::cell::RefCell;

pub type EntityId = TableId;
pub type Entity = Table<ComponentType, Component>;
pub type EntityTable = TableTable<ComponentType, Component>;

macro_rules! entity {
    () => { game::entity::Entity::new() };
    ( $( $x:expr ),* , ) => { entity!( $( $x ),* ) };
    ( $( $x:expr ),* ) => {{
        let mut entity = game::entity::Entity::new();
        $(entity.add($x);)*
        entity
    }};
}

pub struct EntityIter<'a> {
    hash_set_iter: hash_set::Iter<'a, EntityId>,
    entities: &'a EntityTable,
}

impl<'a> Iterator for EntityIter<'a> {
    type Item = &'a Entity;
    fn next(&mut self) -> Option<Self::Item> {
        self.hash_set_iter.next().map(|id| {
            self.entities.get(*id)
        })
    }
}

impl EntityTable {
    pub fn id_set_iter<'a>(&'a self, set: &'a HashSet<EntityId>) -> EntityIter<'a> {
        EntityIter {
            hash_set_iter: set.iter(),
            entities: self,
        }
    }
}


#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ComponentType {
    NullComponent,
    Position,
    Solid,
    Collider,
    DestroyOnCollision,
    Tile,
    TileDepth,
    LevelData,
    PlayerActor,
    OnLevel,
    Door,
    DoorOpener,
    Opacity,
    VisionDistance,
    DrawableKnowledge,
    Bullet,
    AxisVelocity,
    MoonlightSlot,
    BeastTransform,
    HumanTransform,
    FormSlot,
}

#[derive(Debug, Clone)]
pub enum Component {
    NullComponent,
    Position(Vector2<isize>),
    Solid,
    Collider,
    DestroyOnCollision,
    Tile(ComplexTile),
    TileDepth(isize),
    LevelData(Box<Level>),
    PlayerActor,
    OnLevel(EntityId),
    Door(DoorState),
    DoorOpener,
    Opacity(f64),
    VisionDistance(usize),
    DrawableKnowledge(RefCell<DrawableKnowledge>),
    Bullet,
    AxisVelocity { direction: Direction, speed: Speed },
    MoonlightSlot(Moonlight),
    BeastTransform(StatusCounter),
    HumanTransform(StatusCounter),
    FormSlot(Form),
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
            Component::LevelData(_) => ComponentType::LevelData,
            Component::PlayerActor => ComponentType::PlayerActor,
            Component::OnLevel(_) => ComponentType::OnLevel,
            Component::Door(_) => ComponentType::Door,
            Component::DoorOpener => ComponentType::DoorOpener,
            Component::Opacity(_) => ComponentType::Opacity,
            Component::VisionDistance(_) => ComponentType::VisionDistance,
            Component::DrawableKnowledge(_) => ComponentType::DrawableKnowledge,
            Component::Bullet => ComponentType::Bullet,
            Component::AxisVelocity { direction: _, speed: _ } => ComponentType::AxisVelocity,
            Component::MoonlightSlot(_) => ComponentType::MoonlightSlot,
            Component::BeastTransform(_) => ComponentType::BeastTransform,
            Component::HumanTransform(_) => ComponentType::HumanTransform,
            Component::FormSlot(_) => ComponentType::FormSlot,
        }
    }
}
