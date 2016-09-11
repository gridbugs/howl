use game::{
    EntityId,
    EntityWrapper,
    EntityStore,
    Level,
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

impl DrawableObserver {
    pub fn new() -> Self {
        DrawableObserver {
            visibility_report: DefaultVisibilityReport::new(),
            vision_system: Shadowcast::new(),
        }
    }

    pub fn observe(&mut self, entity_id: EntityId, entities: &Level, turn_count: u64) -> bool{
        let entity = entities.get(entity_id).unwrap();

        let eye = entity.position().unwrap();
        let grid = &entities.spatial_hash().grid;
        let info = entity.vision_distance().unwrap();

        self.visibility_report.clear();
        self.vision_system.detect_visible_area(eye, grid, info, &mut self.visibility_report);

        let mut knowledge = entity.drawable_knowledge_mut().unwrap();
        knowledge.update(entities, grid, self.visibility_report.iter(), turn_count)
    }
}
