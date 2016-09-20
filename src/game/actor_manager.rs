use game::{Level, EntityId, MetaAction, EntityStore, EntityWrapper, ReserveEntityId, ActorType};

use game::actors::{PlayerActor, SimpleNpcActor};

use terminal::InputSource;

pub struct ActorManager<'a> {
    player_actor: PlayerActor<'a>,
    simple_npc_actor: SimpleNpcActor,
}

impl<'a> ActorManager<'a> {
    pub fn new(input_source: InputSource<'a>) -> Self {
        ActorManager {
            player_actor: PlayerActor::new(input_source),
            simple_npc_actor: SimpleNpcActor::new(),
        }
    }

    pub fn act(&self, level: &Level, id: EntityId, ids: &ReserveEntityId) -> MetaAction {
        if let Some(actor_type) = level.get(id).unwrap().actor_type() {
            match actor_type {
                ActorType::Player => self.player_actor.act(level, id, ids),
                ActorType::SimpleNpc => self.simple_npc_actor.act(level, id, ids),
            }
        } else {
            MetaAction::NotActor
        }
    }
}
