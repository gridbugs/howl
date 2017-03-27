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
    EngineRepair,
    TyresRepair,
    ArmourUpgrade(usize),
    EngineRepairKit,
    SpareTyre,
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
    EngineRepaired,
    TyreReplaced,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum DescriptionMessageType {
    Pistol,
    Shotgun,
    MachineGun,
    Railgun,
}
