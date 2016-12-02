use std::time::Duration;
use std::thread;

use game::*;
use ecs::*;
use util::Schedule;
use math::Coord;

const FAILED_ACTION_DELAY: u64 = 10;

#[derive(Clone, Copy)]
pub struct Turn<'a> {
    pub ecs: &'a EcsCtx,
    pub id: u64,
}

pub enum TurnResolution {
    Quit,
    Schedule(EntityId, u64),
}

pub struct TurnEnv<'a, 'b, 'c: 'a> {
    pub turn_id: u64,
    pub level_id: isize,
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
    pub pc_observer: &'a Shadowcast,
}

impl<'a> Turn<'a> {
    pub fn new(ecs: &'a EcsCtx, id: u64) -> Self {
        Turn {
            ecs: ecs,
            id: id,
        }
    }
}

const RENDER_WIDTH: usize = 37;
const RENDER_HEIGHT: usize = 26;

impl<'a, 'b, 'c: 'a> TurnEnv<'a, 'b, 'c> {
    pub fn turn(&mut self) -> Result<TurnResolution> {

        self.pc_render_ansi()?;

        match self.get_meta_action()? {
            MetaAction::External(External::Quit) => Ok(TurnResolution::Quit),
            MetaAction::ActionArgs(action_args) => {
                if let Some(delay) = self.try_commit_action(action_args)? {
                    self.declare_action_return(true)?;
                    Ok(TurnResolution::Schedule(self.entity_id, delay))
                } else {
                    self.declare_action_return(false)?;
                    Ok(TurnResolution::Schedule(self.entity_id, FAILED_ACTION_DELAY))
                }
            }
        }
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
        self.spatial_hash.update(Turn::new(self.ecs, self.turn_id), self.ecs_action);
        self.ecs.commit(self.ecs_action);
    }

    fn pc_render_ansi(&mut self) -> Result<bool> {
        let entity = self.ecs.entity(self.pc_id);
        let mut knowledge = entity.ansi_drawable_knowledge_borrow_mut().ok_or(Error::MissingComponent)?;
        let level_knowledge = knowledge.level_mut(self.level_id);
        let position = entity.position().ok_or(Error::MissingComponent)?;
        let vision_distance = entity.vision_distance().ok_or(Error::MissingComponent)?;
        let turn = Turn::new(self.ecs, self.turn_id);

        if self.pc_observer.observe(position, self.spatial_hash, vision_distance, level_knowledge, turn) {
            self.renderer.render(level_knowledge, self.turn_id, Coord::new(0, 0), RENDER_WIDTH, RENDER_HEIGHT);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn try_commit_action(&mut self, action: ActionArgs) -> Result<Option<u64>> {

        let mut turn_time = None;

        self.action_schedule.insert(action, 0);

        while let Some(action_event) = self.action_schedule.next() {

            // render the scene if time has passed
            if action_event.time_delta != 0 {
                if self.pc_render_ansi()? {
                    // if the change in scene was visible, add a delay
                    thread::sleep(Duration::from_millis(action_event.time_delta));
                }
            }

            // construct an action from the action args
            action_event.event.to_action(&mut self.ecs_action, self.ecs)?;

            self.check_rules()?;

            let mut action_time = 0;

            if self.rule_resolution.is_accept() {
                if turn_time.is_none() {
                    turn_time = Some(self.ecs_action.turn_time().unwrap_or(0));
                }
                action_time = self.ecs_action.action_time().unwrap_or(0);

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

        Ok(turn_time)
    }

    fn get_meta_action(&self) -> Result<MetaAction> {
        let entity = self.ecs.entity(self.entity_id);
        let mut behaviour_state = entity.behaviour_state_borrow_mut().ok_or(Error::MissingComponent)?;
        if !behaviour_state.is_initialised() {
            let behaviour_type = entity.behaviour_type().ok_or(Error::MissingComponent)?;
            behaviour_state.initialise(self.behaviour_ctx.graph(), self.behaviour_ctx.nodes().index(behaviour_type))?;
        }
        let input = BehaviourInput { entity: entity };
        Ok(behaviour_state.run(self.behaviour_ctx.graph(), input)?)
    }

    fn declare_action_return(&self, value: bool) -> Result<()> {
        let entity = self.ecs.entity(self.entity_id);
        let mut behaviour_state = entity.behaviour_state_borrow_mut().ok_or(Error::MissingComponent)?;
        behaviour_state.declare_return(value)?;
        Ok(())
    }
}
