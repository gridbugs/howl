use game::{
    ComponentType,
    RuleResult,
    RuleContext,
};
use game::entity::Component::*;
use game::components::DoorState;
use game::actions;
use game::rule;

pub fn detect_open(ctx: RuleContext)
    -> RuleResult
{
    for (entity_id, changes) in &ctx.update.added_components {

        if !changes.has(ComponentType::Position) {
            continue;
        }

        let entity = ctx.entities.get(*entity_id).unwrap();

        if !entity.is_collider() || !entity.is_door_opener() {
            continue;
        }

        let spacial_hash = ctx.entities.spacial_hash(entity.on_level().unwrap()).unwrap();

        let new_position = changes.position().unwrap();

        if let Some(cell) = spacial_hash.get(new_position.to_tuple()) {
            if cell.has(ComponentType::Door) && cell.has(ComponentType::Solid) {
                for entity_id in &cell.entities {
                    if let Some(&Door(DoorState::Closed)) = ctx.entities.get(*entity_id).unwrap().get(ComponentType::Door) {
                        return RuleResult::Instead(vec![
                            (0, actions::open_door(*entity_id))
                        ]);
                    }
                }
            }
        }
    }

    rule::pass()
}
