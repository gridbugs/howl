#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
