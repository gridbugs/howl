use game::{Level, EntityId, MetaAction, EntityStore, EntityWrapper, ReserveEntityId, ActorType};

use game::actors::{PlayerActor, SimpleNpcActor};

use terminal::InputSource;

pub struct ActorManager {
    player_actor: PlayerActor,
    simple_npc_actor: SimpleNpcActor,
}

impl ActorManager {
    pub fn new(input_source: InputSource) -> Self {
        ActorManager {
            player_actor: PlayerActor::new(input_source),
            simple_npc_actor: SimpleNpcActor::new(),
        }
    }

    pub fn act(&mut self,
               level: &mut Level,
               id: EntityId,
               ids: &ReserveEntityId,
               turn: u64)
               -> MetaAction {

        if let Some(actor_type) = level.get(id).unwrap().actor_type() {
            match actor_type {
                ActorType::Player => self.player_actor.act(level, id, ids),
                ActorType::SimpleNpc => self.simple_npc_actor.act(level, id, ids, turn),
            }
        } else {
            MetaAction::NotActor
        }
    }
}
