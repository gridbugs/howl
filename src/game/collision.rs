use ecs::entity::{EntityTable, ComponentType};
use ecs::update::UpdateSummary;

use game::rule::RuleResult;
use game::rule;
use game::game_entity::GameEntity;

use debug;

pub fn detect_collision(summary: &UpdateSummary,
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
            if cell.has(ComponentType::Solid) {
                return rule::fail();
            }
        } else {
            debug_println!("{:?}", current_position.to_tuple());
        }
    }

    rule::pass()
}
