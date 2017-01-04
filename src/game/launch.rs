use game::*;

pub fn launch(args: Arguments) -> ExternalResult<()> {

    match args.frontend {
        Frontend::Ansi => {

        #[cfg(unix)]
            frontends::ansi::launch(args)?;

        #[cfg(not(unix))]
            return Err("ansi frontend only available on unix");
        }
    }

    Ok(())
}
