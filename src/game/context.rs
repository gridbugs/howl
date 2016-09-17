use game::{
    UpdateSummary,
    MetaAction,
    Rule,
    EntityContext,
    LevelStore,
    EntityId,
    Level,
    LevelId,
    ComponentType,
    actions,
    EntityWrapper,
    EntityStore,
    CommitContext,
    Renderer,
    ActorManager,
};
use game::components::Form;

use game::io::{
    WindowKnowledgeRenderer,
};
use game::observer::DrawableObserver;

use terminal::{
    Window,
    InputSource
};

fn cloud_progress(level: &mut Level) -> UpdateSummary {
    level.update_clouds_action()
}

fn transformation(id: EntityId, level: &Level) -> Option<UpdateSummary> {
    let entity = level.get(id).unwrap();
    if let Some(form) = entity.form() {
        if let Some(position) = entity.position() {
            let sh = level.spatial_hash();
            let sh_cell = sh.get((position.x, position.y)).unwrap();
            if sh_cell.has(ComponentType::Moon) {
                if form == Form::Human {
                    return Some(actions::beast_transform_progress(entity, -1));
                } else {
                    return Some(actions::human_transform_progress(entity, 1));
                }
            } else {
                if form == Form::Human {
                    return Some(actions::beast_transform_progress(entity, 1));
                } else {
                    return Some(actions::human_transform_progress(entity, -1));
                }
            }
        }
    }

    None
}

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
}

#[derive(Debug)]
enum TurnError {
    Quit,
}

#[derive(Debug)]
enum UpdateError {
    NothingApplied,
}

impl<'a> GameContext<'a> {
    pub fn new(input_source: InputSource<'a>,
               game_window: Window<'a>) -> Self {
        GameContext {
            entities: EntityContext::new(),
            pc: None,
            pc_level_id: 0,
            actors: ActorManager::new(input_source),
            renderer: Renderer::new(
                DrawableObserver::new(),
                WindowKnowledgeRenderer::new(game_window)
            ),
            commit_context: CommitContext::new(),
            rules: Vec::new(),
            turn: 0,
        }
    }

    pub fn rule<R: 'static + Rule>(&mut self, r: R) -> &mut Self {
        self.rules.push(Box::new(r));

        self
    }

    fn game_turn(&mut self) -> Result<(), TurnError> {

        self.turn += 1;

        let level = self.entities.levels.level_mut(self.pc_level_id).unwrap();
        let ids = &self.entities.entity_ids;

        let entity_id = level.schedule.next().expect("schedule is empty");

        // update cloud positions, bypassing rules
        let cloud_update = cloud_progress(level);
        level.commit_update(cloud_update, self.turn);

        // apply transformation system
        if let Some(transform_update) = transformation(entity_id, level) {
            self.turn = self.commit_context.apply_update(
                level,
                transform_update,
                &self.rules,
                None,
                ids,
                self.turn);
        }

        self.renderer.render(level, self.pc.unwrap(), self.turn);

        loop {
            match self.actors.act(level, entity_id, ids) {
                MetaAction::Quit => return Err(TurnError::Quit),
                MetaAction::Update(update) => {
                    let old_turn = self.turn;
                    self.turn = self.commit_context.apply_update(
                        level,
                        update,
                        &self.rules,
                        Some((self.pc.unwrap(), &mut self.renderer)),
                        ids,
                        self.turn);

                    if self.turn == old_turn &&
                        level.get(entity_id).unwrap().is_pc()
                    {
                        continue;
                    } else {
                        break;
                    }
                },
            }
        }

        Ok(())
    }

    pub fn game_loop(&mut self) {
        loop {
            if let Err(err) = self.game_turn() {
                match err {
                    TurnError::Quit => break,
                }
            }
        }
    }
}
