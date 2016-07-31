use ecs::message::Message;
use ecs::message::Field;
use ecs::message::FieldType as FType;
use ecs::systems::apply_update::apply_update;
use ecs::update::Update;

use game::context::GameContext;

pub enum ActionResult {
    Invalid,
    Success,
}

impl<'a> GameContext<'a> {

    fn apply_update(&mut self, update: &Update) -> Update {
        apply_update(update, &mut self.entities)
    }

    fn apply_action(&mut self, action: &Message) -> Update {
        if let Some(&Field::Update(ref update)) = action.get(FType::Update) {
            self.apply_update(update)
        } else {
            panic!("No Update field found in message")
        }
    }

    pub fn apply_action_over_rules(&mut self, action: &Message) -> ActionResult {

        self.apply_action(action);

        ActionResult::Success
    }
}
