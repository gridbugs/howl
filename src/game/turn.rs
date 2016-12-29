use std::time::Duration;
use std::thread;
use std::cmp;

use game::*;
use ecs::*;
use util::Schedule;

const FAILED_ACTION_DELAY: u64 = 16;
const MIN_TURN_TIME: u64 = 1;

pub const TURN_DURATION_BASE: u64 = 16;
pub const ENV_TURN_OFFSET: u64 = 0;
pub const NPC_TURN_OFFSET: u64 = 1;
pub const PC_TURN_OFFSET: u64 = 2;

#[derive(Clone, Copy)]
pub struct ActionEnv<'a> {
    pub ecs: &'a EcsCtx,
    pub id: u64,
}

#[derive(Clone, Copy)]
pub struct Turn<'a> {
    pub ecs: &'a EcsCtx,
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

pub struct TurnEnv<'a, 'b, 'c: 'a> {
    pub turn_id: u64,
    pub action_id: &'a mut u64,
    pub level_id: LevelId,
    pub entity_id: EntityId,
    pub pc_id: EntityId,
    pub renderer: &'a mut AnsiRenderer<'c>,
    pub ecs: &'b mut EcsCtx,
    pub spatial_hash: &'b mut SpatialHashTable,
    pub behaviour_ctx: &'a BehaviourCtx,
    pub rules: &'a Vec<Box<Rule>>,
    pub rule_resolution: &'a mut RuleResolution,
    pub ecs_action: &'a mut EcsAction,
    pub action_schedule: &'a mut Schedule<ActionArgs>,
    pub turn_schedule: &'a mut Schedule<EntityId>,
    pub pc_observer: &'a Shadowcast,
    pub entity_ids: &'a EntityIdReserver,
}

impl<'a> Turn<'a> {
    pub fn new(ecs: &'a EcsCtx, id: u64) -> Self {
        Turn {
            ecs: ecs,
            id: id,
        }
    }
}

impl<'a> ActionEnv<'a> {
    pub fn new(ecs: &'a EcsCtx, id: u64) -> Self {
        ActionEnv {
            ecs: ecs,
            id: id,
        }
    }
}

impl<'a, 'b, 'c: 'a> TurnEnv<'a, 'b, 'c> {
    pub fn turn(&mut self) -> Result<TurnResolution> {

        self.pc_render_ansi()?;

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

    fn check_rules(&mut self) -> Result<()> {
        self.rule_resolution.reset();

        let rule_env = RuleEnv {
            ecs: self.ecs,
            spatial_hash: self.spatial_hash,
        };

        for rule in self.rules {
            rule.check(rule_env, self.ecs_action, self.rule_resolution)?;

            if self.rule_resolution.is_reject() {
                return Ok(())
            }
        }
        Ok(())
    }

    fn commit(&mut self) {
        self.spatial_hash.update(ActionEnv::new(self.ecs, *self.action_id), self.ecs_action);
        self.ecs.commit(self.ecs_action);
    }

    fn pc_render_ansi(&mut self) -> Result<bool> {

        let entity = self.ecs.entity(self.pc_id);

        if !entity.contains_should_render() {
            return Ok(false);
        }

        let mut knowledge = entity.drawable_knowledge_borrow_mut().ok_or(Error::MissingComponent)?;
        let level_knowledge = knowledge.level_mut_or_insert_size(self.level_id,
                                                                 self.spatial_hash.width(),
                                                                 self.spatial_hash.height());
        let position = entity.position().ok_or(Error::MissingComponent)?;
        let vision_distance = entity.vision_distance().ok_or(Error::MissingComponent)?;
        let action_env = ActionEnv::new(self.ecs, *self.action_id);

        if self.pc_observer.observe(position, self.spatial_hash, vision_distance, level_knowledge, action_env) {
            self.renderer.render(level_knowledge, *self.action_id, position);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn try_commit_action(&mut self, action: ActionArgs) -> Result<Option<u64>> {

        let mut turn_time = self.ecs.turn_time(self.entity_id);
        let mut first = true;

        self.action_schedule.insert(action, 0);

        while let Some(action_event) = self.action_schedule.next() {

            // render the scene if time has passed
            if action_event.time_delta != 0 {
                if self.pc_render_ansi()? {
                    // if the change in scene was visible, add a delay
                    thread::sleep(Duration::from_millis(action_event.time_delta));
                }
            }

            *self.action_id += 1;

            // construct an action from the action args
            action_event.event.to_action(&mut self.ecs_action, self.ecs, self.spatial_hash, self.entity_ids)?;

            self.check_rules()?;

            let mut action_time = 0;

            if self.rule_resolution.is_accept() {
                if first {
                    first = false;
                    if let Some(alternative_turn_time) = self.ecs_action.alternative_turn_time() {
                        turn_time = Some(alternative_turn_time);
                    }
                }
                action_time = self.ecs_action.action_time_ms().unwrap_or(0);

                self.commit();
            } else {
                // Committing the action clears its data.
                // It must be cleared explicitly if the action is rejected.
                self.ecs_action.clear();
            }

            for reaction in self.rule_resolution.drain_reactions() {
                self.action_schedule.insert(reaction.action, action_time + reaction.delay);
            }
        }

        if first {
            return Ok(None);
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
