#[macro_use] mod entity;

pub use self::entity::{
    EntityId,
    EntityRef,
    IterEntityRef,
    EntityRefMut,
    IdEntityRef,
    Entity,
    EntityTable,
    Component,
    ComponentType,
    FlatEntityTable,
    FlatEntityRef,
    FlatEntityRefMut,
    HashMapEntityTable,
    HashMapEntityRef,
    HashMapEntityRefMut,
};

#[cfg(feature = "flat_table")]
pub type LevelEntityTable = FlatEntityTable;
#[cfg(feature = "flat_table")]
pub type LevelEntityRef<'a> = FlatEntityRef<'a>;
#[cfg(feature = "flat_table")]
pub type LevelEntityRefMut<'a> = FlatEntityRefMut<'a>;

#[cfg(not(feature = "flat_table"))]
pub type LevelEntityTable = HashMapEntityTable;
#[cfg(not(feature = "flat_table"))]
pub type LevelEntityRef<'a> = HashMapEntityRef<'a>;
#[cfg(not(feature = "flat_table"))]
pub type LevelEntityRefMut<'a> = HashMapEntityRefMut<'a>;


mod entity_context;
pub use self::entity_context::{
    EntityContext,
};

mod entity_store;
pub use self::entity_store::{
    EntityStore,
};

pub mod io;
pub mod rules;
pub mod actions;
pub mod entities;
pub mod components;

mod meta_action;
pub use self::meta_action::MetaAction;

pub mod update;
pub use self::update::UpdateSummary;

mod turn_schedule;
pub use self::turn_schedule::TurnSchedule;

mod context;
pub use self::context::GameContext;

mod entity_wrapper;
pub use self::entity_wrapper::EntityWrapper;

mod rule;
pub use self::rule::{
    Rule,
    RuleResult,
    RuleContext,
};

mod spacial_hash;
pub use self::spacial_hash::{
    SpacialHashMap,
    SpacialHashCell,
};

pub mod observer;

pub mod knowledge;

mod speed;
pub use self::speed::Speed;

mod status;
pub use self::status::StatusCounter;

mod level;
pub use self::level::{
    Level,
    LevelId,
    LevelSpacialHashMap,
};
