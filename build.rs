extern crate genecs;

fn main() {
    genecs::generate_ecs("ecs.toml", "src/ecs/generated.rs")
}
