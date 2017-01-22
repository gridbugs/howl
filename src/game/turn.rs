use std::time::Duration;
use std::thread;
use std::cmp;
use std::cell::RefCell;
use std::ops::Deref;

use game::*;
use game::data::*;
use ecs::*;
use util::Schedule;

const FAILED_ACTION_DELAY: u64 = 16;
const MIN_TURN_TIME: u64 = 1;

pub const TURN_DURATION_BASE: u64 = 16;
pub const ENV_TURN_OFFSET: u64 = 0;
pub const NPC_TURN_OFFSET: u64 = 1;
pub const PC_TURN_OFFSET: u64 = 2;

#[derive(Clone, Copy)]
pub struct ActionEnv<'game> {
    pub ecs: &'game EcsCtx,
    pub id: u64,
}

#[derive(Clone, Copy)]
pub struct Turn<'game> {
    pub ecs: &'game EcsCtx,
    pub id: u64,
}

pub enum TurnResolution {
    Quit,
    Schedule(EntityId, u64),
}

impl TurnResolution {
    pub fn game_continues(&self) -> bool {
        match *self {
            TurnResolution::Quit => false,
            _ => true,
        }
    }
}

pub struct TurnEnv<'game, 'level: 'game, Renderer: 'game + KnowledgeRenderer> {
    pub turn_id: u64,
    pub action_id: &'game mut u64,
    pub level_id: LevelId,
    pub entity_id: EntityId,
    pub pc_id: EntityId,
    pub renderer: &'game RefCell<Renderer>,
    pub ecs: &'level mut EcsCtx,
    pub spatial_hash: &'level mut SpatialHashTable,
    pub behaviour_ctx: &'game BehaviourCtx<Renderer>,
    pub rule_reactions: &'game mut Vec<Reaction>,
    pub ecs_action: &'game mut EcsAction,
    pub action_schedule: &'game mut Schedule<ActionArgs>,
    pub turn_schedule: &'game mut Schedule<EntityId>,
    pub pc_observer: &'game Shadowcast,
    pub entity_ids: &'game EntityIdReserver,
    pub rng: &'game GameRng,
    pub language: &'game Box<Language>,
}

impl<'game> Turn<'game> {
    pub fn new(ecs: &'game EcsCtx, id: u64) -> Self {
        Turn {
            ecs: ecs,
            id: id,
        }
    }
}

impl<'game> ActionEnv<'game> {
    pub fn new(ecs: &'game EcsCtx, id: u64) -> Self {
        ActionEnv {
            ecs: ecs,
            id: id,
        }
    }
}

impl<'game, 'level, Renderer: KnowledgeRenderer> TurnEnv<'game, 'level, Renderer> {
    pub fn turn(&mut self) -> Result<TurnResolution> {

        self.pc_render(None)?;

        let resolution = self.take_turn()?;

        match resolution {
            TurnResolution::Schedule(id, old_delay) => {
                let new_delay = if let Some(delay) = self.process_transformation()? {
                    delay
                } else {
                    old_delay
                };

                Ok(TurnResolution::Schedule(id, new_delay))
            }
            other => Ok(other),
        }
    }

    fn process_transformation(&mut self) -> Result<Option<u64>> {
        if let Some(transformation) = self.get_transformation() {
            let action_args = transformation.to_action_args(self.entity_id);
            self.try_commit_action(action_args)?;

            let delay = self.ecs.turn_time(self.entity_id);
            return Ok(delay);
        }

        Ok(None)
    }

    fn get_transformation(&self) -> Option<TransformationType> {
        // The state of the cell in which an actor ends their turn determines
        // whether a transformation will occur.

        let entity = self.ecs.entity(self.entity_id);

        if let Some(position) = entity.position() {
            if let Some(transformation_state) = entity.transformation_state() {
                if self.spatial_hash.get(position).moon() {
                    if transformation_state == TransformationState::Real {
                        return entity.transformation_type();
                    }
                } else {
                    if transformation_state == TransformationState::Other {
                        return entity.transformation_type();
                    }
                }
            }
        }

        None
    }

    fn take_turn(&mut self) -> Result<TurnResolution> {
        loop {
            match self.get_meta_action()? {
                MetaAction::External(External::Quit) => return Ok(TurnResolution::Quit),
                MetaAction::ActionArgs(action_args) => {
                    if let Some(delay) = self.try_commit_action(action_args)? {
                        self.declare_action_return(true)?;
                        return Ok(TurnResolution::Schedule(self.entity_id, delay));
                    } else {
                        self.declare_action_return(false)?;
                        if self.is_pc_turn() {
                            continue;
                        } else {
                            return Ok(TurnResolution::Schedule(self.entity_id, FAILED_ACTION_DELAY));
                        }
                    }
                }
            }
        }
    }

    fn is_pc_turn(&self) -> bool {
        self.entity_id == self.pc_id
    }

    fn check_rules_wrapper(&mut self) -> Result<bool> {
        match self.check_rules() {
            Ok(()) => Ok(true),
            Err(RuleError::Rejection) => Ok(false),
            Err(RuleError::GameError(e)) => Err(e),
        }
    }

