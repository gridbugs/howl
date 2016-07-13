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
