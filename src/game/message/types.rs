#[derive(Clone, Copy, PartialEq, Eq, Debug, RustcEncodable, RustcDecodable)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug, RustcEncodable, RustcDecodable)]
pub enum YouSeeMessageType {
    Player,
    Tree,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, RustcEncodable, RustcDecodable)]
pub enum ActionMessageType {
    PlayerOpenDoor,
    PlayerCloseDoor,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, RustcEncodable, RustcDecodable)]
pub enum DescriptionMessageType {
    Player,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, RustcEncodable, RustcDecodable)]
pub enum MenuMessageType {
    NewGame,
    Continue,
    Quit,
    SaveAndQuit,
}
