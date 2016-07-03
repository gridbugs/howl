use ecs::table::{Table, TableId, ToType};
use geometry::vector2::Vector2;
use renderer::tile::Tile;
use colour::rgb24::Rgb24;

pub type EntityId = TableId;
pub type Entity = Table<ComponentType, Component>;

macro_rules! entity {
    ( $( $x:expr ),* ) => { table![$($x),*] }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ComponentType {
    Position,
    Solid,
    SolidTile,
    TransparentTile,
}

#[derive(Copy, Clone, Debug)]
pub enum Component {
    Position(Vector2<isize>),
    Solid,
    SolidTile { tile: Tile, background: Rgb24 },
    TransparentTile(Tile),
}

impl ToType<ComponentType> for Component {
    fn to_type(&self) -> ComponentType {
        match *self {
            Component::Position(_) => ComponentType::Position,
            Component::Solid => ComponentType::Solid,
            Component::SolidTile { tile: _, background: _ } => ComponentType::SolidTile,
            Component::TransparentTile(_) => ComponentType::TransparentTile,
        }
    }
}
