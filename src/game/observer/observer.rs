use game::{
    EntityId,
    EntityContext,
};

/// Different characters may represent their knowledge of the world in different ways. Observer is
/// an interface for updating knowledge by observing the world using a vision system.
pub trait Observer {
    fn observe(&mut self, entity_id: EntityId, entities: &EntityContext, turn_count: u64) -> bool;
}
