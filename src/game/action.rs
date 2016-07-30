use ecs::message::Message;
use ecs::systems::apply_update::apply_update;

use game::context::GameContext;

pub enum ActionResult {
    Invalid,
    Success,
}

impl<'a> GameContext<'a> {
    pub fn apply_action(&mut self, action: &Message) -> ActionResult {
        apply_update(action, &mut self.entities);

        ActionResult::Success
    }
}
