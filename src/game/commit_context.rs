use game::{
    UpdateSummary,
    Level,
    Rule,
    EntityId,
    Renderer,
    ReserveEntityId,
};

use schedule::Schedule;

use std::thread;
use std::time::Duration;

pub struct CommitContext {
    update_queue: Schedule<UpdateSummary>,
}

impl CommitContext {
    pub fn new() -> Self {
        CommitContext {
            update_queue: Schedule::new(),
        }
    }

    pub fn apply_update(
        &mut self,
        level: &mut Level,
        initial_update: UpdateSummary,
        rules: &Vec<Box<Rule>>,
        mut renderer: Option<(EntityId, &mut Renderer)>,
        ids: &ReserveEntityId,
        initial_turn: u64) -> u64
    {
        let mut turn = initial_turn;

        // start with the initial action in the queue
        self.update_queue.insert(initial_update, 0);

        while let Some((update, time_delta)) = self.update_queue.next() {

            // render the scene if time has passed
            if time_delta != 0 {
                if let Some((id, ref mut r)) = renderer {
                    if r.render(level, id, turn) {
                        // only delay if something changed
                        thread::sleep(Duration::from_millis(time_delta));
                    }
                }
            }

            // check the update against all rules
            let mut result = level.check_rules(&update, rules, ids);

            let mut action_time = 0;

            if result.is_accept() {
                action_time = update.action_time();
                level.commit_update(update, turn);

                turn += 1;
            }

            for reaction in result.drain_reactions() {
                self.update_queue.insert(
                    reaction.action,
                    action_time + reaction.delay);
            }
        }

        if let Some((id, ref mut r)) = renderer {
            r.render(level, id, turn);
        }


        turn
    }
}

