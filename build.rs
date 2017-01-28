use std::env;
use std::fs;
use std::path;

extern crate genecs;
extern crate copy_dir;
extern crate tomson;
extern crate handlebars;
extern crate rustc_serialize;

mod gensh;

fn main() {
    gensh::generate_spatial_hash("sh.toml", format!("src{}spatial_hash{}generated.rs",
                                                    path::MAIN_SEPARATOR,
                                                    path::MAIN_SEPARATOR));
    genecs::generate_ecs("ecs.toml", format!("src{}ecs{}generated.rs",
                                             path::MAIN_SEPARATOR,
                                             path::MAIN_SEPARATOR));
    copy_resources("resources");
}

fn target_dir_names() -> Result<Vec<String>, env::VarError> {
    let target = env::var("TARGET")?;
    let host = env::var("HOST")?;
    let profile = env::var("PROFILE")?;

    if target == host {
        Ok(vec![
            format!("target{}{}", path::MAIN_SEPARATOR, profile).to_string(),
            format!("target{}{}{}{}", path::MAIN_SEPARATOR, target, path::MAIN_SEPARATOR, profile).to_string(),
        ])
    } else {
        Ok(vec![format!("target{}{}{}{}", path::MAIN_SEPARATOR, target, path::MAIN_SEPARATOR, profile).to_string()])
    }
}

fn copy_resources(resources_dir_name: &str) {
    let target_dir_names = target_dir_names().expect("Failed to locate target directory");

    for target_dir_name in target_dir_names {

        if !path::PathBuf::from(&target_dir_name).is_dir() {
            continue;
        }

        let dest_name = format!("{}{}{}", target_dir_name, path::MAIN_SEPARATOR, resources_dir_name);
        let dest_path = path::PathBuf::from(dest_name);
        if dest_path.is_dir() {
            fs::remove_dir_all(&dest_path).expect("Failed to remove old resources directory");
        }
        if dest_path.is_file() {
            fs::remove_file(&dest_path).expect("Failed to remove file in place of resources directory");
        }
        copy_dir::copy_dir(resources_dir_name, &dest_path).expect("Failed to copy resources");
    }
}
