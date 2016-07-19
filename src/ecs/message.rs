use ecs::table::{Table, TableId, ToType};
use ecs::entity::EntityId;

pub type MessageId = TableId;
pub type Message = Table<FieldType, Field>;

macro_rules! message {
    () => { ecs::message::Message::new() };
    ( $( $x:expr ),* , ) => { message!( $( $x ),* ) };
    ( $( $x:expr ),* ) => {{
        let mut message = ecs::message::Message::new();
        $(message.add($x);)*
        message
    }};
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum FieldType {
    RenderLevel,
}

#[derive(Debug)]
pub enum Field {
    RenderLevel { level: EntityId },
}

impl ToType<FieldType> for Field {
    fn to_type(&self) -> FieldType {
        match *self {
            Field::RenderLevel { level: _ } => FieldType::RenderLevel,
        }
    }
}
