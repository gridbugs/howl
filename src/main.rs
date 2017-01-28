#![allow(dead_code)]

#[macro_use]
extern crate itertools;
extern crate rand;
extern crate num;
extern crate getopts;
extern crate toml;

#[cfg(all(unix, feature = "rustty"))]
extern crate rustty;

#[cfg(feature = "sdl2")]
extern crate sdl2;

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
mod colour;
mod spatial_hash;

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let opts = game::make_options();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            print_usage(&program, opts);
            return;
        }
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

    println!("RNG Seed: {}", args.rng_seed);

    if let Err(message) = game::launch(args) {
        println!("Error: {}", message);
    }
}
