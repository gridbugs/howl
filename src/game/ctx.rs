use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;

use game::*;
use game::data::*;
use ecs::*;
use util::{LeakyReserver, Schedule};
use coord::Coord;

pub struct EntityIdReserver(RefCell<LeakyReserver<EntityId>>);

impl EntityIdReserver {
    pub fn new() -> Self {
        EntityIdReserver(RefCell::new(LeakyReserver::new()))
    }

    pub fn new_id(&self) -> EntityId {
        self.0.borrow_mut().reserve()
    }
}

pub struct GameCtx<Renderer: KnowledgeRenderer, Input: InputSource> {
    levels: LevelTable,
    renderer: RefCell<Renderer>,
    input_source: Input,
    entity_ids: EntityIdReserver,
    turn_id: u64,
    action_id: u64,
    level_id: LevelId,
    pc_id: Option<EntityId>,
    pc_observer: Shadowcast,
    behaviour_ctx: BehaviourCtx<Renderer>,
    rule_reactions: Vec<Reaction>,
    ecs_action: EcsAction,
    action_schedule: Schedule<ActionArgs>,
    width: usize,
    height: usize,
    rng: GameRng,
    language: Box<Language>,
}

impl<Renderer: KnowledgeRenderer, Input: 'static + InputSource + Clone> GameCtx<Renderer, Input> {
    pub fn new(renderer: Renderer, input_source: Input, seed: usize, width: usize, height: usize) -> Self {
        GameCtx {
            levels: LevelTable::new(),
            renderer: RefCell::new(renderer),
            input_source: input_source.clone(),
            entity_ids: EntityIdReserver::new(),
            turn_id: 0,
            action_id: 0,
            level_id: 0,
            pc_id: None,
            pc_observer: Shadowcast::new(),
            behaviour_ctx: BehaviourCtx::new(input_source),
            rule_reactions: Vec::new(),
            ecs_action: EcsAction::new(),
            action_schedule: Schedule::new(),
            width: width,
            height: height,
            rng: GameRng::new(seed),
            language: Box::new(languages::English),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.init_demo();
        self.intro_message();
        self.welcome_message();
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
                    rule_reactions: &mut self.rule_reactions,
                    ecs_action: &mut self.ecs_action,
                    action_schedule: &mut self.action_schedule,
                    turn_schedule: &mut level.turn_schedule,
                    pc_observer: &self.pc_observer,
                    entity_ids: &self.entity_ids,
                    rng: &self.rng,
                    language: &self.language,
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
        level.spatial_hash.update(&level.ecs, action, self.action_id);
        level.ecs.commit(action);
    }

    fn welcome_message(&self) {
        let ref ecs = self.levels.level(self.level_id).ecs;
        ecs.message_log_borrow_mut(self.pc_id.unwrap()).unwrap().add(MessageType::Welcome);
    }

    fn intro_message(&mut self) {
        let ref ecs = self.levels.level(self.level_id).ecs;
        let pc = ecs.entity(self.pc_id.unwrap());
        let control_map_ref = pc.control_map_borrow().unwrap();
        let mut message = Message::new();

        self.language.translate(MessageType::Intro, &mut message);
        message.push(MessagePart::Newline);
        message.push(MessagePart::Newline);
        self.language.translate(MessageType::PressAnyKey, &mut message);
        message.push(MessagePart::Newline);
        message.push(MessagePart::Newline);
        message.push(MessagePart::Newline);
        self.language.translate_controls(control_map_ref.deref(), &mut message);

        display_message_scrolling(self.renderer.borrow_mut().deref_mut(), &mut self.input_source, &message, true);
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
                        prototypes::floor(g.entity_mut(self.new_id()), coord);
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
            prototypes::clouds(g.entity_mut(cloud_id), self.width, self.height, self.rng.gen_usize());
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
         "&,,#.........#,,,,t&,,,,,,,,&,,,,,,,,&",
         "&,,#.........+,,,,,,&,,,,,,,,,,,,,,,,&",
         "&&,#.........#,,,,,&,,,,,,,,,&,,,,,,,&",
         "&,,#.........#,,,,,,,,,,&,,&,,,&,&,,,&",
         "&,&#.........#,,,,@,,,,&,,,,,,,,,,,,,&",
         "&,,###########,t,,,,,&,,,,,,,&,&,,,,,&",
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


