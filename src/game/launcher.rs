use game::*;

pub const GAME_WIDTH: usize = 41;
pub const GAME_HEIGHT: usize = 31;

pub fn launch(args: Arguments) -> ExternalResult<()> {

    match args.frontend {
        Frontend::Ansi => {

            #[cfg(all(unix, feature = "rustty"))]
            return frontends::ansi::launch(args);

            #[cfg(not(unix))]
            return Err("ansi frontend only available on unix".to_string());

            #[cfg(not(feature = "rustty"))]
            return Err("ansi frontend not supported".to_string());
        }
        Frontend::Sdl => {
            #[cfg(feature = "sdl2")]
            return frontends::sdl::launch(args);

            #[cfg(not(feature = "sdl2"))]
            return Err("sdl frontend not supported".to_string());
        }
    }
}
