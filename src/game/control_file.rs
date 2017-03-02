use std::path::Path;

use game::*;

pub fn from_file<P: AsRef<Path>>(path: P) -> Option<ControlMap> {
    let spec: Option<StringControlSpec> = game_file::read_toml(path).ok();
    spec.as_ref().map(ControlMap::from)
}

pub fn to_file<P: AsRef<Path>>(path: P, map: &ControlMap) {
    game_file::write_toml(path, &StringControlSpec::from(map))
        .expect("Failed to write controls file");
}
