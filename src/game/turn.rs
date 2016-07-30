use ecs::message::FieldType;

use game::context::GameContext;
use game::action::ActionResult;

enum TurnResult {
    Continue,
    QuitGame,
}

impl<'a> GameContext<'a> {
    fn game_turn(&mut self) -> TurnResult {
        let turn = self.pc_schedule_next();

        if self.turn_entity_is_pc(&turn) {
            self.render_pc_level();
        }

        loop {
            let action = self.get_action(&turn);

            if action.has(FieldType::QuitGame) {
                return TurnResult::QuitGame;
            }

            if let ActionResult::Success = self.apply_action(&action) {
                break;
            }
        }

        self.render_pc_level();

        TurnResult::Continue
    }

    pub fn game_loop(&mut self) {
        loop {
            if let TurnResult::QuitGame = self.game_turn() {
                break;
            }
        }
    }
}
