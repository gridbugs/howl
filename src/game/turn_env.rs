use std::cell::RefCell;
use std::time::Duration;
use std::thread;

use game::*;
use ecs::*;
use frontends::ansi;
use util::{LeakyReserver, Schedule};
use math::Coord;


struct TurnEnv<'a, 'b, 'c: 'a> {
    turn_id: u64,
    entity_id: EntityId,
    pc_id: EntityId,
    renderer: &'a mut AnsiRenderer<'c>,
    ecs: &'b mut EcsCtx,
    spatial_hash: &'b mut SpatialHashTable,
    behaviour_ctx: &'a BehaviourCtx,
    rules: &'a Vec<Box<Rule>>,
    rule_resolution: &'a mut RuleResolution,
    ecs_action: &'a mut EcsAction,
    action_schedule: &'a mut Schedule<ActionArgs>,
}

impl<'a, 'b, 'c: 'a> TurnEnv<'a, 'b, 'c> {
    pub fn turn(&mut self) -> Result<TurnResolution> {
        unimplemented!()
    }
}


