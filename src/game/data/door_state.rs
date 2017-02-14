#[derive(Clone, Copy, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub enum DoorState {
    Open,
    Closed,
}

impl DoorState {
    pub fn is_open(self) -> bool {
        self == DoorState::Open
    }
    pub fn is_closed(self) -> bool {
        self == DoorState::Closed
    }
}
