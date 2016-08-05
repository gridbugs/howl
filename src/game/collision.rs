use ecs::entity::{EntityTable, ComponentType};
use ecs::message::Message;
use ecs::update::UpdateSummary;

use game::rule::RuleResult;
use game::rule;
use game::util;

use debug;

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

        let entity = after.get(*entity_id);
        let level = after.get(util::get_level(entity).unwrap());

        if !entity.has(ComponentType::Collider) {
            continue;
        }

        let spacial_hash = util::get_level_spacial_hash(level).unwrap();

        let current_position = util::get_position(entity).unwrap();

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
