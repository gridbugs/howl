use std::io;

use game::*;
use frontends::ansi::{WindowAllocator, BufferType};
use debug;

const GAME_WINDOW_WIDTH: usize = 41;
const GAME_WINDOW_HEIGHT: usize = 31;

const ANSI_GAME_WINDOW_X: usize = 1;
const ANSI_GAME_WINDOW_Y: usize = 1;

const DEBUG_WINDOW_WIDTH: usize = 80;
const DEBUG_WINDOW_HEIGHT: usize = 10;

const DEBUG_WINDOW_RIGHT_X: usize = 0;
const DEBUG_WINDOW_BOTTOM_Y: usize = 0;

const DEBUG_WINDOW_BORDER_X: usize = 2;
const DEBUG_WINDOW_BORDER_Y: usize = 1;

pub fn launch(args: Arguments) -> ExternalResult<()> {
    let mut window_allocator = Box::new(WindowAllocator::new().unwrap());

    if window_allocator.width() <= GAME_WINDOW_WIDTH || window_allocator.height() <= GAME_WINDOW_HEIGHT {
        return Err(format!("Terminal too small. Must be at least {}x{}.",
                           GAME_WINDOW_WIDTH + ANSI_GAME_WINDOW_X,
                           GAME_WINDOW_HEIGHT + ANSI_GAME_WINDOW_Y));
    }

    let input_source = window_allocator.make_input_source();
    let input_source_ref = InputSourceRef::new(&input_source);

    let game_window = window_allocator.make_window(
        ANSI_GAME_WINDOW_X as isize,
        ANSI_GAME_WINDOW_Y as isize,
        GAME_WINDOW_WIDTH,
        GAME_WINDOW_HEIGHT,
        BufferType::Double);

    let renderer = frontends::ansi::AnsiKnowledgeRenderer::new(game_window, false);

    let mut game = GameCtx::new(Box::new(renderer), input_source_ref, args.rng_seed, GAME_WINDOW_WIDTH, GAME_WINDOW_HEIGHT);

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

    game.run()?;

    Ok(())
}
