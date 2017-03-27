#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate util;
extern crate ecs_core;
extern crate behaviour;

use std::cell::RefCell;

use ecs_core::EntityId;
use util::LeakyReserver;

pub type LevelId = usize;

pub struct EntityIdReserver(RefCell<LeakyReserver<EntityId>>);

impl EntityIdReserver {
    pub fn new() -> Self {
        EntityIdReserver(RefCell::new(LeakyReserver::new()))
    }

    pub fn new_id(&self) -> EntityId {
        self.0.borrow_mut().reserve()
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableEntityIdReserver(LeakyReserver<EntityId>);

impl From<EntityIdReserver> for SerializableEntityIdReserver {
    fn from(r: EntityIdReserver) -> Self {
        SerializableEntityIdReserver(r.0.into_inner())
    }
}

impl From<SerializableEntityIdReserver> for EntityIdReserver {
    fn from(r: SerializableEntityIdReserver) -> Self {
        EntityIdReserver(RefCell::new(r.0))
    }
}

pub type BehaviourState = behaviour::State;
pub type BehaviourNodeIndex = behaviour::NodeIndex;
