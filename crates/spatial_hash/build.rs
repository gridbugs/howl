use std::path::Path;

extern crate statecs_spatial_hash;

fn main() {
    statecs_spatial_hash::generate("sh.toml", Path::new("src").join("generated.rs"));
}
