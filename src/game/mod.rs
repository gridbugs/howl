#[macro_use] mod entity;

pub use self::entity::{
    EntityId,
    EntityRef,
    IterEntityRef,
    EntityRefMut,
    IdEntityRef,
    Entity,
    EntityTable,
    InvertedEntityTable,
    InvertedEntityRef,
    InvertedEntityRefMut,
};

mod component;
pub use self::component::{
    Component,
    ComponentType,
};

pub type LevelEntityTable = InvertedEntityTable;
pub type LevelEntityRef<'a> = InvertedEntityRef<'a>;
pub type LevelEntityRefMut<'a> = InvertedEntityRefMut<'a>;

mod entity_context;
pub use self::entity_context::{
    EntityContext,
    ReserveEntityId,
    LevelStore,
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
pub mod actors;

mod meta_action;
pub use self::meta_action::MetaAction;

pub mod update;
pub use self::update::{
    UpdateSummary,
    AddedComponents,
    Metadata,
    MetadataWrapper,
};

mod turn_schedule;
pub use self::turn_schedule::TurnSchedule;

mod context;
pub use self::context::GameContext;

mod entity_wrapper;
pub use self::entity_wrapper::EntityWrapper;

mod component_wrapper;
pub use self::component_wrapper::ComponentWrapper;

mod rule;
pub use self::rule::{
    Rule,
    RuleResult,
    RuleContext,
    Reaction,
};

mod spatial_hash;
pub use self::spatial_hash::{
    SpatialHashMap,
    SpatialHashCell,
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
    LevelSpatialHashMap,
};

mod actor;
pub use self::actor::ActorType;

mod clouds;

mod commit_context;
pub use self::commit_context::{
    CommitContext,
    CommitTime,
    CommitError,
};

mod renderer;
pub use self::renderer::Renderer;

mod actor_manager;
pub use self::actor_manager::ActorManager;
