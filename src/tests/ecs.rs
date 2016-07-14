use ecs;
use ecs::entity::Component::*;
use ecs::entity::ComponentType as Type;
use geometry::vector2::Vector2;

#[test]
pub fn set_fields() {

    let mut entity = entity!(
        Position(Vector2::new(0, 0))
    );

    if let Some(&mut Position(Vector2 {ref mut x, ref mut y})) = entity.get_mut(Type::Position) {
        *x = 42;
        *y = 43;
    } else {
        panic!();
    }

    if let Some(&Position(Vector2 {x, y})) = entity.get(Type::Position) {
        assert_eq!(x, 42);
        assert_eq!(y, 43);
    } else {
        panic!();
    }
}

#[test]
pub fn add_remove_component() {
    let mut entity = entity!();

    entity.add(Position(Vector2::new(0, 0)));

    if !entity.has(Type::Position) {
        panic!();
    }

    entity.remove(Type::Position);

    if entity.has(Type::Position) {
        panic!();
    }
}

#[cfg(test)]
mod populate_ecs_test {

    use ecs::ecs_context::EcsContext;
    use ecs::entity_types::*;
    use ecs::entity::Component::*;
    use ecs::entity::ComponentType as Type;
    use ecs::entity::EntityId;

    use grid::static_grid::StaticGrid;
    use geometry::vector2::Vector2;

    use std::fmt::Write;

    const WIDTH: usize = 6;
    const HEIGHT: usize = 4;

    fn write_level(f: &mut Write, ecs: &EcsContext, level_id: EntityId) {
        if let Some(&Level(ref level)) = ecs.get(level_id).get(Type::Level) {
            let mut grid = StaticGrid::new_copy(level.width as isize, level.height as isize, (' ', -1));

            for entity in level.entities(ecs) {
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
                    write!(f, "{}", cell.0).unwrap();
                }
                write!(f, "\n").unwrap();
            }
        }
    }

    #[test]
    fn populate_ecs() {
        let mut ecs = EcsContext::new();

        let level = ecs.add(make_level(WIDTH, HEIGHT));

        for y in 0..HEIGHT {
            for x in 0..WIDTH {

                let floor = ecs.add(make_floor(Vector2::new(x as isize, y as isize)));
                if let Some(&mut Level(ref mut level)) = ecs.get_mut(level).get_mut(Type::Level) {
                    level.add(floor);
                }

                if x == 0 || x == WIDTH - 1 || y == 0 || y == HEIGHT - 1 {
                    let wall = ecs.add(make_wall(Vector2::new(x as isize, y as isize)));
                    if let Some(&mut Level(ref mut level)) = ecs.get_mut(level).get_mut(Type::Level) {
                        level.add(wall);
                    }
                }
            }
        }

        let pc = ecs.add(make_pc(Vector2::new(3, 2)));
        if let Some(&mut Level(ref mut level)) = ecs.get_mut(level).get_mut(Type::Level) {
            level.add(pc);
        }

        let mut s = String::new();
        write_level(&mut s, &ecs, level);
        assert_eq!(s, "######\n#....#\n#..@.#\n######\n");
    }
}
