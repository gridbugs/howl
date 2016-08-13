use game::entity::{EntityTable, ComponentType};
use game::update::UpdateSummary;

use game::rule::RuleResult;
use game::rule;
use game::game_entity::GameEntity;

pub fn detect_collision(summary: &UpdateSummary,
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
            if cell.has(ComponentType::Solid) {
                return rule::fail();
            }
        }
    }

    rule::pass()
}
