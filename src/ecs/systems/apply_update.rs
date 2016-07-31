use ecs::entity::{EntityId, EntityTable};
use ecs::update;
use ecs::update::Update::*;


fn do_apply_update(update: &update::Update, entities: &mut EntityTable)
    -> (Option<EntityId>, update::Update)
{
    match *update {
        SetEntityComponent {
            entity_id,
            component_type,
            ref component_value
        } => {
            let entity = entities.get_mut(entity_id);
            if let Some(component) = entity.get_mut(component_type) {

                let original = component.clone();
                *component = component_value.clone();

                (Some(entity_id), SetEntityComponent {
                    entity_id: entity_id,
                    component_type: component_type,
                    component_value: original
                })
            } else {
                panic!("SetEntityComponent requires component to be present")
            }
        },
        AddEntity(ref entity) => {
            let id = entities.add(entity.clone());
            (Some(id), RemoveEntity(id))
        },
        RemoveEntity(entity_id) => {
            let entity = entities.remove(entity_id).unwrap();

            (None, AddEntity(entity))
        },
        WithEntity(entity_id, ref f) => {
            f(entities.get_mut(entity_id));

            (Some(entity_id), Error("Can't revert"))
        }
        ThenWithEntity(ref sub_update, ref f) => {
            if let (Some(entity_id), revert_a) = do_apply_update(sub_update, entities) {
                let (maybe_id, revert_b) = do_apply_update(&f(entity_id), entities);

                (maybe_id, update::then(revert_b, revert_a))
            } else {
                panic!("ThenWithEntity requires the first action to yield an entity.")
            }
        },
        Then(ref first, ref second) => {
            let (_, revert_a) = do_apply_update(first, entities);
            let (maybe_id, revert_b) = do_apply_update(second, entities);

            (maybe_id, update::then(revert_b, revert_a))
        },
        Null => (None, Null),
        Error(message) => {
            panic!(message)
        }
    }
}

pub fn apply_update(update: &update::Update,
                    entities: &mut EntityTable) -> update::Update
{
    do_apply_update(update, entities).1
}
