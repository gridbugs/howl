use ecs::entity::{EntityTable, ComponentType};
use ecs::message::Message;
use ecs::update::UpdateSummary;

use game::rule::RuleResult;
use game::rule;
use game::util;

pub fn detect_collision(_: &Message,
                        summary: &UpdateSummary,
                        before: &EntityTable,
                        after: &EntityTable) -> RuleResult
{
    if !summary.changed_components.contains(&ComponentType::Position) {
        return rule::pass();
    }

    for (entity_id, components) in &summary.changed_entities {

        if !components.contains_key(&ComponentType::Position) {
            continue;
        }

        if !before.get(*entity_id).has(ComponentType::Collider) {
            continue;
        }

        let current_position = util::get_position(after.get(*entity_id)).unwrap();

        for entity in before.tables() {
            if let Some(v) = util::get_position(entity) {
                if v == current_position && entity.has(ComponentType::Solid) {
                    return rule::fail();
                }
            }
        }
    }

    rule::pass()
}
