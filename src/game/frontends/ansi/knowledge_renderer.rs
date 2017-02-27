use std::result;
use std::cmp;

use ecs::*;
use game::*;
use game::data::*;
use game::frontends::ansi::resolve_tile;
use frontends::ansi::{self, ComplexTile, SimpleTile, AnsiColour, Style};
use coord::Coord;
use colour::Rgb24;

const TEAR_COLOUR: ansi::AnsiColour = ansi::colours::MAGENTA;
const AIM_LINE_COLOUR: ansi::AnsiColour = ansi::colours::YELLOW;
const WOUND_OVERLAY_COLOUR: ansi::AnsiColour = ansi::colours::RED;
const DEATH_OVERLAY_COLOUR: ansi::AnsiColour = ansi::colours::RED;

const ANSI_GAME_WINDOW_X: usize = 1;
const ANSI_GAME_WINDOW_Y: usize = 1;

const MESSAGE_LOG_NUM_LINES: usize = 4;
const MESSAGE_LOG_PADDING_TOP: usize = 1;
const MESSAGE_LOG_PLAIN_COLOUR: Rgb24 = Rgb24 { red: 255, green: 255, blue: 255 };

const HUD_PADDING_TOP: usize = 0;
const HUD_TOTAL_HEIGHT: usize = HUD_PADDING_TOP + 1;
const HUD_TEXT_COLOUR: Rgb24 = Rgb24 { red: 255, green: 255, blue: 255 };

const SCROLL_BAR_COLOUR: Rgb24 = Rgb24 { red: 255, green: 255, blue: 255 };
const UNSEEN_BG: Rgb24 = Rgb24 { red: 0, green: 0, blue: 0 };
const UNSEEN_FG: Rgb24 = Rgb24 { red: 0x80, green: 0x80, blue: 0x80 };

const TEXT_BG: AnsiColour = AnsiColour::Rgb(ansi::RgbColour { red: 0, green: 0, blue: 0 });
const GAME_BG: AnsiColour = AnsiColour::Rgb(ansi::RgbColour { red: 0, green: 0, blue: 0 });
const CLEAR_COLOUR: AnsiColour = ansi::colours::BLACK;

const MENU_SELECTED_COLOUR: Rgb24 = Rgb24 { red: 255, green: 255, blue: 255 };
const MENU_DESELECTED_COLOUR: Rgb24 = Rgb24 { red: 127, green: 127, blue: 127 };

impl<'a> From<&'a CellDrawInfo> for AnsiInfo {
    fn from(cell: &'a CellDrawInfo) -> Self {
        let mut info = AnsiInfo::default();

        if let Some(bg_tile_type) = cell.background {
            let bg_tile = resolve_tile::resolve_tile(bg_tile_type);
            let tile = simple_tile(bg_tile, cell.front);
            if let Some(c) = tile.background_colour() {
                info.bg = c;
            }
            if let Some(c) = tile.character() {
                info.ch = c;
            }
            if let Some(s) = tile.style() {
                info.style = s;
            }
        }

        if let Some(fg_tile_type) = cell.foreground {
            let fg_tile = resolve_tile::resolve_tile(fg_tile_type);
            let tile = simple_tile(fg_tile, cell.front);
            if let Some(c) = tile.foreground_colour() {
                info.fg = c;
            }
            if let Some(c) = tile.character() {
                info.ch = c;
            }
            if let Some(s) = tile.style() {
                info.style = s;
            }
        }

        if cell.tear {
            info.bg = TEAR_COLOUR;
        }

        if let Some(health_overlay) = cell.health_overlay {
            if health_overlay.status() != HealthStatus::Healthy {
                info.bg = WOUND_OVERLAY_COLOUR;
            }
        }

        if !cell.visible {
            info.fg = ansi::AnsiColour::new_from_rgb24(UNSEEN_FG);
            info.bg = ansi::AnsiColour::new_from_rgb24(UNSEEN_BG);
        }

        info
    }
}

