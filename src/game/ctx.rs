use std::cell::RefCell;

use game::*;
use game::data::*;
use ecs::*;
use util::{LeakyReserver, Schedule};
use math::Coord;

pub struct EntityIdReserver(RefCell<LeakyReserver<EntityId>>);

impl EntityIdReserver {
    pub fn new() -> Self {
        EntityIdReserver(RefCell::new(LeakyReserver::new()))
    }

    pub fn new_id(&self) -> EntityId {
        self.0.borrow_mut().reserve()
    }
}

pub struct GameCtx {
    levels: LevelTable,
    renderer: RefCell<Box<KnowledgeRenderer>>,
    input_source: InputSourceRef,
    entity_ids: EntityIdReserver,
    turn_id: u64,
    action_id: u64,
    level_id: LevelId,
    pc_id: Option<EntityId>,
    pc_observer: Shadowcast,
    behaviour_ctx: BehaviourCtx,
    rules: Vec<Box<Rule>>,
    rule_resolution: RuleResolution,
    ecs_action: EcsAction,
    action_schedule: Schedule<ActionArgs>,
    width: usize,
    height: usize,
}

impl GameCtx {
    pub fn new(renderer: Box<KnowledgeRenderer>, input_source: InputSourceRef, width: usize, height: usize) -> Self {
        GameCtx {
            levels: LevelTable::new(),
            renderer: RefCell::new(renderer),
            input_source: input_source,
            entity_ids: EntityIdReserver::new(),
            turn_id: 0,
            action_id: 0,
            level_id: 0,
            pc_id: None,
            pc_observer: Shadowcast::new(),
            behaviour_ctx: BehaviourCtx::new(input_source),
            rules: Vec::new(),
            rule_resolution: RuleResolution::new(),
            ecs_action: EcsAction::new(),
            action_schedule: Schedule::new(),
            width: width,
            height: height,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.rules.push(Box::new(rules::OpenDoor));
        self.rules.push(Box::new(rules::Collision));
        self.rules.push(Box::new(rules::RealtimeAxisVelocity));
        self.rules.push(Box::new(rules::RealtimeAxisVelocityStart));
        self.rules.push(Box::new(rules::CloseDoor));
        self.rules.push(Box::new(rules::BurstFire));
        self.rules.push(Box::new(rules::MoonTransform));

        self.init_demo();

        self.game_loop()
    }

    fn game_loop(&mut self) -> Result<()> {
        loop {

            self.turn_id += 1;

            let level = self.levels.level_mut(self.level_id);
            if let Some(turn_event) = level.turn_schedule.next() {

                let resolution = TurnEnv {
                    turn_id: self.turn_id,
                    action_id: &mut self.action_id,
                    level_id: self.level_id,
                    entity_id: turn_event.event,
                    pc_id: self.pc_id.unwrap(),
                    renderer: &self.renderer,
                    ecs: &mut level.ecs,
                    spatial_hash: &mut level.spatial_hash,
                    behaviour_ctx: &self.behaviour_ctx,
                    rules: &self.rules,
                    rule_resolution: &mut self.rule_resolution,
                    ecs_action: &mut self.ecs_action,
                    action_schedule: &mut self.action_schedule,
                    turn_schedule: &mut level.turn_schedule,
                    pc_observer: &self.pc_observer,
                    entity_ids: &self.entity_ids,
                }.turn()?;

                match resolution {
                    TurnResolution::Quit => return Ok(()),
                    TurnResolution::Schedule(entity_id, delay) => {
                        let ticket = level.turn_schedule.insert(entity_id, delay);
                        level.ecs.insert_schedule_ticket(turn_event.event, ticket);
                    }
                }
            } else {
                return Err(Error::ScheduleEmpty);
            }
        }
    }

    fn new_id(&self) -> EntityId {
        self.entity_ids.new_id()
    }

    fn commit(&mut self, action: &mut EcsAction) {
        let level = self.levels.level_mut(self.level_id);
        level.spatial_hash.update(ActionEnv::new(&level.ecs, self.action_id), action);
        level.ecs.commit(action);
    }

    fn init_demo(&mut self) {
        let strings = demo_level_str();

        let mut level = Level::new(self.width, self.height);

        let mut g = EcsAction::new();

        let mut y = 0;
        for line in &strings {
            let mut x = 0;
            for ch in line.chars() {
                let coord = Coord::new(x, y);
                match ch {
                    '#' => {
                        prototypes::wall(g.entity_mut(self.new_id()), coord);
                        prototypes::floor(g.entity_mut(self.new_id()), coord);
                    }
                    '&' => {
                        prototypes::tree(&mut g, &self.entity_ids, coord);

                        prototypes::outside_floor(g.entity_mut(self.new_id()), coord);
                    }
                    '.' => {
                        prototypes::floor(g.entity_mut(self.new_id()), coord);
                    }
                    ',' => {
                        prototypes::outside_floor(g.entity_mut(self.new_id()), coord);
                    }
                    '+' => {
                        prototypes::door(g.entity_mut(self.new_id()), coord, DoorState::Closed);
                    }
                    '@' => {
                        let id = self.new_id();
                        self.pc_id = Some(id);
                        prototypes::pc(g.entity_mut(id), coord);
                        prototypes::outside_floor(g.entity_mut(self.new_id()), coord);

                        let ticket = level.turn_schedule.insert(id, PC_TURN_OFFSET);
                        g.insert_schedule_ticket(id, ticket);
                    }
                    't' => {
                        prototypes::outside_floor(g.entity_mut(self.new_id()), coord);
                        let id = prototypes::terror_pillar(&mut g, &self.entity_ids, coord);

                        let ticket = level.turn_schedule.insert(id, NPC_TURN_OFFSET);
                        g.insert_schedule_ticket(id, ticket);
                    }
                    _ => panic!(),
                }
                x += 1;
            }
            y += 1;
        }

        {
            let cloud_id = self.new_id();
            prototypes::clouds(g.entity_mut(cloud_id), self.width, self.height);
            let ticket = level.turn_schedule.insert(cloud_id, ENV_TURN_OFFSET);
            self.levels.add_level(level);
            g.insert_schedule_ticket(cloud_id, ticket);
        }

        self.commit(&mut g);
    }
}

fn demo_level_str() -> Vec<&'static str> {
    vec!["&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&",
         "&,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,&",
         "&,,############################,,,,,,&",
         "&,,#.........#................#,,&,,,&",
         "&,,#.........#................#,,,&,,&",
         "&,,#..........................#,,&,,,&",
         "&&,#.........#................#,,,,,,&",
         "&,&#.........##########+#######,,,,,,&",
         "&,,#.........#,,,,,,,,,,,,,,,,,,,,,,,&",
         "&&,#.........#,t,,,,,,,&,,,,,,,&,&,&,&",
         "&,,#.........#,,,,,&,,,,,,,,&,,,,,,,,&",
         "&,,#.........+,,,,,,&,,,,,,,,,,,,,,,,&",
         "&&,#.........#,,,,,&,,,,,,,,,&,,,,,,,&",
         "&,,#.........#,,,,,,,,,,&,,&,,,&,&,,,&",
         "&,&#.........#,,,,@,,,,&,,,,,,,,,,,,,&",
         "&,,###########,,,,,,,&,,,,,,,&,&,,,,,&",
         "&,,&,,,,,,,,,,,,,,,,,&,,,,&,,,,,,,,,,&",
         "&,&,,,,,,,,,,,,&,,,,,,,,,,,,,,,,,,,,,&",
         "&,,,&,,,,,,,,,,,,,,,,&,,,,,#########,&",
         "&,&,,,&,,,,,&,,&,,,,&,,,,,,#.......#,&",
         "&,,,,,&,,,,,,,,,&,,,,&,,,,,#.......#,&",
         "&,,,,,,,,,&,,,,,,,,,,,,,&,,........#,&",
         "&,&,&,,,,&&,,,&,&,,,,,,,&,,#.......#,&",
         "&,,,,,,,,,,,,,,,,,,,&,,,,,,#.......#,&",
         "&,,,&,,,,,,,&,,,,,,,,,,,,,,#########,&",
         "&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&"]
}


