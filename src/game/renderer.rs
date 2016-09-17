use game::{
    Level,
    EntityId,
};
use game::io::WindowKnowledgeRenderer;
use game::observer::DrawableObserver;

pub struct Renderer<'a> {
    observer: DrawableObserver,
    renderer: WindowKnowledgeRenderer<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(observer: DrawableObserver,
           renderer: WindowKnowledgeRenderer<'a>) -> Self
    {
        Renderer {
            observer: observer,
            renderer: renderer,
        }
    }

    pub fn render(&mut self, level: &Level, entity_id: EntityId, turn: u64) -> bool {
        let change = self.observer.observe(entity_id, level, turn);

        if change {
            self.renderer.render(level, entity_id, level.id(), turn);
        }

        change
    }
}