fn simple_tile(tile: ComplexTile, is_front: bool) -> SimpleTile {
    match tile {
        ComplexTile::Simple(s) => s,
        ComplexTile::Wall { front, back } => {
            if is_front {
                front
            } else {
                back
            }
        }
    }
}

fn render_message_part(window: &mut ansi::Window, part: &MessagePart, cursor: Coord) -> Coord {
    match part.as_text() {
        Some(text_part) => render_text_message_part(window, text_part, MESSAGE_LOG_PLAIN_COLOUR, cursor),
        None => cursor,
    }
}

fn render_text_message_part(window: &mut ansi::Window, part: &TextMessagePart, plain_colour: Rgb24, mut cursor: Coord) -> Coord {
    let (colour, string) = match *part {
        TextMessagePart::Plain(ref s) => (plain_colour, s),
        TextMessagePart::Colour(c, ref s) => (c, s),
    };

    let ansi_colour = ansi::AnsiColour::new_from_rgb24(colour);

    for ch in string.chars() {
        if cursor.x >= window.width() as isize {
            break;
        }

        assert!(cursor.x < window.width() as isize && cursor.y < window.height() as isize,
            "Cursor {:?} out of bounds ({}x{})", cursor, window.width(), window.height());

        window.get_cell(cursor.x, cursor.y).set(ch, ansi_colour, TEXT_BG, ansi::styles::NONE);

        cursor.x += 1;
    }

    cursor

}

fn clear_to_line_end(window: &mut ansi::Window, mut cursor: Coord) -> Coord {
    while cursor.x < window.width() as isize {
        window.get_cell(cursor.x, cursor.y).set(' ', TEXT_BG, TEXT_BG, ansi::styles::NONE);

        cursor.x += 1;
    }

    cursor
}

fn render_message(window: &mut ansi::Window, message: &Message, mut cursor: Coord) -> Coord {
    for part in message {
        cursor = render_message_part(window, part, cursor);
    }

    cursor = clear_to_line_end(window, cursor);

    cursor.x = 0;
    cursor.y += 1;

    cursor
}

fn render_text_message(window: &mut ansi::Window, message: &TextMessage, plain_colour: Rgb24, mut cursor: Coord) -> Coord {
    for part in message {
        cursor = render_text_message_part(window, part, plain_colour, cursor);
    }

    cursor = clear_to_line_end(window, cursor);

    cursor.x = 0;
    cursor.y += 1;

    cursor
}


struct AnsiInfo {
    ch: char,
    fg: AnsiColour,
    bg: AnsiColour,
    style: Style,
}

impl Default for AnsiInfo {
    fn default() -> Self {
        AnsiInfo {
            bg: GAME_BG,
            fg: ansi::colours::DARK_GREY,
            ch: ' ',
            style: ansi::styles::NONE,
        }
    }
}

pub struct AnsiKnowledgeRendererInner {
    window_allocator: Box<ansi::WindowAllocator>,
    window: ansi::Window,
    scroll_position: Coord,
    message_log_window: ansi::Window,
    total_width: usize,
    total_height: usize,
    top_left: Coord,
    hud_window: ansi::Window,
}

pub struct AnsiKnowledgeRenderer {
    renderer: AnsiKnowledgeRendererInner,
    buffers: RendererBuffers,
}

pub enum AnsiKnowledgeRendererError {
    TerminalTooSmall {
        min_width: usize,
        min_height: usize,
    },
}

