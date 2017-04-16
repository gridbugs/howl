#![allow(dead_code)]


#[macro_use]
extern crate itertools;
extern crate rand;
extern crate num;
extern crate getopts;
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;
extern crate fnv;

#[cfg(feature = "sdl2")]
extern crate sdl2;

extern crate action;
extern crate behaviour;
extern crate colour;
extern crate content_types;
extern crate control;
extern crate ecs_content;
extern crate ecs_core;
extern crate engine_defs;
extern crate grid;
extern crate line;
extern crate math;
extern crate message;
extern crate perlin;
extern crate search;
extern crate spatial_hash;
extern crate tile;
extern crate util;

use std::env;

#[macro_use]
mod debug;
mod game;

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
