use game::{
    Level,
    EntityId,
    MetaAction,
    ReserveEntityId,
};

use game::actors::{
    PlayerActor,
};

use terminal::InputSource;

pub struct ActorManager<'a> {
    player_actor: PlayerActor<'a>,
}

impl<'a> ActorManager<'a> {
    pub fn new(input_source: InputSource<'a>) -> Self {
        ActorManager {
            player_actor: PlayerActor::new(input_source),
        }
    }

    pub fn act(&self, level: &Level, id: EntityId,
               ids: &ReserveEntityId) -> MetaAction
    {
        self.player_actor.act(level, id, ids)
    }
}


