use game::*;
use getopts;

#[derive(Debug)]
pub struct Arguments {
    pub debug: bool,
    pub frontend: Frontend,
}

impl Arguments {
    pub fn from_opts(matches: getopts::Matches) -> ExternalResult<Self> {
        let mut args = Arguments::default();

        if matches.opt_present("debug") {
            args.debug = true;
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
        }
    }
}
