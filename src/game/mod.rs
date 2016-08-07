pub mod context;
pub mod rule;
pub mod game_entity;
pub mod control;
pub mod spacial_hash;
pub mod schedule;
#[macro_use] pub mod entity;

pub mod io;
#[macro_use] pub mod table;
pub mod update;

pub mod rules;
pub mod actions;
pub mod entities;
pub mod components;

pub use self::entity::{
    EntityId,
    Entity,
    Component,
    ComponentType,
    EntityTable,
};
