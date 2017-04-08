use std::env;
use std::path;

use getopts;
use rand::{Rng, StdRng};

use game::*;

const RESOURCE_DIR: &'static str = "resources";
const USER_DIR: &'static str = "user";

pub fn make_options() -> getopts::Options {

    let mut opts = getopts::Options::new();

    let frontends = format!("[ {} ]", FRONTEND_STRINGS.join(" | "));

    opts.optflag("d", "debug", "enable debugging output");
    opts.optopt("f", "frontend", "specify frontend", frontends.as_ref());
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("r", "rngseed", "seed the random number generator with a non-negative integer", "SEED");
    opts.optflag("a", "no-animation", "disable real-time animations");

    opts
}

#[derive(Debug)]
pub struct Arguments {
    pub debug: bool,
    pub frontend: Frontend,
    pub rng_seed: usize,
    pub animation: bool,
    pub resource_path: path::PathBuf,
    pub user_path: path::PathBuf,
    pub config: GameConfig,
}

impl Arguments {
    pub fn from_opts(matches: getopts::Matches) -> ExternalResult<Self> {
        let mut args = Arguments::default();

        if matches.opt_present("debug") {
            args.debug = true;
        }

        if matches.opt_present("no-animation") {
            args.animation = false;
        }

        if let Some(rng_seed_str) = matches.opt_str("rngseed") {
            if let Ok(rng_seed) = rng_seed_str.parse::<usize>() {
                args.rng_seed = rng_seed;
            } else {
                return Err("RNG seed must be a non-negative integer".to_string());
            }
        } else {
            if let Ok(mut tmp_rng) = StdRng::new() {
                args.rng_seed = tmp_rng.gen();
            } else {
                return Err("Failed to seed RNG.".to_string());
            }
        }

        if let Some(frontend_str) = matches.opt_str("frontend") {
            if let Some(frontend) = Frontend::from_string(frontend_str.as_ref()) {
                args.frontend = frontend;
            } else {
                return Err(format!("No such frontend: {}", frontend_str));
            }
        } else if let Some(frontend) = sane_frontend() {
            args.frontend = frontend;
        } else {
            return Err("Could not find suitable frontend".to_string());
        }

        if let Some(path) = resource_dir_path() {
            args.resource_path = path;
        }

        if let Some(path) = user_dir_path() {
            args.user_path = path;
        }

        args.config = GameConfig::from_file(args.user_path.join(user_files::CONFIG)).unwrap_or_default();

        Ok(args)
    }
}

fn resource_dir_path() -> Option<path::PathBuf> {
    env::current_exe().ok().and_then(|mut path| {
        // get directory containing exe
        if !path.pop() {
            return None;
        }

        Some(path.join(RESOURCE_DIR))
    })
}

fn user_dir_path() -> Option<path::PathBuf> {
    env::current_exe().ok().and_then(|mut path| {
        // get directory containing exe
        if !path.pop() {
            return None;
        }

        Some(path.join(USER_DIR))
    })
}

fn sane_frontend() -> Option<Frontend> {
    if cfg!(feature = "sdl2") {
        Some(Frontend::Sdl)
    } else {
        None
    }
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            debug: false,
            frontend: Frontend::Sdl,
            rng_seed: 0,
            animation: true,
            resource_path: path::PathBuf::new(),
            user_path: path::PathBuf::new(),
            config: GameConfig::default(),
        }
    }
}
