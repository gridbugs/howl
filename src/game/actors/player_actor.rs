use game::{
    Level,
    EntityId,
    ReserveEntityId,
    MetaAction,
};

use game::io::terminal_player_actor;

use terminal::InputSource;

pub struct PlayerActor<'a> {
    input_source: InputSource<'a>,
}

impl<'a> PlayerActor<'a> {
    pub fn new(input_source: InputSource<'a>) -> Self {
        PlayerActor {
            input_source: input_source,
        }
    }

    pub fn act(&self, level: &Level, id: EntityId,
               ids: &ReserveEntityId) -> MetaAction
    {
        loop {
            if let Some(meta_action) = terminal_player_actor::act(
                &self.input_source, level, id, ids)
            {
                return meta_action;
            }
        }
    }
}


