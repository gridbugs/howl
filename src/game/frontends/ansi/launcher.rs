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

pub fn launch() -> ExternalResult<()> {
    let mut window_allocator = Box::new(WindowAllocator::new().unwrap());

    if window_allocator.width() <= GAME_WINDOW_WIDTH || window_allocator.height() <= GAME_WINDOW_HEIGHT {
        return Err(format!("Terminal too small. Must be at least {}x{}.",
                           GAME_WINDOW_WIDTH + 1,
                           GAME_WINDOW_HEIGHT + 1));
    }

    let input_source = InputSourceRef::new(&window_allocator.make_input_source());

    let game_window = window_allocator.make_window(
        ANSI_GAME_WINDOW_X as isize,
        ANSI_GAME_WINDOW_Y as isize,
        GAME_WINDOW_WIDTH,
        GAME_WINDOW_HEIGHT,
        BufferType::Double);

    let renderer = frontends::ansi::AnsiKnowledgeRenderer::new(game_window, false);

    let mut game = GameCtx::new(Box::new(renderer), input_source, GAME_WINDOW_WIDTH, GAME_WINDOW_HEIGHT);

    let mut debug_buffer = window_allocator.make_window_buffer(
        (window_allocator.width() - DEBUG_WINDOW_WIDTH - DEBUG_WINDOW_BORDER_X - DEBUG_WINDOW_RIGHT_X) as isize,
        (window_allocator.height() - DEBUG_WINDOW_HEIGHT - DEBUG_WINDOW_BORDER_Y - DEBUG_WINDOW_BOTTOM_Y) as isize,
        DEBUG_WINDOW_WIDTH,
        DEBUG_WINDOW_HEIGHT,
        DEBUG_WINDOW_BORDER_X,
        DEBUG_WINDOW_BORDER_Y);

    debug_buffer.draw_borders();

    debug::init(&mut debug_buffer);

    game.run()?;

    Ok(())
}
