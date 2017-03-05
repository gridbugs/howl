use game::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum MessageType {
    Empty,
    Welcome,
    Intro,
    Title,
    PressAnyKey,
    YouDied,
    Action(ActionMessageType),
    YouSee(Option<YouSeeMessageType>),
    YouRemember(Option<YouSeeMessageType>),
    Unseen,
    Description(DescriptionMessageType),
    YouSeeDescription(YouSeeMessageType),
    NoDescription,
    Menu(MenuMessageType),
    ChooseDirection,
    EmptyWeaponSlot,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum YouSeeMessageType {
    Player,
    Tree,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ActionMessageType {
    PlayerOpenDoor,
    PlayerCloseDoor,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum DescriptionMessageType {
    Player,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum MenuMessageType {
    NewGame,
    Continue,
    Quit,
    SaveAndQuit,
    Controls,
    Control(InputEvent, Control),
    UnboundControl(Control),
    ControlBinding(Control),
    NextDelivery,
    Shop,
    Garage,
}
