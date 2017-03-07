use game::*;
use game::data::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum MessageType {
    Empty,
    Title,
    PressAnyKey,
    YouDied,
    Action(ActionMessageType),
    Name(NameMessageType),
    YouRemember(Option<NameMessageType>),
    Unseen,
    Description(DescriptionMessageType),
    NameDescription(NameMessageType),
    NoDescription,
    Menu(MenuMessageType),
    ChooseDirection,
    EmptyWeaponSlotMessage,
    Front,
    Rear,
    Left,
    Right,
    EmptyWeaponSlot,
    SurvivorCamp,
    ShopTitle(usize),
    ShopTitleInsufficientFunds(usize),
    ShopTitleInventoryFull(usize),
    Inventory {
        size: usize,
        capacity: usize,
    },
    NameAndDescription(NameMessageType, DescriptionMessageType),
    Garage,
    GarageInventoryFull,
    WeaponSlotTitle(RelativeDirection, Option<NameMessageType>),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum VerbMessageType {
    Ram,
    Claw,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum NameMessageType {
    Pistol,
    Shotgun,
    MachineGun,
    Railgun,
    Car,
    Bike,
    Zombie,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ActionMessageType {
    TyreDamage,
    EngineDamage,
    ArmourDamage,
    ArmourDeflect,
    PersonalDamage,
    Shot,
    ShotBy(NameMessageType),
    BumpedBy(NameMessageType, VerbMessageType),
    FailToTurn,
    FailToAccelerate,
    TyreAcidDamage,
    MaxSpeedDecreased,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum DescriptionMessageType {
    Pistol,
    Shotgun,
    MachineGun,
    Railgun,
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
    Inventory,
    Name(NameMessageType),
    ShopItem(NameMessageType, usize),
    Back,
    Remove,
    WeaponSlot(RelativeDirection, Option<NameMessageType>),
    Empty,
}
