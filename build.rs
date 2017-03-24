use std::env;
use std::fs;
use std::path::{Path, PathBuf};

extern crate statecs;
extern crate statecs_spatial_hash;
extern crate copy_dir;
extern crate tomson;
extern crate handlebars;
extern crate rustc_serialize;

fn main() {

    let mut cfg = statecs::Config::new();

    cfg.combine_flag_set = false;
    cfg.component_bookkeeping = true;
    cfg.action_component_bookkeeping = true;
    cfg.ecs_ctx_hash_collections = true;
    cfg.ecs_action_hash_collections = true;
    cfg.fnv_hasher = true;

    statecs::generate("ecs.toml", Path::new("src").join("ecs").join("generated.rs"), cfg);
    statecs_spatial_hash::generate("sh.toml", Path::new("src").join("spatial_hash").join("generated.rs"));

    copy_resources("resources");
    copy_resources("user");
}

fn target_dirs() -> Result<Vec<PathBuf>, env::VarError> {
    let target = env::var("TARGET")?;
    let host = env::var("HOST")?;
    let profile = env::var("PROFILE")?;

    if target == host {
        Ok(vec![
           Path::new("target").join(&profile),
           Path::new("target").join(&target).join(&profile),
        ])
    } else {
        Ok(vec![Path::new("target").join(&target).join(&profile)])
    }
}

fn copy_resources(resources_dir_name: &str) {

    for target_dir in target_dirs().expect("Failed to locate target directory") {

        if !target_dir.is_dir() {
            continue;
        }

        let dest_path = target_dir.join(resources_dir_name);

        if dest_path.is_dir() {
            fs::remove_dir_all(&dest_path).expect("Failed to remove old resources directory");
        }
        if dest_path.is_file() {
            fs::remove_file(&dest_path).expect("Failed to remove file in place of resources directory");
        }
        copy_dir::copy_dir(resources_dir_name, &dest_path).expect("Failed to copy resources");
    }
}
