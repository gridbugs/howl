use ecs::table::{Table, TableId, ToType};
use geometry::vector2::Vector2;

pub type EntityId = TableId;
pub type Entity = Table<ComponentType, Component>;

macro_rules! entity {
    ( $( $x:expr ),* ) => { table![$($x),*] }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ComponentType {
    Position,
    Solid,
}

#[derive(Copy, Clone, Debug)]
pub enum Component {
    Position(Vector2<isize>),
    Solid,
}

impl ToType<ComponentType> for Component {
    fn to_type(&self) -> ComponentType {
        match *self {
            Component::Position(_) => ComponentType::Position,
            Component::Solid => ComponentType::Solid,
        }
    }
}
