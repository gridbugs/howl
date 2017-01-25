#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MessageType {
    Welcome,
    Intro,
    Empty,
    Action(ActionMessageType),
    YouSee(Option<YouSeeMessageType>),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum YouSeeMessageType {
    Player,
    Tree,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ActionMessageType {
    PlayerOpenDoor,
    PlayerCloseDoor,
}
