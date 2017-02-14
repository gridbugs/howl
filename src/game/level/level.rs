use game::*;
use ecs::*;
use spatial_hash::*;
use util::*;

pub struct Level {
    pub ecs: EcsCtx,
    pub spatial_hash: SpatialHashTable,
    pub turn_schedule: TurnSchedule,
}

#[derive(Serialize, Deserialize)]
pub struct SerializableLevel {
    ecs: SerializableEcsCtx,
    spatial_hash: SpatialHashTable,
    turn_schedule: SerializableSchedule<EntityId>,
}

impl From<Level> for SerializableLevel {
    fn from(level: Level) -> Self {
        let Level { ecs, spatial_hash, turn_schedule } = level;
        SerializableLevel {
            ecs: SerializableEcsCtx::from(ecs),
            spatial_hash: spatial_hash,
            turn_schedule: SerializableSchedule::from(turn_schedule),
        }
    }
}

impl From<SerializableLevel> for Level {
    fn from(level: SerializableLevel) -> Self {
        let SerializableLevel { ecs, spatial_hash, turn_schedule } = level;
        Level {
            ecs: EcsCtx::from(ecs),
            spatial_hash: spatial_hash,
            turn_schedule: TurnSchedule::from(turn_schedule),
        }
    }
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

    pub fn commit(&mut self, action: &mut EcsAction, action_id: u64) {
        self.spatial_hash.update(&self.ecs, action, action_id);
        self.ecs.commit(action);
    }

    pub fn commit_into(&mut self, from: &mut EcsAction, to: &mut EcsAction, action_id: u64) {
        self.spatial_hash.update(&self.ecs, from, action_id);
        self.ecs.commit_into(from, to);
    }
}
