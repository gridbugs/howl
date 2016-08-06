use ecs::table::{Table, TableId, ToType};
use ecs::table_table::TableTable;
use ecs::components;
use geometry::vector2::Vector2;
use renderer::tile::Tile;
use colour::ansi::AnsiColour;

pub type EntityId = TableId;
pub type Entity = Table<ComponentType, Component>;
pub type EntityTable = TableTable<ComponentType, Component>;

macro_rules! entity {
    () => { ecs::entity::Entity::new() };
    ( $( $x:expr ),* , ) => { entity!( $( $x ),* ) };
    ( $( $x:expr ),* ) => {{
        let mut entity = ecs::entity::Entity::new();
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
    SolidTile,
    TransparentTile,
    TileDepth,
    LevelData,
    PlayerActor,
    OnLevel,
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
    LevelData(components::level::Level),
    PlayerActor,
    OnLevel(EntityId),
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
        }
    }
}
