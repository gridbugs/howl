use ecs::table::{Table, TableId, ToType};
use ecs::entity::EntityId;
use ecs::update::Update;

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
    QuitGame,
    ActorTurn,
    Update,
    UpdateEntity,
}

#[derive(Debug)]
pub enum Field {
    QuitGame,
    ActorTurn { actor: EntityId },
    Update(Update),
    UpdateEntity(EntityId),
}

impl ToType<FieldType> for Field {
    fn to_type(&self) -> FieldType {
        match *self {
            Field::QuitGame => FieldType::QuitGame,
            Field::ActorTurn { actor: _ } => FieldType::ActorTurn,
            Field::Update(_) => FieldType::Update,
            Field::UpdateEntity(_) => FieldType::UpdateEntity,
        }
    }
}