impl AnsiKnowledgeRendererInner {
    pub fn new(window_allocator: Box<ansi::WindowAllocator>,
               game_width: usize,
               game_height: usize) -> result::Result<Self, AnsiKnowledgeRendererError> {

        if window_allocator.width() <= game_width || window_allocator.height() <= game_height {
            return Err(AnsiKnowledgeRendererError::TerminalTooSmall {
                min_width: game_width + ANSI_GAME_WINDOW_X,
                min_height: game_height + ANSI_GAME_WINDOW_Y,
            });
        }

        let window = window_allocator.make_window(
            ANSI_GAME_WINDOW_X as isize,
            ANSI_GAME_WINDOW_Y as isize,
            game_width,
            game_height,
            ansi::BufferType::Double);

        let message_log_window = window_allocator.make_window(
            ANSI_GAME_WINDOW_X as isize,
            (ANSI_GAME_WINDOW_Y + game_height + HUD_TOTAL_HEIGHT + MESSAGE_LOG_PADDING_TOP) as isize,
            game_width,
            MESSAGE_LOG_NUM_LINES,
            ansi::BufferType::Double);

        let hud_window = window_allocator.make_window(
            ANSI_GAME_WINDOW_X as isize,
            (ANSI_GAME_WINDOW_Y + game_height + HUD_PADDING_TOP) as isize,
            game_width,
            1,
            ansi::BufferType::Double);

        window_allocator.fill(CLEAR_COLOUR);
        window_allocator.flush();

        Ok(AnsiKnowledgeRendererInner {
            window_allocator: window_allocator,
            window: window,
            scroll_position: Coord::new(0, 0),
            message_log_window: message_log_window,
            total_width: game_width,
            total_height: game_height + HUD_TOTAL_HEIGHT + MESSAGE_LOG_PADDING_TOP + MESSAGE_LOG_NUM_LINES,
            top_left: Coord::new(ANSI_GAME_WINDOW_X as isize, ANSI_GAME_WINDOW_Y as isize),
            hud_window: hud_window,
        })
    }

    fn scroll_bar(&self, num_messages: usize, offset: usize, from_top: bool) -> Option<(Coord, usize)> {
        let num_lines = self.fullscreen_log_num_rows();
        if num_messages > num_lines {
            let scroll_bar_height = (self.total_height * num_lines) / num_messages;
            let remaining = self.total_height - scroll_bar_height;
            let max_offset = num_messages - num_lines;
            let scroll_position = (offset * remaining) / max_offset;
            let scroll_bar_top = if from_top {
                scroll_position
            } else {
                remaining - scroll_position
            };

            Some((Coord::new(self.total_width as isize - 1, scroll_bar_top as isize), scroll_bar_height))
        } else {
            None
        }
    }

    fn create_fullscreen_window(&self) -> ansi::Window {
        self.window_allocator.make_window(self.top_left.x, self.top_left.y,
                                          self.total_width, self.total_height,
                                          ansi::BufferType::Double)
    }

    fn display_wrapped_message_fullscreen_internal(&mut self,
                                                   window: &mut ansi::Window,
                                                   wrapped: &Vec<TextMessage>,
                                                   offset: usize) -> Coord {

        let mut cursor = Coord::new(0, 0);
        let end_idx = cmp::min(wrapped.len(), offset + self.total_height);

        for line in &wrapped[offset..end_idx] {
            cursor = render_text_message(window, line, MESSAGE_LOG_PLAIN_COLOUR, cursor);
        }

        let ret = cursor;

        let message = Message::new();
        while cursor.y < self.fullscreen_log_num_rows() as isize {
            cursor = render_message(window, &message, cursor);
        }

        if let Some((position, size)) = self.scroll_bar(wrapped.len(), offset, true) {
            let scroll_bar_colour = ansi::AnsiColour::new_from_rgb24(SCROLL_BAR_COLOUR);
            for i in 0..(size as isize) {
                let coord = position + Coord::new(0, i);
                window.get_cell(coord.x, coord.y).set(' ', scroll_bar_colour, scroll_bar_colour, ansi::styles::NONE);
            }
        }

        ret
    }

    fn delete_window(&mut self, mut window: ansi::Window) {
        window.flush();
        self.window_allocator.fill(CLEAR_COLOUR);
        window.delete();
    }

    fn fullscreen_log_num_rows(&self) -> usize {
        self.total_height
    }
}

impl AnsiKnowledgeRenderer {
    pub fn new(window_allocator: Box<ansi::WindowAllocator>,
               game_width: usize,
               game_height: usize) -> result::Result<Self, AnsiKnowledgeRendererError> {

        Ok(AnsiKnowledgeRenderer {
            renderer: AnsiKnowledgeRendererInner::new(window_allocator, game_width, game_height)?,
            buffers: RendererBuffers::new(game_width, game_height, MESSAGE_LOG_NUM_LINES),
        })
    }

