use game::entity::{
    EntityTable,
    EntityId
};
use grid::Coord;
use terminal::window_manager::WindowRef;
use colour::ansi;

pub struct WindowKnowledgeRenderer<'a> {
    window: WindowRef<'a>,
}
impl<'a> WindowKnowledgeRenderer<'a> {
    pub fn new(window: WindowRef<'a>) -> Self {
        WindowKnowledgeRenderer {
            window: window,
        }
    }

    pub fn render(&self,
                  entities: &EntityTable,
                  entity_id: EntityId,
                  turn_count: u64)
    {
        let entity = entities.get(entity_id);
        let level_id = entity.on_level().unwrap();
        let knowledge = entity.drawable_knowledge().unwrap();
        let grid = &knowledge.grid(level_id).unwrap();

        for (Coord {x, y}, cell) in izip!(
            grid.coord_iter(),
            grid.iter())
        {
            let window_cell = self.window.get_cell(x, y);
            cell.foreground.value().map(|tile| {
                let colour = if cell.last_turn == turn_count {
                    tile.colour
                } else {
                    ansi::BLACK
                };
                window_cell.set(tile.character, colour, ansi::DARK_GREY);
            });
        }

        self.window.flush();
    }
}
