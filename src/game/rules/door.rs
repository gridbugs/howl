use game::{
    ComponentType,
    RuleResult,
    RuleContext,
    actions,
    Rule,
    EntityWrapper,
    EntityStore,
};
use game::Component::*;
use game::components::DoorState;

use table::TableRef;

pub struct DetectOpen;

impl Rule for DetectOpen {
    fn check(&self, ctx: RuleContext) -> RuleResult {
        for (entity_id, changes) in &ctx.update.added_components {

            if !changes.has(ComponentType::Position) {
                continue;
            }

            let entity = ctx.level.get(*entity_id).unwrap();

            if !entity.is_collider() || !entity.is_door_opener() {
                continue;
            }

            let spatial_hash = ctx.level.spatial_hash();

            let new_position = changes.position().unwrap();

            if let Some(cell) = spatial_hash.get(new_position.to_tuple()) {
                if cell.has(ComponentType::Door) && cell.has(ComponentType::Solid) {
                    for entity_id in &cell.entities {
                        if let Some(&Door(DoorState::Closed)) = ctx.level.get(*entity_id).unwrap().get(ComponentType::Door) {
                            return RuleResult::instead(actions::open_door(*entity_id));
                        }
                    }
                }
            }
        }

        RuleResult::pass()
    }
}
