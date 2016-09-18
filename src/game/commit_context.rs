use game::{
    UpdateSummary,
    Level,
    Rule,
    EntityId,
    Renderer,
    ReserveEntityId,
    MetadataWrapper,
};

use schedule::Schedule;

use std::thread;
use std::time::Duration;

pub struct CommitContext {
    update_queue: Schedule<UpdateSummary>,
}

pub enum CommitError {
    NoCommits,
}


pub struct CommitTime {
    pub turn: u64,
    pub time: u64,
}

impl CommitTime {
    fn new(turn: u64, time: u64) -> Self {
        CommitTime {
            turn: turn,
            time: time,
        }
    }
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
        initial_turn: u64) -> Result<CommitTime, CommitError>
    {
        let mut turn = initial_turn;

        // record the time of the first successful update
        let mut first_time = None;

        // start with the initial action in the queue
        self.update_queue.insert(initial_update, 0);

        while let Some(update) = self.update_queue.next() {

            // render the scene if time has passed
            if update.time_delta != 0 {
                if let Some((id, ref mut r)) = renderer {
                    if r.render(level, id, turn) {
                        // only delay if something changed
                        thread::sleep(Duration::from_millis(update.time_delta));
                    }
                }
            }

            // check the update against all rules
            let mut result = level.check_rules(&update.event, rules, ids);

            let mut action_time = 0;

            if result.is_accept() {
                let metadata = level.commit_update(update.event, turn);

                if first_time.is_none() {
                    first_time = Some(metadata.turn_time());
                }

                action_time = metadata.action_time();

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

        if let Some(time) = first_time {
            Ok(CommitTime::new(turn, time))
        } else {
            Err(CommitError::NoCommits)
        }
    }
}

