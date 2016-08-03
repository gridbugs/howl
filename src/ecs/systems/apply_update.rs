use ecs::entity::{EntityId, EntityTable};
use ecs::update;
use ecs::update::Update::*;
use ecs::update::UpdateSummary;


fn do_apply_update(update: &update::Update, entities: &mut EntityTable, summary: &mut UpdateSummary)
    -> Option<EntityId>
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

                summary.change_entity(entity_id, original);

                Some(entity_id)
            } else {
                panic!("SetEntityComponent requires component to be present")
            }
        },
        AddEntity(ref entity) => {
            let id = entities.add(entity.clone());

            summary.add_entity(id);

            Some(id)
        },
        RemoveEntity(entity_id) => {
            let entity = entities.remove(entity_id).unwrap();

            summary.remove_entity(entity);

            None
        },
        WithEntity(entity_id, ref f) => {
            f(entities.get_mut(entity_id));

            Some(entity_id)
        }
        ThenWithEntity(ref sub_update, ref f) => {
            if let Some(entity_id) = do_apply_update(sub_update, entities, summary) {
                do_apply_update(&f(entity_id), entities, summary)
            } else {
                panic!("ThenWithEntity requires the first action to yield an entity.")
            }
        },
        Then(ref first, ref second) => {
            do_apply_update(first, entities, summary);
            do_apply_update(second, entities, summary)
        },
        Null => None,
        Error(message) => {
            panic!(message)
        }
    }
}

pub fn apply_update(update: &update::Update,
                    entities: &mut EntityTable) -> UpdateSummary
{
    let mut summary = UpdateSummary::new();
    do_apply_update(update, entities, &mut summary);
    summary
}
