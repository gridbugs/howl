use ecs::system_queue::SystemQueue;
use ecs::message::Message;
use ecs::entity::{EntityId, EntityTable};
use ecs::message::Field::*;
use ecs::message::FieldType as FType;
use ecs::update;
use ecs::update::Update::*;
use ecs::update::UpdateStage::*;


fn apply_update(update: &update::Update, entities: &mut EntityTable)
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
                *component = component_value.clone();
            }

            Some(entity_id)
        },
        AddEntity(ref entity) => {
            Some(entities.add(entity.clone()))
        },
        WithEntity(entity_id, ref f) => {
            f(entities.get_mut(entity_id));

            Some(entity_id)
        }
        ThenWithEntity(ref sub_update, ref f) => {
            if let Some(entity_id) = apply_update(sub_update, entities) {
                apply_update(&f(entity_id), entities)
            } else {
                unreachable!()
            }
        },
        Then(ref first, ref second) => {
            apply_update(first, entities);
            apply_update(second, entities)
        },
        Null => None,
    }
}

pub fn apply_update_message(message: &mut Message,
                    entities: &mut EntityTable, _: &SystemQueue)
{
    if let Some(&UpdateStage(Commit)) = message.get(FType::UpdateStage) {
        if let Some(&Update(ref update)) = message.get(FType::Update) {
            apply_update(update, entities);
        }
    }
}
