use game::entity::{EntityTable, ComponentType};
use game::entity::Component::*;
use game::components::door::DoorState;
use game::update::UpdateSummary;
use game::update::UpdateSummary_;

use game::rule::RuleResult;
use game::rule;
use game::game_entity::GameEntity;
use game::actions;

pub fn detect_open(summary: &UpdateSummary_,
                    entities: &EntityTable)
    -> RuleResult
{
    for (entity_id, changes) in &summary.added_components {

        if !changes.has(ComponentType::Position) {
            continue;
        }

        let entity = entities.get(*entity_id);

        if !entity.has(ComponentType::Collider) {
            continue;
        }

        let level = entities.get(entity.on_level().unwrap());
        let spacial_hash = level.level_spacial_hash().unwrap();

        let new_position = changes.position().unwrap();

        if let Some(cell) = spacial_hash.get(new_position.to_tuple()) {
            if cell.has(ComponentType::Door) && cell.has(ComponentType::Solid) {
                for entity_id in &cell.entities {
                    if let Some(&Door(DoorState::Closed)) = entities.get(*entity_id).get(ComponentType::Door) {
                        return RuleResult::Instead(vec![
                            { let mut u = UpdateSummary_::new(); u.open_door(*entity_id); u }
                        ]);
                    }
                }
            }
        }
    }

    rule::pass()
}

pub fn detect_open_(summary: &UpdateSummary,
                    entities: &EntityTable)
    -> RuleResult
{
    if !summary.changed_components.contains(&ComponentType::Position) {
        return rule::pass();
    }

    for (entity_id, components) in &summary.changed_entities {

        if !components.contains_key(&ComponentType::Position) {
            continue;
        }

        let entity = entities.get(*entity_id);
        let level = entities.get(entity.on_level().unwrap());

        if !entity.has(ComponentType::Collider) {
            continue;
        }

        let spacial_hash = level.level_spacial_hash().unwrap();

        let current_position = entity.position().unwrap();

        if let Some(cell) = spacial_hash.get(current_position.to_tuple()) {
            if cell.has(ComponentType::Door) && cell.has(ComponentType::Solid) {
                for entity_id in &cell.entities {
                    if let Some(&Door(DoorState::Closed)) = entities.get(*entity_id).get(ComponentType::Door) {
                        return RuleResult::Instead(vec![
                            //actions::open_door(*entity_id),
                        ]);
                    }
                }
            }
        }
    }

    rule::pass()
}
