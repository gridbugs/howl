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
mod system_queue_test {
    use ecs;
    use ecs::entity_table::EntityTable;
    use ecs::entity_types::*;
    use ecs::message::Field;
    use ecs::entity::Component::*;
    use ecs::entity::ComponentType as Type;
    use ecs::entity::EntityId;
    use ecs::system::{System, SystemName};
    use ecs::systems::write_renderer::WriteRenderer;

    const WIDTH: usize = 6;
    const HEIGHT: usize = 4;

    fn populate(entities: &mut EntityTable) -> Option<EntityId> {
        let level_id = entities.add(make_level(WIDTH, HEIGHT));

        for y in 0..HEIGHT {
            for x in 0..WIDTH {

                let floor = entities.add(make_floor(x as isize, y as isize));
                if let Some(&mut Level(ref mut level)) = entities.get_mut(level_id).get_mut(Type::Level) {
                    level.add(floor);
                }

                if x == 0 || x == WIDTH - 1 || y == 0 || y == HEIGHT - 1 {
                    let wall = entities.add(make_wall(x as isize, y as isize));
                    if let Some(&mut Level(ref mut level)) = entities.get_mut(level_id).get_mut(Type::Level) {
                        level.add(wall);
                    }
                }
            }
        }

        let pc = entities.add(make_pc(3, 2));
        if let Some(&mut Level(ref mut level)) = entities.get_mut(level_id).get_mut(Type::Level) {
            level.add(pc);
            Some(level_id)
        } else {
            None
        }
    }

    #[test]
    fn test() {

        let systems = system_queue![
            SystemName::Renderer => System::TestRenderer(WriteRenderer::new(Vec::<u8>::new())),
        ];

        let mut entities = EntityTable::new();

        if let Some(level_id) = populate(&mut entities) {

            let mut message = message![
                Field::RenderLevel { level: level_id },
            ];

            for system in systems.iter() {
                system.borrow_mut().process_message(&mut message, &mut entities, &systems);
            }
        }

        let renderer_system = systems.get(SystemName::Renderer).borrow();
        if let System::TestRenderer(ref renderer) = *renderer_system {
            let rendered = String::from_utf8(renderer.0.to_vec()).unwrap();
            assert_eq!(rendered, "######\n#....#\n#..@.#\n######\n");
        }
    }
}
