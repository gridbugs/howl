use game::*;
use math::Coord;

pub trait KnowledgeRenderer {

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn world_offset(&self) -> Coord;

    fn world_limit(&self) -> Coord {
        self.world_offset() + Coord::new(self.width() as isize - 1, self.height() as isize - 1)
    }

    fn contains_world_coord(&self, coord: Coord) -> bool {
        coord >= self.world_offset() && coord < self.world_limit()
    }

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