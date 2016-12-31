use game::*;
use math::Coord;

pub trait KnowledgeRenderer {

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn draw(&mut self);

    fn update(&mut self, knowledge: &DrawableKnowledgeLevel, turn_id: u64, position: Coord);

    fn render(&mut self, knowledge: &DrawableKnowledgeLevel,
              turn_id: u64, position: Coord) {
        self.update(knowledge, turn_id, position);
        self.draw();
    }
}
