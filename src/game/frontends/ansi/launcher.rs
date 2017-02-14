use std::io;

use game::*;
use frontends::ansi::WindowAllocator;
use debug;

const DEBUG_WINDOW_WIDTH: usize = 80;
const DEBUG_WINDOW_HEIGHT: usize = 10;

const DEBUG_WINDOW_RIGHT_X: usize = 0;
const DEBUG_WINDOW_BOTTOM_Y: usize = 0;

const DEBUG_WINDOW_BORDER_X: usize = 2;
const DEBUG_WINDOW_BORDER_Y: usize = 1;

pub fn launch(args: Arguments) -> ExternalResult<()> {

    let window_allocator = Box::new(WindowAllocator::new().unwrap());

    let debug_buffer: Box<io::Write> = if args.debug {
        let mut debug_buffer = window_allocator.make_window_buffer(
            (window_allocator.width() - DEBUG_WINDOW_WIDTH - DEBUG_WINDOW_BORDER_X - DEBUG_WINDOW_RIGHT_X) as isize,
            (window_allocator.height() - DEBUG_WINDOW_HEIGHT - DEBUG_WINDOW_BORDER_Y - DEBUG_WINDOW_BOTTOM_Y) as isize,
            DEBUG_WINDOW_WIDTH,
            DEBUG_WINDOW_HEIGHT,
            DEBUG_WINDOW_BORDER_X,
            DEBUG_WINDOW_BORDER_Y);

        debug_buffer.draw_borders();

        Box::new(debug_buffer)
    } else {
        Box::new(debug::NullDebug)
    };

    debug::init(debug_buffer);

    let input_source = window_allocator.make_input_source();

    let renderer = match frontends::ansi::AnsiKnowledgeRenderer::new(window_allocator, GAME_WIDTH, GAME_HEIGHT, true) {
        Ok(r) => r,
        Err(frontends::ansi::AnsiKnowledgeRendererError::TerminalTooSmall { min_width, min_height }) => {
            return Err(format!("Terminal too small. Must be at least {}x{}.", min_width, min_height));
        },
    };

    let mut game = GameCtx::new(renderer, input_source, args.rng_seed, GAME_WIDTH, GAME_HEIGHT);

    game.run(args)?;

    Ok(())
}
