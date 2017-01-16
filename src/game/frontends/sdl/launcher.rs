use std::io::{self, Read};
use std::fs;
use std::path;

use sdl2;
use sdl2::image::INIT_PNG;
use toml;

use game::*;
use game::frontends::sdl::SdlKnowledgeRendererError;

use debug;

const TILESET_NAME: &'static str = "PxPlus_IBM_BIOS";

pub fn launch(args: Arguments) -> ExternalResult<()> {

    let debug_buffer: Box<io::Write> = if args.debug {
        Box::new(debug::PrintDebug)
    } else {
        Box::new(debug::NullDebug)
    };

    debug::init(debug_buffer);

    let sdl = sdl2::init().expect("SDL2 initialization failed");

    let (tile_spec, tile_path) = match parse_tileset_spec(&args.resource_path) {
        Some(value) => value,
        None => return Err("Couldn't find tileset".to_string()),
    };

    let tileset = match frontends::sdl::Tileset::new(tile_spec) {
        Ok(tileset) => tileset,
        Err(e) => return Err(format!("Couldn't parse tileset: {:?}", e).to_string()),
    };

    let video = sdl.video().map_err(|_| "Failed to connect to video subsystem")?;
    sdl2::image::init(INIT_PNG).map_err(|_| "Failed to connect to image subsystem")?;

    let renderer = match frontends::sdl::SdlKnowledgeRenderer::new(
        video,
        "Howl",
        GAME_WIDTH,
        GAME_HEIGHT,
        tile_path,
        tileset) {
        Ok(r) => r,
        Err(SdlKnowledgeRendererError::WindowCreationFailure) => return Err("Failed to create window".to_string()),
        Err(SdlKnowledgeRendererError::RendererInitialisationFailure) => return Err("Failed to initialise renderer".to_string()),
        Err(SdlKnowledgeRendererError::TileLoadFailure) => return Err("Failed to load tiles".to_string()),
    };

    let input = frontends::sdl::SdlInputSource::new(sdl.clone());
    let input_ref = InputSourceRef::new(&input);

    let mut game = GameCtx::new(Box::new(renderer),
                                input_ref,
                                args.rng_seed,
                                GAME_WIDTH,
                                GAME_HEIGHT);

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
