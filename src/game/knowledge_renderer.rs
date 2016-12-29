use game::*;
use math::Coord;

pub trait KnowledgeRenderer {
    fn render(&mut self, knowledge: &DrawableKnowledgeLevel,
              turn_id: u64, position: Coord);
}
