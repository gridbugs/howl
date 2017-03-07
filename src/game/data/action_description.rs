use game::*;
use coord::Coord;

#[derive(Clone, Copy, Debug)]
pub struct ActionDescription {
    pub coord: Coord,
    pub message: ActionMessageType,
}

impl ActionDescription {
    pub fn new(coord: Coord, message: ActionMessageType) -> Self {
        ActionDescription {
            coord: coord,
            message: message,
        }
    }
}
