#[derive(Clone, Copy, RustcEncodable, RustcDecodable)]
pub enum BehaviourType {
    Null,
    PlayerInput,
    SimpleNpc,
    Clouds,
}
