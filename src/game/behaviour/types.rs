#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum BehaviourType {
    Null,
    PlayerInput,
    Zombie,
    AcidAnimate,
    Physics,
    Car,
    Bike,
}
