#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MessageType {
    Empty,
    Welcome,
    Intro,
    Title,
    PressAnyKey,
    Action(ActionMessageType),
    YouSee(Option<YouSeeMessageType>),
    YouRemember(Option<YouSeeMessageType>),
    Unseen,
    Description(DescriptionMessageType),
    YouSeeDescription(YouSeeMessageType),
    NoDescription,
    Menu(MenuMessageType),
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DescriptionMessageType {
    Player,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MenuMessageType {
    NewGame,
    Quit,
}
