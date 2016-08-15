use game::table::{
    Table,
    TableId,
    ToType,
    TableTable
};
use game::components::{
    Level,
    DoorState,
};
use geometry::Vector2;
use renderer::tile::Tile;
use colour::ansi::AnsiColour;

use std::collections::HashSet;
use std::collections::hash_set;

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
    SolidTile,
    TransparentTile,
    TileDepth,
    LevelData,
    PlayerActor,
    OnLevel,
    Door,
    Opacity,
}

#[derive(Debug, Clone)]
pub enum Component {
    NullComponent,
    Position(Vector2<isize>),
    Solid,
    Collider,
    SolidTile { tile: Tile, background: AnsiColour },
    TransparentTile(Tile),
    TileDepth(isize),
    LevelData(Level),
    PlayerActor,
    OnLevel(EntityId),
    Door(DoorState),
    Opacity(f64),
}

impl ToType<ComponentType> for Component {
    fn to_type(&self) -> ComponentType {
        match *self {
            Component::NullComponent => ComponentType::NullComponent,
            Component::Position(_) => ComponentType::Position,
            Component::Solid => ComponentType::Solid,
            Component::Collider => ComponentType::Collider,
            Component::SolidTile { tile: _, background: _ } => ComponentType::SolidTile,
            Component::TransparentTile(_) => ComponentType::TransparentTile,
            Component::TileDepth(_) => ComponentType::TileDepth,
            Component::LevelData(_) => ComponentType::LevelData,
            Component::PlayerActor => ComponentType::PlayerActor,
            Component::OnLevel(_) => ComponentType::OnLevel,
            Component::Door(_) => ComponentType::Door,
            Component::Opacity(_) => ComponentType::Opacity,
        }
    }
}
