use game::{
    Speed,
    StatusCounter,
    Level,
    LevelId,
    LevelSpacialHashMap,
};
use game::components::{
    DoorState,
    Form,
};
use game::knowledge::DrawableKnowledge;

use table::{
    Table,
    TableId,
    TableRef,
    TableMutRef,
    ToType,
    TableTable,
};

use geometry::{
    Vector2,
    Direction,
};
use renderer::ComplexTile;

use std::collections::{
    HashSet,
    hash_set,
    HashMap,
};
use std::cell::RefCell;

pub type EntityId = TableId;
pub type EntityRef<'a> = TableRef<'a, ComponentType, Component>;
pub type EntityMutRef<'a> = TableMutRef<'a, ComponentType, Component>;
pub type Entity = Table<ComponentType, Component>;
pub type EntityTable = TableTable<ComponentType, Component>;

pub struct EntityIter<'a> {
    hash_set_iter: hash_set::Iter<'a, EntityId>,
    entities: &'a EntityContext,
}

impl<'a> Iterator for EntityIter<'a> {
    type Item = Option<EntityRef<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.hash_set_iter.next().map(|id| {
            self.entities.get(*id)
        })
    }
}

pub struct EntityContext {
    pub entities: EntityTable,
    pub levels: HashMap<LevelId, Level>,
}

impl EntityContext {
    pub fn new() -> Self {
        EntityContext {
            entities: EntityTable::new(),
            levels: HashMap::new(),
        }
    }

    pub fn reserve_level_id(&self) -> LevelId {
        self.reserve_id()
    }

    pub fn add_level(&mut self, mut level: Level) -> LevelId {

        let id = if let Some(id) = level.id {
            id
        } else {
            let id = self.reserve_level_id();
            level.id = Some(id);
            id
        };

        self.levels.insert(id, level);

        id
    }

    pub fn reserve_id(&self) -> EntityId {
        self.entities.reserve_id()
    }

    pub fn add(&mut self, entity: Entity) -> EntityId {
        self.entities.add(entity)
    }

    pub fn remove(&mut self, id: EntityId) -> Option<Entity> {
        self.entities.remove(id)
    }

    pub fn get(&self, id: EntityId) -> Option<EntityRef> {
        self.entities.get(id)
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<EntityMutRef> {
        self.entities.get_mut(id)
    }

    pub fn id_set_iter<'a>(&'a self, set: &'a HashSet<EntityId>) -> EntityIter<'a> {
        EntityIter {
            hash_set_iter: set.iter(),
            entities: &self,
        }
    }

    pub fn level(&self, level_id: LevelId) -> Option<&Level> {
        self.levels.get(&level_id)
    }

    pub fn level_mut(&mut self, level_id: LevelId) -> Option<&mut Level> {
        self.levels.get_mut(&level_id)
    }

    pub fn spacial_hash(&self, level_id: LevelId) -> Option<&LevelSpacialHashMap> {
        self.level(level_id).map(|level| {
            &level.spacial_hash
        })
    }
}

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
