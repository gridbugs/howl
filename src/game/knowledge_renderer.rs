use game::*;
use math::Coord;

pub trait KnowledgeRenderer {

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn update(&mut self, knowledge: &DrawableKnowledgeLevel, turn_id: u64, position: Coord);

    fn draw(&mut self);

    fn draw_with_overlay(&mut self, overlay: &RenderOverlay);

    fn render(&mut self, knowledge: &DrawableKnowledgeLevel,
              turn_id: u64, position: Coord) {
        self.update(knowledge, turn_id, position);
        self.draw();
    }

    fn render_with_overlay(&mut self, knowledge: &DrawableKnowledgeLevel,
                           turn_id: u64, position: Coord, overlay: &RenderOverlay) {
        self.update(knowledge, turn_id, position);
        self.draw_with_overlay(overlay);
    }
}
