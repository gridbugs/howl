use game::{Level, EntityId, EntityRef, EntityStore, EntityWrapper, ReserveEntityId, MetaAction,
           actions};
use game::knowledge::SimpleNpcCell;

use search::{Path, Query, SearchContext, WeightedGridSearchContext, Traverse, TraverseType};
use vision::{VisionSystem, DefaultVisibilityReport, Shadowcast};
use grid::{Grid, Coord};

impl Traverse for SimpleNpcCell {
    fn get_type(&self) -> TraverseType {
        if self.extra().solid {
            TraverseType::NonTraversable
        } else {
            TraverseType::Traversable(1.0)
        }
    }
}

#[derive(Clone, Debug)]
pub struct SimpleNpcAiState {
    path: Option<Path>,
    target: Option<Coord>,
}

impl SimpleNpcAiState {
    pub fn new() -> Self {
        SimpleNpcAiState {
            path: None,
            target: None,
        }
    }
}

pub struct SimpleNpcActor {
    visibility_report: DefaultVisibilityReport,
    vision_system: Shadowcast,
    search_context: WeightedGridSearchContext,
}

impl SimpleNpcActor {
    pub fn new() -> Self {
        SimpleNpcActor {
            visibility_report: DefaultVisibilityReport::new(),
            vision_system: Shadowcast::new(),
            search_context: WeightedGridSearchContext::new(),
        }
    }

    fn observe<'a, E: EntityRef<'a>>(&mut self, level: &Level, entity: E, turn: u64) {
        let eye = entity.position().unwrap();
        let grid = level.spatial_hash().grid();
        let info = entity.vision_distance().unwrap();

        self.visibility_report.clear();
        self.vision_system.detect_visible_area(eye, grid, info, &mut self.visibility_report);

        let mut knowledge = entity.simple_npc_knowledge_mut().unwrap();
        knowledge.update(level, grid, self.visibility_report.iter(), turn);
    }

    fn player_coord<'a, E: EntityRef<'a>>(&mut self, level: &Level, entity: E) -> Option<Coord> {

        let knowledge = entity.simple_npc_knowledge().unwrap();
        let grid = knowledge.grid(level.id()).unwrap();

        for coord in self.visibility_report.keys() {
            if grid.get_unsafe(*coord).extra().player {
                return Some(*coord);
            }
        }

        None
    }

    fn update_ai<'a, E: EntityRef<'a>>(&mut self, level: &Level, entity: E) {
        if let Some(target) = self.player_coord(level, entity) {

            let mut ai = entity.simple_npc_ai_mut().unwrap();

            if let Some(current_target) = ai.target {
                if current_target == target {
                    // nothing needs to change
                    return;
                }
            }

            // either there is no target, or the target is out of date

            let position = entity.position().unwrap();
            let knowledge = entity.simple_npc_knowledge().unwrap();
            let grid = knowledge.grid(level.id()).unwrap();
            let query = Query::new_to_coord(position, target);
            let result = self.search_context.search(grid, &query);

            if let Ok(path) = result {
                ai.target = Some(target);
                ai.path = Some(path);
            }
        }
    }

    pub fn act(&mut self,
               level: &Level,
               id: EntityId,
               _: &ReserveEntityId,
               turn: u64)
               -> MetaAction {

        let entity = level.get(id).unwrap();
        self.observe(level, entity, turn);
        self.update_ai(level, entity);

        let mut ai = entity.simple_npc_ai_mut().unwrap();

        if let Some(ref mut path) = ai.path {
            if let Some(node) = path.next() {
                return MetaAction::Update(actions::walk(entity, node.direction));
            }
        }

        ai.path = None;
        ai.target = None;

        MetaAction::Update(actions::wait())
    }
}
