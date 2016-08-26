use game::{
    Observer,
    EntityId,
    EntityTable,
};

use vision::{
    VisionSystem,
    DefaultVisibilityReport,
    Shadowcast,
};

pub struct DrawableObserver {
    visibility_report: DefaultVisibilityReport,
    vision_system: Shadowcast,
}

impl Observer for DrawableObserver {
    fn observe(&mut self, entity_id: EntityId, entities: &EntityTable, turn_count: u64) {
        let entity = entities.get(entity_id);
        let level_id = entity.on_level().unwrap();
        let level = entities.get(level_id);

        let eye = entity.position().unwrap();
        let grid = &level.level_spacial_hash().unwrap().grid;
        let info = entity.vision_distance().unwrap();

        self.visibility_report.clear();
        self.vision_system.detect_visible_area(eye, grid, info, &mut self.visibility_report);

        let mut knowledge = entity.drawable_knowledge_mut().unwrap();
        knowledge.update(level_id, entities, grid, self.visibility_report.iter(), turn_count);
    }
}

impl DrawableObserver {
    pub fn new() -> Self {
        DrawableObserver {
            visibility_report: DefaultVisibilityReport::new(),
            vision_system: Shadowcast::new(),
        }
    }
}
