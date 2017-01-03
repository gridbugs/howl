use getopts;
use rand::{Rng, StdRng};

use game::*;

#[derive(Debug)]
pub struct Arguments {
    pub debug: bool,
    pub frontend: Frontend,
    pub rng_seed: usize,
}

impl Arguments {
    pub fn from_opts(matches: getopts::Matches) -> ExternalResult<Self> {
        let mut args = Arguments::default();

        if matches.opt_present("debug") {
            args.debug = true;
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
        }

        Ok(args)
    }
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            debug: false,
            frontend: Frontend::Ansi,
            rng_seed: 0,
        }
    }
}
