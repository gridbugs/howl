use ecs::table::{Table, TableId, ToType};
use ecs::entity::EntityId;
use ecs::update::{Update, UpdateStage};

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
    NewTurn,
    RenderLevel,
    ActorTurn,
    UpdateStage,
    Update,
}

#[derive(Debug)]
pub enum Field {
    QuitGame,
    NewTurn,
    RenderLevel { level: EntityId },
    ActorTurn { actor: EntityId },
    UpdateStage(UpdateStage),
    Update(Update),
}

impl ToType<FieldType> for Field {
    fn to_type(&self) -> FieldType {
        match *self {
            Field::QuitGame => FieldType::QuitGame,
            Field::NewTurn => FieldType::NewTurn,
            Field::RenderLevel { level: _ } => FieldType::RenderLevel,
            Field::ActorTurn { actor: _ } => FieldType::ActorTurn,
            Field::UpdateStage(_) => FieldType::UpdateStage,
            Field::Update(_) => FieldType::Update,
        }
    }
}