    fn draw_message_log_internal(&mut self) {

        let mut cursor = Coord::new(0, 0);

        for line in &self.buffers.message_log {
            cursor = render_message(&mut self.renderer.message_log_window, line, cursor);
       }
    }

    fn draw_internal(&mut self) {
        for (coord, cell) in izip!(self.buffers.tiles.coord_iter(), self.buffers.tiles.iter()) {
            let info = AnsiInfo::from(cell);
            self.renderer.window.get_cell(coord.x, coord.y).set(info.ch, info.fg, info.bg, info.style);
        }
    }

    fn draw_overlay_internal(&mut self, overlay: &RenderOverlay) {
        match *overlay {
            RenderOverlay::AimLine(ref aim_line) => {
                for coord in aim_line.iter() {
                    let screen_coord = self.world_to_screen(coord);
                    if let Some(cell) = self.buffers.tiles.get(screen_coord) {
                        let mut info = AnsiInfo::from(cell);
                        info.bg = AIM_LINE_COLOUR;
                        self.renderer.window.get_cell(screen_coord.x, screen_coord.y).set(info.ch, info.fg, info.bg, info.style);
                    }
                }
            }
            RenderOverlay::ExamineCursor(examine_cursor) => {
                let screen_coord = self.world_to_screen(examine_cursor);
                if let Some(cell) = self.buffers.tiles.get(screen_coord) {
                    let mut info = AnsiInfo::from(cell);
                    info.bg = AIM_LINE_COLOUR;
                    self.renderer.window.get_cell(screen_coord.x, screen_coord.y).set(info.ch, info.fg, info.bg, info.style);
                }
            }
            RenderOverlay::Death => {
                for (coord, cell) in izip!(self.buffers.tiles.coord_iter(), self.buffers.tiles.iter()) {
                    let mut info = AnsiInfo::from(cell);
                    info.bg = DEATH_OVERLAY_COLOUR;
                    self.renderer.window.get_cell(coord.x, coord.y).set(info.ch, info.fg, info.bg, info.style);
                }
            }
        }
    }
}

impl KnowledgeRenderer for AnsiKnowledgeRenderer {

    fn width(&self) -> usize {
        self.renderer.window.width()
    }

    fn height(&self) -> usize {
        self.renderer.window.height()
    }

    fn world_offset(&self) -> Coord {
        self.renderer.scroll_position
    }

    fn update_game_window_buffer(&mut self, knowledge: &DrawableKnowledgeLevel, turn_id: u64, position: Coord) {
        self.renderer.scroll_position = self.centre_offset(position);
        self.buffers.tiles.update(knowledge, turn_id, self.renderer.scroll_position);
    }

    fn draw_game_window(&mut self) {
        self.draw_internal();
        self.renderer.window.flush();
    }

    fn draw_game_window_with_overlay(&mut self, overlay: &RenderOverlay) {
        self.draw_internal();
        self.draw_overlay_internal(overlay);
        self.renderer.window.flush();
    }

    fn draw_log(&mut self) {
        self.draw_message_log_internal();
        self.renderer.message_log_window.flush();
    }

    fn update_log_buffer(&mut self, messages: &MessageLog, language: &Box<Language>) {
        for (log_entry, message) in izip!(messages.tail(MESSAGE_LOG_NUM_LINES), &mut self.buffers.message_log) {
            message.clear();
            language.translate_repeated(log_entry.message, log_entry.repeated, message);
        }
    }

