use ecs::system_queue::SystemQueue;
use ecs::message::Message;
use ecs::entity::EntityTable;
use ecs::message::Field::*;
use ecs::message::FieldType as FType;
use ecs::update::Update::*;
use ecs::update::UpdateStage::*;

pub fn apply_update(message: &mut Message,
                    entities: &mut EntityTable, _: &SystemQueue)
{
    if let Some(&UpdateStage(Commit)) = message.get(FType::UpdateStage) {
        if let Some(&Update(SetEntityComponent{
            entity_id,
            component_type,
            ref component_value
        })) = message.get(FType::Update) {
            let entity = entities.get_mut(entity_id);
            if let Some(component) = entity.get_mut(component_type) {
                *component = component_value.clone();
            }
        }
    }
}
