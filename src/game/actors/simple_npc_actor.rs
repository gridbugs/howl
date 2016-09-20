use game::{Level, EntityId, EntityRef, EntityStore, EntityWrapper, ReserveEntityId, MetaAction,
           actions};

use game::knowledge::SimpleNpcCell;

use search::{Query, SearchContext, WeightedGridSearchContext, Traverse, TraverseType, CellInfo};

use vision::{VisionSystem, DefaultVisibilityReport, Shadowcast};

impl Traverse for SimpleNpcCell {
    fn get_type(&self) -> TraverseType {
        if self.extra().solid {
            TraverseType::NonTraversable
        } else {
            TraverseType::Traversable(1.0)
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

    pub fn act(&mut self,
               level: &Level,
               id: EntityId,
               _: &ReserveEntityId,
               turn: u64)
               -> MetaAction {

        let entity = level.get(id).unwrap();
        self.observe(level, entity, turn);

        let knowledge = entity.simple_npc_knowledge().unwrap();
        let grid = knowledge.grid(level.id()).unwrap();
        let position = entity.position().unwrap();

        let query = Query::new_to_predicate(position, |info: CellInfo<SimpleNpcCell>| {
            info.value.extra().player
        });
        let result = self.search_context.search(grid, &query);

        if let Ok(ref path) = result {
            let start = &path.coords[0];
            MetaAction::Update(actions::walk(entity, start.direction))
        } else {
            MetaAction::Update(actions::wait())
        }
    }
}