    fn check_rules(&mut self) -> RuleResult {
        self.rule_reactions.clear();

        let rule_env = RuleEnv {
            ecs: self.ecs,
            spatial_hash: self.spatial_hash,
        };

        rules::open_door(rule_env, self.ecs_action, self.rule_reactions)?;
        rules::collision(rule_env, self.ecs_action, self.rule_reactions)?;
        rules::close_door(rule_env, self.ecs_action, self.rule_reactions)?;
        rules::moon_transform(rule_env, self.ecs_action, self.rule_reactions)?;
        rules::realtime_velocity_start(rule_env, self.ecs_action, self.rule_reactions)?;
        rules::realtime_velocity(rule_env, self.ecs_action, self.rule_reactions)?;

        RULE_ACCEPT
    }

    fn commit(&mut self) {
        self.spatial_hash.update(ActionEnv::new(self.ecs, *self.action_id), self.ecs_action);
        self.ecs.commit(self.ecs_action);
    }

    fn pc_render(&mut self, action_description: Option<&ActionDescription>) -> Result<bool> {

        let entity = self.ecs.entity(self.pc_id);

        if !self.ecs.contains_should_render(self.entity_id) {
            return Ok(false);
        }

        let mut knowledge = entity.drawable_knowledge_borrow_mut().ok_or(Error::MissingComponent)?;
        let level_knowledge = knowledge.level_mut_or_insert_size(self.level_id,
                                                                 self.spatial_hash.width(),
                                                                 self.spatial_hash.height());
        let position = entity.position().ok_or(Error::MissingComponent)?;
        let vision_distance = entity.vision_distance().ok_or(Error::MissingComponent)?;
        let mut message_log = entity.message_log_borrow_mut().ok_or(Error::MissingComponent)?;


        let action_env = ActionEnv::new(self.ecs, *self.action_id);

        let mut changed = self.pc_observer.observe(position, self.spatial_hash, vision_distance, level_knowledge, action_env);

        if let Some(action_description) = action_description {
            if level_knowledge.can_see(action_description.coord, action_env) {
                message_log.add(action_description.message);
                changed = true;
            }
        }

        if changed {
            let mut renderer = self.renderer.borrow_mut();
            renderer.update_log(message_log.deref(), self.language);
            renderer.render(level_knowledge, *self.action_id, position);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn try_commit_action(&mut self, action: ActionArgs) -> Result<Option<u64>> {

        let mut turn_time = self.ecs.turn_time(self.entity_id);
        let mut first = true;
        let mut action_description = None;

        self.action_schedule.insert(action, 0);

        while let Some(action_event) = self.action_schedule.next() {

            // render the scene if time has passed
            if action_event.time_delta != 0 {
                if self.pc_render(action_description.as_ref())? {
                    // if the change in scene was visible, add a delay
                    thread::sleep(Duration::from_millis(action_event.time_delta));
                }
            }

            *self.action_id += 1;

            // construct an action from the action args
            action_event.event.to_action(&mut self.ecs_action, self.ecs, self.spatial_hash, self.entity_ids)?;

            let accept = self.check_rules_wrapper()?;

            let mut action_time = 0;

            if accept {
                if first {
                    first = false;
                    if let Some(alternative_turn_time) = self.ecs_action.alternative_turn_time() {
                        turn_time = Some(alternative_turn_time);
                    }
                }
                action_time = self.ecs_action.action_time_ms().unwrap_or(0);
                action_description = self.ecs_action.clear_action_description();

                self.commit();
            } else {
                // Committing the action clears its data.
                // It must be cleared explicitly if the action is rejected.
                self.ecs_action.clear();
            }

            for reaction in self.rule_reactions.drain(..) {
                self.action_schedule.insert(reaction.action, action_time + reaction.delay);
            }
        }

        if first {
            return Ok(None);
        }

        if action_description.is_some() {
            self.pc_render(action_description.as_ref())?;
        }

        Ok(turn_time.map(|t| cmp::max(t, MIN_TURN_TIME)))
    }

    fn get_meta_action(&self) -> Result<MetaAction> {
        let entity = self.ecs.entity(self.entity_id);
        let mut behaviour_state = entity.behaviour_state_borrow_mut().ok_or(Error::MissingComponent)?;
        if !behaviour_state.is_initialised() {
            let behaviour_type = entity.behaviour_type().ok_or(Error::MissingComponent)?;
            behaviour_state.initialise(self.behaviour_ctx.graph(), self.behaviour_ctx.nodes().index(behaviour_type))?;
        }
        let input = BehaviourInput {
            entity: entity,
            spatial_hash: self.spatial_hash,
            level_id: self.level_id,
            action_env: ActionEnv::new(self.ecs, *self.action_id),
            renderer: self.renderer,
            rng: self.rng,
            language: self.language,
        };
        Ok(behaviour_state.run(self.behaviour_ctx.graph(), input)?)
    }

    fn declare_action_return(&self, value: bool) -> Result<()> {
        let entity = self.ecs.entity(self.entity_id);
        let mut behaviour_state = entity.behaviour_state_borrow_mut().ok_or(Error::MissingComponent)?;
        behaviour_state.declare_return(value)?;
        Ok(())
    }
}
