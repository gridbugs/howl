use std::io::{self, Read};
use std::fs;
use std::path;

use sdl2;
use toml;

use game::*;
use debug;

const GAME_WINDOW_WIDTH: usize = 41;
const GAME_WINDOW_HEIGHT: usize = 31;

const TILESET_NAME: &'static str = "PxPlus_IBM_BIOS";

pub fn launch(args: Arguments) -> ExternalResult<()> {

    let sdl = sdl2::init().expect("SDL2 initialization failed");

    let (tile_spec, tile_path) = match parse_tileset_spec(&args.resource_path) {
        Some(value) => value,
        None => return Err("Couldn't find tileset".to_string()),
    };

    let tileset = match frontends::sdl::Tileset::new(tile_spec) {
        Ok(tileset) => tileset,
        Err(e) => return Err(format!("Couldn't parse tileset: {:?}", e).to_string()),
    };

    let renderer = frontends::sdl::SdlKnowledgeRenderer::new(
        sdl.clone(),
        GAME_WINDOW_WIDTH,
        GAME_WINDOW_HEIGHT,
        tile_path,
        tileset);

    let input = frontends::sdl::SdlInputSource::new(sdl.clone());
    let input_ref = InputSourceRef::new(&input);

    let mut game = GameCtx::new(Box::new(renderer),
                                input_ref,
                                args.rng_seed,
                                GAME_WINDOW_WIDTH,
                                GAME_WINDOW_HEIGHT);

    let debug_buffer: Box<io::Write> = if args.debug {
        Box::new(debug::PrintDebug)
    } else {
        Box::new(debug::NullDebug)
    };

    debug::init(debug_buffer);

    game.run()?;

    Ok(())
}

fn parse_tileset_spec(resource_path: &path::PathBuf) -> Option<(toml::Table, path::PathBuf)> {
    let tileset_base_path = resource_path.join("tilesets").join(TILESET_NAME);
    let tileset_spec_path = tileset_base_path.join("tiles.toml");
    let mut toml_str = String::new();

    fs::File::open(tileset_spec_path).and_then(|mut file| {
        match file.read_to_string(&mut toml_str) {
            Ok(_) => Ok(toml::Parser::new(toml_str.as_ref())),
            Err(e) => Err(e),
        }
    }).ok().and_then(|mut parser| parser.parse()).map(|value| {
        (value, tileset_base_path.join("tiles.png"))
    })
}
