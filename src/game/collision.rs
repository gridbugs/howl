use ecs::entity::{EntityTable, ComponentType};
use ecs::message::Message;
use game::rule::RuleResult;

use game::rule;
use game::util;

pub fn detect_collision(message: &Message,
                        before: &EntityTable,
                        after: &EntityTable) -> RuleResult
{

    let entity_id = util::get_update_entity(message).unwrap();
    let before_entity = before.get(entity_id);

    if !before_entity.has(ComponentType::Collider) {
        return rule::pass();
    }

    let current_position = util::get_position(after.get(entity_id)).unwrap();

    for entity in before.tables() {
        if let Some(v) = util::get_position(entity) {
            if v == current_position && entity.has(ComponentType::Solid) {
                return rule::fail();
            }
        }
    }

    rule::pass()
}