    fn fullscreen_log(&mut self, message_log: &MessageLog, offset: usize, language: &Box<Language>) {

        let mut window = self.renderer.create_fullscreen_window();

        let mut cursor = Coord::new(0, 0);
        let mut message = Message::new();
        let messages = message_log.tail_with_offset(self.fullscreen_log_num_rows(), offset);
        for log_entry in messages {
            message.clear();
            language.translate_repeated(log_entry.message, log_entry.repeated, &mut message);
            cursor = render_message(&mut window, &message, cursor);
        }

        message.clear();
        while cursor.y < self.fullscreen_log_num_rows() as isize {
            cursor = render_message(&mut window, &message, cursor);
        }

        if let Some((position, size)) = self.renderer.scroll_bar(message_log.len(), offset, false) {
            let scroll_bar_colour = ansi::AnsiColour::new_from_rgb24(SCROLL_BAR_COLOUR);
            for i in 0..(size as isize) {
                let coord = position + Coord::new(0, i);
                window.get_cell(coord.x, coord.y).set(' ', scroll_bar_colour, scroll_bar_colour, ansi::styles::NONE);
            }
        }

        self.renderer.delete_window(window);
    }

    fn fullscreen_log_num_rows(&self) -> usize {
        self.renderer.fullscreen_log_num_rows()
    }

    fn fullscreen_log_num_cols(&self) -> usize {
        self.renderer.total_width - 1
    }

    fn fullscreen_wrapped_translated_message(&mut self, wrapped: &Vec<TextMessage>, offset: usize) {

        let mut window = self.renderer.create_fullscreen_window();

        self.renderer.display_wrapped_message_fullscreen_internal(&mut window, wrapped, offset);

        self.renderer.delete_window(window);
    }

    fn draw_hud(&mut self, entity: EntityRef, _language: &Box<Language>) {

        let ansi_colour = ansi::AnsiColour::new_from_rgb24(HUD_TEXT_COLOUR);
        let mut cursor = 0;

        let hit_points = entity.hit_points().expect("Entity missing hit_points");

        let health_text = format!(" ❤ {}/{}", hit_points.current(), hit_points.max());

        for ch in health_text.chars() {
            if cursor >= self.renderer.hud_window.width() as isize {
                break;
            }

            self.renderer.hud_window.get_cell(cursor, 0).set(ch, ansi_colour, TEXT_BG, ansi::styles::NONE);
            cursor += 1;
        }

        while cursor < self.renderer.hud_window.width() as isize {
            self.renderer.hud_window.get_cell(cursor, 0).set(' ', TEXT_BG, TEXT_BG, ansi::styles::NONE);
            cursor += 1;
        }

        self.renderer.hud_window.flush();
    }

    fn fullscreen_menu<T>(&mut self, prelude: Option<MessageType>, menu: &Menu<T>, state: &MenuState, language: &Box<Language>) {

        let mut window = self.renderer.create_fullscreen_window();
        let mut message = Message::new();
        let mut wrapped = Vec::new();

        let mut cursor = if let Some(message_type) = prelude {
            language.translate(message_type, &mut message);

            self.fullscreen_wrap(&message, &mut wrapped);

            let mut cursor = self.renderer.display_wrapped_message_fullscreen_internal(&mut window, &wrapped, 0);
            cursor.y += 1;

            assert!(cursor.x < window.width() as isize && cursor.y < window.height() as isize,
                "Cursor {:?} out of bounds ({}x{})", cursor, window.width(), window.height());

            cursor
        } else {
            Coord::new(0, 0)
        };

        for (item_state, item) in state.iter(menu) {
            message.clear();
            language.translate(MessageType::Menu(item.message()), &mut message);

            wrapped.clear();
            self.fullscreen_wrap(&message, &mut wrapped);

            let colour = if item_state == MenuItemState::Selected {
                MENU_SELECTED_COLOUR
            } else {
                MENU_DESELECTED_COLOUR
            };

            assert!(cursor.x < window.width() as isize && cursor.y < window.height() as isize,
                "Cursor {:?} out of bounds ({}x{})", cursor, window.width(), window.height());

            assert!(wrapped.len() > 0, "{:?} {:?}", message, wrapped);
            cursor = render_text_message(&mut window, &wrapped[0], colour, cursor);
        }

        self.renderer.delete_window(window);
    }

    fn log_num_lines(&self) -> usize {
        MESSAGE_LOG_NUM_LINES
    }

    fn publish(&mut self) {}

    fn reset_buffers(&mut self) {
        self.buffers.reset();
    }
}
