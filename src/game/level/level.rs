use game::*;
use ecs::*;
use spatial_hash::*;

pub struct Level {
    pub ecs: EcsCtx,
    pub spatial_hash: SpatialHashTable,
    pub turn_schedule: TurnSchedule,
}

impl Level {
    pub fn new_with_pc(terrain: TerrainType,
                       pc_id: EntityId,
                       action: &mut EcsAction,
                       ids: &EntityIdReserver,
                       rng: &GameRng,
                       action_id: u64) -> Self {

        let mut schedule = TurnSchedule::new();

        let md = terrain.generate(ids, rng, &mut schedule, action);

        let mut sh = SpatialHashTable::new(md.width, md.height);
        let mut ecs = EcsCtx::new();

        action.insert_position(pc_id, md.start_coord);

        let pc_ticket = schedule.insert(pc_id, NPC_TURN_OFFSET);
        action.insert_schedule_ticket(pc_id, pc_ticket);

        sh.update(&ecs, &action, action_id);

        ecs.commit(action);

        Level {
            ecs: ecs,
            spatial_hash: sh,
            turn_schedule: schedule,
        }
    }
}
