use game::{
    Level,
    EntityId,
    ReserveEntityId,
    MetaAction,
    actions,
};

pub struct SimpleNpcActor {}

impl SimpleNpcActor {
    pub fn new() -> Self {
        SimpleNpcActor {}
    }

    pub fn act(&self, _: &Level, _: EntityId,
               _: &ReserveEntityId) -> MetaAction
    {
        MetaAction::Update(actions::wait())
    }
}
