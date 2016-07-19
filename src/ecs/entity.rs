use ecs::table::{Table, TableId, ToType};
use geometry::vector2::Vector2;
use renderer::tile::Tile;
use colour::ansi::AnsiColour;
use game;

pub type EntityId = TableId;
pub type Entity = Table<ComponentType, Component>;

macro_rules! entity {
    () => { ecs::entity::Entity::new() };
    ( $( $x:expr ),* , ) => { entity!( $( $x ),* ) };
    ( $( $x:expr ),* ) => {{
        let mut entity = ecs::entity::Entity::new();
        $(entity.add($x);)*
        entity
    }};
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ComponentType {
    Position,
    Solid,
    SolidTile,
    TransparentTile,
    TileDepth,
    Level,
}

#[derive(Debug)]
pub enum Component {
    Position(Vector2<isize>),
    Solid,
    SolidTile { tile: Tile, background: AnsiColour },
    TransparentTile(Tile),
    TileDepth(isize),
    Level(game::level::Level),
}

impl ToType<ComponentType> for Component {
    fn to_type(&self) -> ComponentType {
        match *self {
            Component::Position(_) => ComponentType::Position,
            Component::Solid => ComponentType::Solid,
            Component::SolidTile { tile: _, background: _ } => ComponentType::SolidTile,
            Component::TransparentTile(_) => ComponentType::TransparentTile,
            Component::TileDepth(_) => ComponentType::TileDepth,
            Component::Level(_) => ComponentType::Level,
        }
    }
}
