use game::Component;

use geometry::Vector2;

pub trait ComponentWrapper<'a> {
    fn position(self) -> Option<Vector2<isize>>;
}

impl<'a> ComponentWrapper<'a> for Option<&'a Component> {
    fn position(self) -> Option<Vector2<isize>> {
        if let Some(&Component::Position(ref vec)) = self {
            Some(*vec)
        } else {
            None
        }
    }
}
