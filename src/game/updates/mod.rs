use game::entity::{
    EntityId,
    Component,
    EntityTable,
};
use game::table::table::ToType;

use game::update::monad::{UpdateMonad, Action};

use std::mem;

pub fn set_entity_component<F: 'static>(f: F) -> Action
    where F: Fn(&mut EntityTable) -> (EntityId, Component)
{
    UpdateMonad::new(move |summary, entities| {
        let (entity_id, new_component) = f(entities);
        let mut entity = entities.get_mut(entity_id);

        if let Some(current_component) = entity.get_mut(new_component.to_type()) {
            let original_component = mem::replace(current_component, new_component);
            summary.change_entity(entity_id, original_component);
        } else {
            panic!("No component of type {:?} found.", new_component.to_type());
        }
    })
}
