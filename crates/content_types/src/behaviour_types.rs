#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum BehaviourType {
    Null,
    PlayerInput,
    Zombie,
    Physics,
    Car,
    Bike,
}
