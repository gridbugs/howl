use std::cell::RefCell;

use game::{LevelTable, Turn, Shadowcast, AnsiRenderer, BehaviourCtx, Result, Error,
           MetaAction, BehaviourInput, Control};
use frontends::ansi;
use util::LeakyReserver;
use ecs::{self, EntityId, EcsAction};
use math::Coord;

use debug;

enum TurnResolution {
    Quit,
    Reschedule(u64),
}

pub struct GameCtx<'a> {
    levels: LevelTable,
    renderer: AnsiRenderer<'a>,
    input_source: ansi::InputSource,
    entity_ids: RefCell<LeakyReserver<EntityId>>,
    turn_id: u64,
    level_id: isize,
    pc_id: Option<EntityId>,
    pc_observer: Shadowcast,
    behaviour_ctx: BehaviourCtx,
}

impl<'a> GameCtx<'a> {
    pub fn new(window: ansi::Window<'a>, input_source: ansi::InputSource) -> Self {
        GameCtx {
            levels: LevelTable::new(),
            renderer: AnsiRenderer::new(window),
            input_source: input_source,
            entity_ids: RefCell::new(LeakyReserver::new()),
            turn_id: 0,
            level_id: 0,
            pc_id: None,
            pc_observer: Shadowcast::new(),
            behaviour_ctx: BehaviourCtx::new(input_source),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.init_demo();

        self.game_loop()
    }

    fn get_meta_action(&self, entity_id: EntityId) -> Result<MetaAction> {
        let entity = self.levels.level(self.level_id).ecs.entity(entity_id);
        let mut behaviour_state = entity.behaviour_state_borrow_mut().ok_or(Error::MissingComponent)?;
        if !behaviour_state.is_initialised() {
            debug_println!("Initialising behaviour state for entity {}.", entity_id);
            let behaviour_type = entity.behaviour_type().ok_or(Error::MissingComponent)?;
            behaviour_state.initialise(self.behaviour_ctx.graph(), self.behaviour_ctx.nodes().index(behaviour_type))?;
        }
        let input = BehaviourInput { entity: entity };
        Ok(behaviour_state.run(self.behaviour_ctx.graph(), input)?)
    }

    fn declare_action_return(&self, entity_id: EntityId, value: bool) -> Result<()> {
        let entity = self.levels.level(self.level_id).ecs.entity(entity_id);
        let mut behaviour_state = entity.behaviour_state_borrow_mut().ok_or(Error::MissingComponent)?;
        behaviour_state.declare_return(value)?;
        Ok(())
    }

    fn game_turn(&mut self, entity_id: EntityId) -> Result<TurnResolution> {

        let meta_action = self.get_meta_action(entity_id)?;

        match meta_action {
            MetaAction::Control(Control::Quit) => Ok(TurnResolution::Quit),
            MetaAction::ActionArgs(action_args) => {
                debug_println!("{:?}", action_args);
                self.declare_action_return(entity_id, true)?;
                Ok(TurnResolution::Reschedule(1))
            }
        }
    }

    fn game_loop(&mut self) -> Result<()> {
        while let Some(turn_event) = self.levels.level_mut(self.level_id).turn_schedule.next() {
            let entity_id = turn_event.event;
            let resolution = self.game_turn(entity_id)?;
            match resolution {
                TurnResolution::Quit => return Ok(()),
                TurnResolution::Reschedule(delay) => {
                    self.levels.level_mut(self.level_id).turn_schedule.insert(entity_id, delay);
                }
            }
        }

        Err(Error::ScheduleEmpty)
    }

    fn new_id(&self) -> EntityId {
        self.entity_ids.borrow_mut().reserve()
    }

    fn commit(&mut self, action: &mut EcsAction) {
        let level = self.levels.level_mut(self.level_id);
        level.spatial_hash.update(Turn::new(&level.ecs, self.turn_id), action);
        level.ecs.commit(action);
    }

    fn pc_render_ansi(&mut self) {
        self.pc_observe_ansi();
        self.pc_draw_ansi(Coord::new(0, 0), 37, 26);
    }

    fn pc_observe_ansi(&self) {
        let level = self.levels.level(self.level_id);
        let entity = level.ecs.entity(self.pc_id.unwrap());
        let mut knowledge = entity.ansi_drawable_knowledge_borrow_mut().unwrap();
        let level_knowledge = knowledge.level_mut(self.level_id);
        let position = entity.position().unwrap();
        let vision_distance = entity.vision_distance().unwrap();
        let turn = Turn::new(&level.ecs, self.turn_id);
        self.pc_observer.observe(position, &level.spatial_hash, vision_distance, level_knowledge, turn);
    }

    fn pc_draw_ansi(&mut self, top_left: Coord, width: usize, height: usize) {
        let level = self.levels.level(self.level_id);
        let entity = level.ecs.entity(self.pc_id.unwrap());
        let knowledge = entity.ansi_drawable_knowledge_borrow().unwrap();
        let level_knowledge = knowledge.level(self.level_id);
        self.renderer.render(level_knowledge, self.turn_id, top_left, width, height);
    }

    fn init_demo(&mut self) {
        let strings = demo_level_str();

        let mut g = EcsAction::new();

        let mut y = 0;
        for line in &strings {
            let mut x = 0;
            for ch in line.chars() {
                let coord = Coord::new(x, y);
                match ch {
                    '#' => {
                        ecs::prototypes::wall(g.entity_mut(self.new_id()), coord);
                        ecs::prototypes::floor(g.entity_mut(self.new_id()), coord);
                    }
                    '&' => {
                        ecs::prototypes::tree(g.entity_mut(self.new_id()), coord);
                        ecs::prototypes::outside_floor(g.entity_mut(self.new_id()), coord);
                    }
                    '.' => {
                        ecs::prototypes::floor(g.entity_mut(self.new_id()), coord);
                    }
                    ',' => {
                        ecs::prototypes::outside_floor(g.entity_mut(self.new_id()), coord);
                    }
                    '@' => {
                        let id = self.new_id();
                        self.pc_id = Some(id);
                        ecs::prototypes::pc(g.entity_mut(id), coord);
                        ecs::prototypes::outside_floor(g.entity_mut(self.new_id()), coord);
                    }
                    _ => panic!(),
                }
                x += 1;
            }
            y += 1;
        }

        self.commit(&mut g);
        self.levels.level_mut(self.level_id).turn_schedule.insert(self.pc_id.unwrap(), 0);
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
         "&,&#.........##########.#######,,,,,,&",
         "&,,#.........#,,,,,,,,,,,,,,,,,,,,,,,&",
         "&&,#.........#,,,,,,,,,&,,,,,,,&,&,&,&",
         "&,,#.........#,,,,,&,,,,,,,,&,,,,,,,,&",
         "&,,#..........,,,,,,&,,,,,,,,,,,,,,,,&",
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


