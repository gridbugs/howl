use game::{
    Speed,
    StatusCounter,
    LevelId,
};
use game::components::{
    DoorState,
    Form,
};
use game::knowledge::DrawableKnowledge;

use table::{
    TableId,
    ToType,
    ToIndex,
    Table,
    TableTable,
    HashMapTableRef,
    HashMapTableRefMut,
    HashMapTableTable,
    FlatTableRef,
    FlatTableRefMut,
    FlatTableTable,
    InvertedTableRef,
    InvertedTableRefMut,
    InvertedTableTable,
    TableRef,
    IterTableRef,
    TableRefMut,
    IdTableRef,
};

use geometry::{
    Vector2,
    Direction,
};
use renderer::ComplexTile;

use std::cell::RefCell;

pub type HashMapEntityRef<'a> = HashMapTableRef<'a, ComponentType, Component>;
pub type HashMapEntityRefMut<'a> = HashMapTableRefMut<'a, ComponentType, Component>;
pub type HashMapEntityTable = HashMapTableTable<ComponentType, Component>;

pub type FlatEntityRef<'a> = FlatTableRef<'a, ComponentType, Component>;
pub type FlatEntityRefMut<'a> = FlatTableRefMut<'a, ComponentType, Component>;
pub type FlatEntityTable = FlatTableTable<ComponentType, Component>;

pub type InvertedEntityRef<'a> = InvertedTableRef<'a, ComponentType, Component>;
pub type InvertedEntityRefMut<'a> = InvertedTableRefMut<'a, ComponentType, Component>;
pub type InvertedEntityTable = InvertedTableTable<ComponentType, Component>;

pub type EntityId = TableId;
pub type Entity = Table<ComponentType, Component>;

pub trait EntityTable<'a>: TableTable<'a, ComponentType, Component> {}

impl<'a> EntityTable<'a> for HashMapEntityTable {}
impl<'a> EntityTable<'a> for FlatEntityTable {}
impl<'a> EntityTable<'a> for InvertedEntityTable {}

pub trait EntityRef<'a>: TableRef<'a, ComponentType, Component> {}
pub trait IterEntityRef<'a>: IterTableRef<'a, ComponentType, Component> + EntityRef<'a> {}
pub trait EntityRefMut<'a>: TableRefMut<'a, ComponentType, Component> {}
pub trait IdEntityRef<'a>: IdTableRef<'a, ComponentType, Component> + IterEntityRef<'a> {}

impl<'a> EntityRef<'a> for &'a Entity {}
impl<'a> IterEntityRef<'a> for &'a Entity {}
impl<'a> EntityRefMut<'a> for Entity {}
impl<'a> EntityRefMut<'a> for &'a mut Entity {}

impl<'a> EntityRef<'a> for HashMapEntityRef<'a> {}
impl<'a> IterEntityRef<'a> for HashMapEntityRef<'a> {}
impl<'a> EntityRefMut<'a> for HashMapEntityRefMut<'a> {}
impl<'a> IdEntityRef<'a> for HashMapEntityRef<'a> {}

impl<'a> EntityRef<'a> for FlatEntityRef<'a> {}
impl<'a> IterEntityRef<'a> for FlatEntityRef<'a> {}
impl<'a> EntityRefMut<'a> for FlatEntityRefMut<'a> {}
impl<'a> IdEntityRef<'a> for FlatEntityRef<'a> {}

impl<'a> EntityRef<'a> for InvertedEntityRef<'a> {}
impl<'a> IterEntityRef<'a> for InvertedEntityRef<'a> {}
impl<'a> EntityRefMut<'a> for InvertedEntityRefMut<'a> {}
impl<'a> IdEntityRef<'a> for InvertedEntityRef<'a> {}

macro_rules! entity {
    () => { game::entity::Entity::new() };
    ( $( $x:expr ),* , ) => { entity!( $( $x ),* ) };
    ( $( $x:expr ),* ) => {{
        let mut entity = game::entity::Entity::new();
        $(entity.add($x);)*
        entity
    }};
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
    PlayerActor,
    OnLevel,
    Door,
    DoorOpener,
    Opacity,
    VisionDistance,
    DrawableKnowledge,
    Bullet,
    AxisVelocity,
    BeastTransform,
    HumanTransform,
    FormSlot,
    Outside,
    Moon,
}

pub const NUM_COMPONENTS: usize = 21;

#[derive(Debug, Clone)]
pub enum Component {
    NullComponent,
    Position(Vector2<isize>),
    Solid,
    Collider,
    DestroyOnCollision,
    Tile(ComplexTile),
    TileDepth(isize),
    PlayerActor,
    OnLevel(LevelId),
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
            Component::PlayerActor => ComponentType::PlayerActor,
            Component::OnLevel(_) => ComponentType::OnLevel,
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
        }
    }
}

impl ToIndex for ComponentType {
    fn num_indices() -> usize {
        NUM_COMPONENTS
    }

    fn to_index(&self) -> usize {
        *self as usize
    }
}
