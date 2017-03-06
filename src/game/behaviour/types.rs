#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum BehaviourType {
    Null,
    PlayerInput,
    SimpleNpc,
    AcidAnimate,
    Physics,
    Car,
}
