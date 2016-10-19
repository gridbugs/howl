use game::{UpdateSummary, MetaAction, Rule, EntityContext, LevelStore, EntityId, Level, LevelId,
           ComponentType, Component, actions, EntityWrapper, EntityStore, CommitContext, CommitError,
           Renderer, ActorManager, LevelEntityRef, EntityRef, EntityRefAccessMut};
use game::components::Form;
use game::behaviour as game_behaviour;

use game::io::WindowKnowledgeRenderer;
use game::observer::DrawableObserver;

use terminal::{Window, InputSource};
use behaviour;

use debug;

const NPC_INVALID_ACTION_DELAY: u64 = 10;

fn cloud_progress(level: &mut Level, time: u64) -> UpdateSummary {
    level.update_clouds_action(time)
}

fn transformation(id: EntityId, level: &Level, time: u64) -> Option<UpdateSummary> {
    let time = time as isize;
    let entity = level.get(id).unwrap();
    if let Some(form) = entity.form() {
        if let Some(position) = entity.position() {
            let sh = level.spatial_hash();
            let sh_cell = sh.get((position.x, position.y)).unwrap();
            if sh_cell.has(ComponentType::Moon) {
                if form == Form::Human {
                    return Some(actions::beast_transform_progress(entity, -time));
                } else {
                    return Some(actions::human_transform_progress(entity, time));
                }
            } else {
                if form == Form::Human {
                    return Some(actions::beast_transform_progress(entity, time));
                } else {
                    return Some(actions::human_transform_progress(entity, -time));
                }
            }
        }
    }

    None
}

type BehaviourContext<'a> = game_behaviour::BehaviourContext<LevelEntityRef<'a>>;

pub struct GameContext<'a> {
    pub entities: EntityContext,
    pub pc: Option<EntityId>,
    pc_level_id: LevelId,

    // io
    renderer: Renderer<'a>,

    // rule application
    commit_context: CommitContext,
    rules: Vec<Box<Rule>>,

    // actors
    actors: ActorManager<'a>,

    // time
    turn: u64,

    // behaviour
    behaviour_context: BehaviourContext<'a>,
}

#[derive(Debug)]
enum TurnError {
    Quit,
    NotActor,
}

impl<'a> GameContext<'a> {
    pub fn new(input_source: InputSource<'a>, game_window: Window<'a>) -> Self {
        GameContext {
            entities: EntityContext::new(),
            pc: None,
            pc_level_id: 0,
            actors: ActorManager::new(input_source),
            renderer: Renderer::new(DrawableObserver::new(),
                                    WindowKnowledgeRenderer::new(game_window)),
            commit_context: CommitContext::new(),
            rules: Vec::new(),
            turn: 0,
            behaviour_context: BehaviourContext::new(),
        }
    }

    pub fn rule<R: 'static + Rule>(&mut self, r: R) -> &mut Self {
        self.rules.push(Box::new(r));

        self
    }

    fn game_turn(&mut self) -> Result<(), TurnError> {

        self.turn += 1;

        let mut level = self.entities.levels.level_mut(self.pc_level_id).unwrap();
        let ids = &self.entities.entity_ids;

        let turn = level.schedule.next().expect("schedule is empty");
        let entity_id = turn.event;

        Self::lazy_init_behaviour(level.get_mut(entity_id).unwrap(), &self.behaviour_context);

        // update cloud positions, bypassing rules
        if level.get(entity_id).unwrap().is_pc() {
            let cloud_update = cloud_progress(level, turn.time_queued);
            level.commit_update(cloud_update, self.turn);
            self.turn += 1;
        }

        // apply transformation system
        if let Some(transform_update) = transformation(entity_id, level, turn.time_queued) {
            if let Ok(commit_time) = self.commit_context
                .apply_update(level, transform_update, &self.rules, None, ids, self.turn) {
                self.turn = commit_time.turn;
            }
        }

        self.renderer.render(level, self.pc.unwrap(), self.turn);

        loop {
            match self.actors.act(level, entity_id, ids, self.turn) {
                MetaAction::Quit => return Err(TurnError::Quit),
                MetaAction::NotActor => return Err(TurnError::NotActor),
                MetaAction::PassTurn => break,
                MetaAction::Update(update) => {
                    match self.commit_context.apply_update(level,
                                                           update,
                                                           &self.rules,
                                                           Some((self.pc.unwrap(),
                                                                 &mut self.renderer)),
                                                           ids,
                                                           self.turn) {
                        Ok(commit_time) => {

                            self.turn = commit_time.turn;
                            level.schedule.insert(entity_id, commit_time.time);

                            break;
                        }
                        Err(CommitError::NoCommits) => {
                            if level.get(entity_id).unwrap().is_pc() {
                                // the player can retry their turn
                                continue;
                            } else {
                                // An npc has made an illegal action.
                                // Reschedule their turn in the future
                                // to prevent them from blocking the
                                // player from acting indefinitely.

                                level.schedule.insert(entity_id, NPC_INVALID_ACTION_DELAY);

                                debug_println!("Illegal action by {}", entity_id);

                                break;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn game_loop(&mut self) {
        loop {
            if let Err(err) = self.game_turn() {
                match err {
                    TurnError::Quit => break,
                    TurnError::NotActor => {
                        debug_println!("Turn given tot non-actor!");
                    }
                }
            }
        }
    }

    fn lazy_init_behaviour<'b, E: EntityRefAccessMut<'b>>(entity: E, ctx: &BehaviourContext) {
        if !entity.has(ComponentType::BehaviourState) {
            if let Some(&Component::Behaviour(b)) = entity.get(ComponentType::Behaviour) {
            }
        }
    }
}
