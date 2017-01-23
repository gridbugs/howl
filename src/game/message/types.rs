#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MessageType {
    // general
    Welcome,
    Empty,

    // actions
    PlayerOpenDoor,
    PlayerCloseDoor,

    YouSee(Option<NameMessageType>),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NameMessageType {
    Player,
    Tree,
}
