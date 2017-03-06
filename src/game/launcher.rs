use game::*;

pub const GAME_WIDTH: usize = 20;
pub const GAME_HEIGHT: usize = 15;

pub fn launch(args: Arguments) -> ExternalResult<()> {

    match args.frontend {
        Frontend::Sdl => {
            #[cfg(feature = "sdl2")]
            return frontends::sdl::launch(args);

            #[cfg(not(feature = "sdl2"))]
            return Err("sdl frontend not supported".to_string());
        }
    }
}
