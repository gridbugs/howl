use rand::Rng;
use action::ActionArgs;
use engine_defs::*;
use ecs_content::*;
use content_types::*;
use spatial_hash::*;

use game::actions;

pub type ActionId = u64;

#[derive(Debug, Clone, Copy)]
pub enum External {
    Pause,
    Quit,
}

#[derive(Debug, Clone, Copy)]
pub enum MetaAction {
    ActionArgs(ActionArgs),
    External(External),
}

pub fn args_to_action<R: Rng>(args: ActionArgs, action: &mut EcsAction, ecs: &EcsCtx, _spatial_hash: &SpatialHashTable, entity_ids: &EntityIdReserver, r: &mut R) {
    match args {
        ActionArgs::Null => (),
        ActionArgs::Walk(entity_id, direction) => {
            actions::walk(action, ecs.entity(entity_id), direction);
        }
    }
}
