use game::observer::Observer;
use game::vision::{
    VisionSystem,
    DefaultVisibilityReport,
    square,
};
use game::{
    EntityId,
    EntityTable,
};

pub struct DefaultObserver {
    visibility_report: DefaultVisibilityReport,
}

impl Observer for DefaultObserver {
    fn observe(&mut self, entity_id: EntityId, entities: &EntityTable) {
        let entity = entities.get(entity_id);
        let level_id = entity.on_level().unwrap();
        let level = entities.get(level_id);

        let eye = entity.position().unwrap();
        let grid = &level.level_spacial_hash().unwrap().grid;
        let info = entity.vision_distance().unwrap();

        self.visibility_report.clear();
        square.detect_visible_area(eye, grid, info, &mut self.visibility_report);

        let mut knowledge = entity.default_knowledge().unwrap();
        knowledge.update(level_id, entities, grid, &self.visibility_report);
    }
}

impl DefaultObserver {
    pub fn new() -> Self {
        DefaultObserver {
            visibility_report: DefaultVisibilityReport::new(),
        }
    }
}
