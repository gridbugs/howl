use std::path::Path;

extern crate statecs;

fn main() {

    let mut cfg = statecs::Config::new();

    cfg.combine_flag_set = false;
    cfg.component_bookkeeping = true;
    cfg.action_component_bookkeeping = true;
    cfg.ecs_ctx_hash_collections = true;
    cfg.ecs_action_hash_collections = true;
    cfg.fnv_hasher = true;

    statecs::generate_core(Path::new("src").join("generated.rs"), cfg);
}
