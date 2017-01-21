#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MessageType {
    Welcome,
    PlayerOpenDoor,
    PlayerCloseDoor,
}
