#![allow(dead_code)]

#[macro_use]
extern crate itertools;
extern crate rand;
extern crate num;
extern crate getopts;

#[cfg(unix)]
extern crate rustty;

use std::env;

#[macro_use]
mod debug;
mod frontends;
mod ecs;
mod math;
mod game;
mod util;
mod direction;
mod grid;
mod behaviour;
mod search;
mod perlin;
mod coord;

fn make_options() -> getopts::Options {

    let mut opts = getopts::Options::new();

    let frontends = format!("[ {} ]", game::FRONTEND_STRINGS.join(" | "));

    opts.optflag("d", "debug", "enable debugging output");
    opts.optopt("f", "frontend", "specify frontend", frontends.as_ref());
    opts.optflag("h", "help", "print this help menu");

    opts
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let opts = make_options();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let args = match game::Arguments::from_opts(matches) {
        Ok(args) => args,
        Err(message) => {
            println!("Error: {}", message);
            print_usage(&program, opts);
            return;
        }
    };

    if let Err(message) = game(args) {
        panic!("Error: {}", message);
    }
}

fn game(args: game::Arguments) -> game::ExternalResult<()> {

#[cfg(unix)]
    game::frontends::ansi::launch(args)?;

    Ok(())
}
