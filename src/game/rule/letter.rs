use game::*;
use ecs_content::*;
use content_types::{ActionArgs, Reaction};

pub fn letter(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, position) in action.copy_iter_position() {

        if let Some(letter_id) = env.spatial_hash.get(position).any_letter() {

            let entity = env.ecs.entity(entity_id);

            if entity.contains_letter_count() {
                reactions.push(Reaction::new(ActionArgs::TakeLetter(entity_id, letter_id), 0));
            }
        }
    }

    RULE_ACCEPT
}
