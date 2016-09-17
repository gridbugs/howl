use game::{
    Rule,
    actions,
    ComponentType,
    RuleResult,
    RuleContext,
    EntityWrapper,
    EntityStore,
};

use game::update::Metadatum::*;

use table::TableRef;

pub struct DetectCollision;

impl Rule for DetectCollision {
    fn check(&self, ctx: RuleContext) -> RuleResult {
        for (entity_id, changes) in &ctx.update.added_components {

            if !changes.has(ComponentType::Position) {
                continue;
            }

            let entity = ctx.level.get(*entity_id).unwrap();

            if !entity.has(ComponentType::Collider) {
                continue;
            }

            let spatial_hash = ctx.level.spatial_hash();

            let new_position = changes.position().unwrap();

            if let Some(cell) = spatial_hash.get(new_position.to_tuple()) {
                if cell.has(ComponentType::Solid) {
                    if entity.is_destroy_on_collision() {
                        let mut remove = actions::remove_entity(entity);
                        remove.set_metadata(ActionTime(1));
                        return RuleResult::instead(remove);
                    } else {
                        return RuleResult::fail();
                    }
                }
            }
        }

        RuleResult::pass()
    }
}
