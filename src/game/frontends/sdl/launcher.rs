use std::io;
use std::path;

use sdl2;
use sdl2::image::INIT_PNG;
use toml;

use game::*;
use game::frontends::sdl::SdlKnowledgeRendererError;

use debug;

const TILESET_NAME: &'static str = "PxPlus_IBM_BIOS";
const FONT_NAME: &'static str = "PxPlus_IBM_BIOS";
const FONT_SIZE: u16 = 48;

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

    let (hud_spec, hud_path) = match parse_hud_spec(&args.resource_path) {
        Some(value) => value,
        None => return Err("Couldn't find hud".to_string()),
    };

    let hud = match frontends::sdl::Hud::new(hud_spec) {
        Ok(hud) => hud,
        Err(e) => return Err(format!("Couldn't parse hud: {:?}", e).to_string()),
    };

    let scale = args.config.graphics.scale;

    let video = sdl.video().map_err(|_| "Failed to connect to video subsystem")?;
    sdl2::image::init(INIT_PNG).map_err(|_| "Failed to connect to image subsystem")?;

    let ttf = sdl2::ttf::init().map_err(|_| "Failed to connect to ttf subsystem")?;
    let font_path = get_font_path(&args.resource_path);
    let font = ttf.load_font(&font_path, FONT_SIZE * scale as u16).map_err(|_| format!("Failed to load font {:?}", &font_path))?;

    let renderer = match frontends::sdl::SdlKnowledgeRenderer::new(
        &video,
        "Apocalypse Post",
        GAME_WIDTH,
        GAME_HEIGHT,
        tile_path,
        tileset,
        hud_path,
        hud,
        font,
        scale) {
        Ok(r) => r,
        Err(SdlKnowledgeRendererError::WindowCreationFailure) => return Err("Failed to create window".to_string()),
        Err(SdlKnowledgeRendererError::RendererInitialisationFailure) => return Err("Failed to initialise renderer".to_string()),
        Err(SdlKnowledgeRendererError::TileLoadFailure) => return Err("Failed to load tiles".to_string()),
        Err(SdlKnowledgeRendererError::HudLoadFailure) => return Err("Failed to load hud".to_string()),
    };

    let input = frontends::sdl::SdlInputSource::new(sdl.clone());

    let mut game = GameCtx::new(renderer,
                                input,
                                args.rng_seed,
                                GAME_WIDTH,
                                GAME_HEIGHT);

    game.run(args)?;

    Ok(())
}

fn parse_tileset_spec<P: AsRef<path::Path>>(resource_path: P) -> Option<(toml::value::Table, path::PathBuf)> {
    let base_path = resource_path.as_ref().join("tilesets").join(TILESET_NAME);
    game_file::read_toml(base_path.join("tiles.toml")).ok().map(|t| {
        (t, base_path.join("tiles.png"))
    })
}

fn parse_hud_spec<P: AsRef<path::Path>>(resource_path: P) -> Option<(toml::value::Table, path::PathBuf)> {
    let base_path = resource_path.as_ref().join("hud");
    game_file::read_toml(base_path.join("hud.toml")).ok().map(|t| {
        (t, base_path.join("hud.png"))
    })
}

fn get_font_path<P: AsRef<path::Path>>(resource_path: P) -> path::PathBuf {
    resource_path.as_ref().join("fonts").join(format!("{}.ttf", FONT_NAME))
}
