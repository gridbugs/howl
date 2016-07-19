use std::io;
use std::fmt;

use ecs::entity_table::EntityTable;
use ecs::system_queue::SystemQueue;
use ecs::message::Message;
use ecs::entity::Component::*;
use ecs::entity::ComponentType as Type;
use ecs::message::Field::*;
use ecs::message::FieldType;

use grid::static_grid::StaticGrid;

pub struct WriteRenderer<T>(pub T);

impl<T> WriteRenderer<T> {
    pub fn new(write: T) -> Self {
        WriteRenderer(write)
    }
}

impl<T> fmt::Debug for WriteRenderer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WriteRenderer {{}}")
    }
}

impl<T: io::Write> WriteRenderer<T> {
    pub fn process_message(&mut self, message: &mut Message,
                       entities: &mut EntityTable,
                       _: &SystemQueue)
    {

        if let Some(&RenderLevel { level: level_id }) = message.get(FieldType::RenderLevel) {

            if let Some(&Level(ref level)) = entities.get(level_id).get(Type::Level) {
                let mut grid = StaticGrid::new_copy(level.width as isize, level.height as isize, (' ', -1));

                for entity in level.entities(entities) {
                    if let Some(&Position(ref v)) = entity.get(Type::Position) {
                        let cell = &mut grid[v];
                        if let Some(&TileDepth(depth)) = entity.get(Type::TileDepth) {
                            if depth <= cell.1 {
                                continue;
                            }
                            cell.1 = depth;
                            if let Some(&SolidTile{ref tile, background: _}) = entity.get(Type::SolidTile) {
                                cell.0 = tile.character;
                            } else if let Some(&TransparentTile(ref tile)) = entity.get(Type::TransparentTile) {
                                cell.0 = tile.character;
                            }
                        }
                    }
                }

                for row in grid.rows() {
                    for cell in row {
                        write!(self.0, "{}", cell.0).unwrap();
                    }
                    write!(self.0, "\n").unwrap();
                }
            }
        }
    }
}
