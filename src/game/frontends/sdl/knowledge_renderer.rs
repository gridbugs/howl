use std::path;
use std::result;
use std::cmp;
use std::slice;
use std::mem;

use sdl2::VideoSubsystem;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture, BlendMode};
use sdl2::image::{LoadTexture, LoadSurface};
use sdl2::pixels::Color;
use sdl2::ttf::Font;
use sdl2::surface::Surface;

use ecs::*;
use game::*;
use game::data::*;
use game::frontends::sdl::{Tileset, Hud};

use coord::Coord;
use colour::{Rgb24, Rgba32};

const RENDERING_FAILED_MSG: &'static str = "Rendering failed";
const MESSAGE_LOG_NUM_LINES: usize = 4;
const MESSAGE_LOG_LINE_HEIGHT_PX: usize = 16;
const MESSAGE_LOG_PLAIN_COLOUR: Rgb24 = Rgb24 { red: 255, green: 255, blue: 255 };
const MESSAGE_LOG_PADDING_PX: usize = 4;
const MESSAGE_LOG_HEIGHT_PX: usize = (MESSAGE_LOG_LINE_HEIGHT_PX + MESSAGE_LOG_PADDING_PX) * MESSAGE_LOG_NUM_LINES;

const HUD_TOP_PADDING_PX: usize = 4;
const HUD_HEIGHT_PX: usize = 16;
const HUD_TOTAL_HEIGHT_PX: usize = HUD_TOP_PADDING_PX + HUD_HEIGHT_PX;
const HUD_TEXT_COLOUR: Rgb24 = Rgb24 { red: 255, green: 255, blue: 255 };
const HUD_HEALTH_LEN: usize = 5;
const HUD_SPEED_LEN: usize = 3;

const SCROLL_BAR_COLOUR: Rgb24 = Rgb24 { red: 255, green: 255, blue: 255 };
const SCROLL_BAR_WIDTH_PX: usize = 16;

const HEALTH_BAR_GREEN: Rgba32 = Rgba32 { red: 0, green: 127, blue: 0, alpha: 255 };
const HEALTH_BAR_RED: Rgba32 = Rgba32 { red: 127, green: 0, blue: 0, alpha: 255 };
const HEALTH_BAR_HEIGHT_PX: usize = 2;

const MENU_SELECTED_COLOUR: Rgb24 = Rgb24 { red: 255, green: 255, blue: 255 };
const MENU_DESELECTED_COLOUR: Rgb24 = Rgb24 { red: 127, green: 127, blue: 127 };

const BOTTOM_PADDING_PX: usize = 10;
const LEFT_PADDING_PX: usize = 10;

fn rgb24_to_sdl_colour(rgb24: Rgb24) -> Color {
    Color::RGB(rgb24.red, rgb24.green, rgb24.blue)
}

fn rgba32_to_sdl_colour(rgba32: Rgba32) -> Color {
    Color::RGBA(rgba32.red, rgba32.green, rgba32.blue, rgba32.alpha)
}

fn create_greyscale_tile_texture<P: AsRef<path::Path>>(renderer: &Renderer, tile_path: P) -> result::Result<Texture, String> {
    let tile_surface = Surface::from_file(tile_path)?;

    let size = (tile_surface.width() * tile_surface.height()) as usize;

    let pixels = unsafe {
        let pixels_ptr = (&mut *tile_surface.raw()).pixels as *mut u32;
        slice::from_raw_parts_mut(pixels_ptr, size)
    };

    for pixel in pixels.iter_mut() {

        const R: usize = 0;
        const G: usize = 1;
        const B: usize = 2;

        let mut arr = unsafe { mem::transmute::<u32, [u8; 4]>(*pixel) };
        let max = cmp::max(arr[R], cmp::max(arr[G], arr[B])) as u32;
        let darkened = ((max * 1) / 3) as u8;

        arr[R] = darkened;
        arr[G] = darkened;
        arr[B] = darkened;

        *pixel = unsafe { mem::transmute::<[u8; 4], u32>(arr) };
    }

    renderer.create_texture_from_surface(tile_surface).map_err(|e| format!("{}", e))
}

struct SdlCellInfo {
    fg: Option<Rect>,
    bg: Option<Rect>,
    visible: bool,
    health_overlay: Option<HitPoints>,
}

struct GameTextures {
    colour: Texture,
    greyscale: Texture,
}

pub struct SdlKnowledgeRendererInternal<'a, 'b> {
    sdl_renderer: Renderer<'static>,
    font: Font<'a, 'b>,
    tileset: Tileset,
    hud_texture: Texture,
    hud: Hud,
    width: usize,
    height: usize,
    tile_width_px: usize,
    tile_height_px: usize,
    game_width_px: usize,
    game_height_px: usize,
    total_width_px: usize,
    total_height_px: usize,
    clear_colour: Color,
    game_rect: Rect,
    screen_rect: Rect,
    message_log_position: Coord,
    message_log_rect: Rect,
    display_log_num_lines: usize,
    display_log_num_cols: usize,
    scroll_position: Coord,
    hud_position: Coord,
    hud_rect: Rect,
    scale: usize,
    zoom: usize,
}

pub struct SdlKnowledgeRenderer<'a, 'b> {
    buffers: RendererBuffers,
    renderer: SdlKnowledgeRendererInternal<'a, 'b>,
    textures: GameTextures,
}

#[derive(Debug)]
pub enum SdlKnowledgeRendererError {
    WindowCreationFailure,
    RendererInitialisationFailure,
    TileLoadFailure,
    HudLoadFailure,
}

impl GameTextures {
    fn new<P: AsRef<path::Path>>(renderer: &Renderer, path: P) -> Self {
        let tile_texture = renderer.load_texture(&path).expect("Failed to load texture");
        let greyscale_tile_texture = create_greyscale_tile_texture(renderer, path)
            .expect("Failed to create greyscale texture");

        GameTextures {
            colour: tile_texture,
            greyscale: greyscale_tile_texture,
        }
    }
}

impl<'a, 'b> SdlKnowledgeRendererInternal<'a, 'b> {
    fn new<P: AsRef<path::Path>>(
        video: &VideoSubsystem,
        title: &str,
        game_width: usize,
        game_height: usize,
        tileset: Tileset,
        hud_path: P,
        hud: Hud,
        font: Font<'a, 'b>,
        scale: usize,
        zoom: usize) -> result::Result<Self, SdlKnowledgeRendererError> {


        let tile_width_px = tileset.tile_width() * scale * zoom;
        let tile_height_px = tileset.tile_height() * scale * zoom;

        let game_width_px = game_width * tile_width_px;
        let game_height_px = game_height * tile_height_px;

        let message_log_width_px = game_width_px;
        let message_log_height_px = MESSAGE_LOG_HEIGHT_PX * scale;
        let message_log_line_height_px = MESSAGE_LOG_LINE_HEIGHT_PX * scale;
        let message_log_padding_px = MESSAGE_LOG_PADDING_PX * scale;
        let message_log_total_line_height_px = message_log_line_height_px + message_log_padding_px;

        let hud_width_px = game_width_px;
        let hud_height_px = HUD_TOTAL_HEIGHT_PX * scale;

        let total_width_px = game_width_px;
        let total_height_px = game_height_px + message_log_height_px + hud_height_px + BOTTOM_PADDING_PX;

        let window = video.window(title, total_width_px as u32, total_height_px as u32)
            .build()
            .map_err(|_| SdlKnowledgeRendererError::WindowCreationFailure)?;

        let mut renderer = window.renderer()
            .build()
            .map_err(|_| SdlKnowledgeRendererError::RendererInitialisationFailure)?;

        renderer.set_blend_mode(BlendMode::Blend);

        let hud_texture = renderer.load_texture(hud_path).map_err(|_| SdlKnowledgeRendererError::HudLoadFailure)?;

        let message_log_position = Coord::new(0, (game_height_px + hud_height_px) as isize);
        let message_log_rect = Rect::new(message_log_position.x as i32,
                                         message_log_position.y as i32,
                                         message_log_width_px as u32,
                                         message_log_height_px as u32);

        let hud_position = Coord::new(0, game_height_px as isize);

        Ok(SdlKnowledgeRendererInternal {
            sdl_renderer: renderer,
            font: font,
            width: game_width,
            height: game_height,
            tile_width_px: tileset.tile_width(),
            tile_height_px: tileset.tile_height(),
            game_width_px: game_width_px,
            game_height_px: game_height_px,
            total_width_px: total_width_px,
            total_height_px: total_height_px,
            tileset: tileset,
            clear_colour: Color::RGB(0, 0, 0),
            game_rect: Rect::new(0, 0, game_width_px as u32, game_height_px as u32),
            screen_rect: Rect::new(0, 0, total_width_px as u32, total_height_px as u32),
            message_log_position: message_log_position,
            message_log_rect: message_log_rect,
            scroll_position: Coord::new(0, 0),
            display_log_num_lines: total_height_px / message_log_total_line_height_px,
            display_log_num_cols: (total_width_px - message_log_padding_px * 2) / message_log_line_height_px, // square fonts only
            hud_position: hud_position,
            hud_texture: hud_texture,
            hud: hud,
            hud_rect: Rect::new(hud_position.x as i32, hud_position.y as i32, hud_width_px as u32, hud_height_px as u32),
            scale: scale,
            zoom: zoom,
        })
    }

    fn text_line_height_px(&self) -> usize {
       MESSAGE_LOG_LINE_HEIGHT_PX * self.scale
    }

    fn text_line_padded_height_px(&self) -> usize {
       (MESSAGE_LOG_LINE_HEIGHT_PX + MESSAGE_LOG_PADDING_PX) * self.scale
    }

    fn text_padding_px(&self) -> usize {
        MESSAGE_LOG_PADDING_PX * self.scale
    }

    fn tile_width_px(&self) -> usize {
        self.tileset.tile_width() * self.scale * self.zoom
    }

    fn tile_height_px(&self) -> usize {
        self.tileset.tile_height() * self.scale * self.zoom
    }

    fn health_bar_height_px(&self) -> usize {
        HEALTH_BAR_HEIGHT_PX * self.scale * self.zoom
    }

    fn scroll_bar_width_px(&self) -> usize {
        SCROLL_BAR_WIDTH_PX * self.scale
    }

    fn hud_height_px(&self) -> usize {
        HUD_HEIGHT_PX * self.scale
    }

    fn hud_padded_height_px(&self) -> usize {
        (HUD_HEIGHT_PX + HUD_TOP_PADDING_PX) * self.scale
    }

    fn screen_rect(&self, coord: Coord) -> Rect {
        let width = self.tile_width_px() as i32;
        let height = self.tile_height_px() as i32;

        Rect::new(coord.x as i32 * width, coord.y as i32 * height, width as u32, height as u32)
    }

    fn simple_tile(tile: frontends::sdl::ComplexTile, is_front: bool) -> frontends::sdl::SimpleTile {
        match tile {
            frontends::sdl::ComplexTile::Simple(s) => s,
            frontends::sdl::ComplexTile::Wall { front, back } => {
                if is_front {
                    front
                } else {
                    back
                }
            }
        }
    }

    fn to_sdl_info(&self, cell: &CellDrawInfo) -> SdlCellInfo {
        let mut info = SdlCellInfo {
            visible: cell.visible,
            fg: None,
            bg: None,
            health_overlay: cell.health_overlay,
        };

        if let Some(bg_type) = cell.background {
            let complex_tile = self.tileset.resolve(bg_type);
            let tile = Self::simple_tile(*complex_tile, cell.front);
            info.bg = tile.background();
            info.fg = tile.foreground();
        }

        if let Some(fg_type) = cell.foreground {
            let complex_tile = self.tileset.resolve(fg_type);
            let tile = Self::simple_tile(*complex_tile, cell.front);
            if let Some(fg) = tile.foreground() {
                info.fg = Some(fg);
            }
        }

        info
    }

    fn clear_game(&mut self) {
        self.sdl_renderer.set_draw_color(self.clear_colour);
        self.sdl_renderer.fill_rect(self.game_rect).expect("Failed to clear game");
    }

    fn clear_hud(&mut self) {
        self.sdl_renderer.set_draw_color(self.clear_colour);
        self.sdl_renderer.fill_rect(self.hud_rect).expect("Failed to clear hud");
    }

    fn clear_message_log(&mut self) {
        self.sdl_renderer.set_draw_color(self.clear_colour);
        self.sdl_renderer.fill_rect(self.message_log_rect).expect("Failed to clear message_log");
    }

    fn clear_screen(&mut self) {
        self.sdl_renderer.set_draw_color(self.clear_colour);
        self.sdl_renderer.fill_rect(self.screen_rect).expect("Failed to clear screen");
    }


    fn render_message_part(&mut self, part: &MessagePart, cursor: Coord) -> Coord {
        match part.as_text() {
            Some(text_part) => self.render_text_message_part(MESSAGE_LOG_PLAIN_COLOUR, text_part, cursor),
            None => cursor,
        }
    }

    fn render_text_message_part(&mut self, plain_colour: Rgb24, part: &TextMessagePart, mut cursor: Coord) -> Coord {
        let (colour, string) = match *part {
            TextMessagePart::Plain(ref s) => (plain_colour, s),
            TextMessagePart::Colour(c, ref s) => (c, s),
        };

        let sdl_colour = rgb24_to_sdl_colour(colour);
        let surface = self.font.render(string).solid(sdl_colour).expect("Failed to create text surface");
        let texture = self.sdl_renderer.create_texture_from_surface(&surface).expect("Failed to create text texture");

        // assume fixed-width, square font
        let text_width = string.len() * self.text_line_height_px();
        let text_rect = Rect::new(cursor.x as i32, cursor.y as i32, text_width as u32,
                                  self.text_line_height_px() as u32);
        self.sdl_renderer.copy(&texture, None, Some(text_rect)).expect("Failed to render text");
        cursor.x += text_width as isize;

        cursor
    }

    fn render_message(&mut self, message: &Message, cursor: Coord) -> Coord {
        let mut tmp_cursor = cursor;
        for part in message {
            tmp_cursor = self.render_message_part(part, tmp_cursor);
        }
        tmp_cursor.x = cursor.x;
        tmp_cursor.y += self.text_line_padded_height_px() as isize;

        tmp_cursor
    }

    fn render_text_message(&mut self, plain_colour: Rgb24, message: &TextMessage, cursor: Coord) -> Coord {
        let mut tmp_cursor = cursor;
        for part in message {
            tmp_cursor = self.render_text_message_part(plain_colour, part, tmp_cursor);
        }
        tmp_cursor.x = cursor.x;
        tmp_cursor.y += self.text_line_padded_height_px() as isize;

        tmp_cursor
    }

    fn scroll_bar_rect(&self, num_messages: usize, offset: usize, from_top: bool) -> Option<Rect> {
        let num_lines = self.fullscreen_log_num_rows();
        if num_messages > num_lines {
            let scroll_bar_height_px = (self.total_height_px * num_lines) / num_messages;
            let remaining_px = self.total_height_px - scroll_bar_height_px;
            let max_offset = num_messages - num_lines;
            let scroll_px = (offset * remaining_px) / max_offset;
            let scroll_bar_top_px = if from_top {
                scroll_px
            } else {
                remaining_px - scroll_px
            };
            let scroll_bar_left_px = self.total_width_px - self.scroll_bar_width_px();
            Some(Rect::new(scroll_bar_left_px as i32, scroll_bar_top_px as i32,
                           self.scroll_bar_width_px() as u32, scroll_bar_height_px as u32))
        } else {
            None
        }
    }

    fn fullscreen_initial_cursor(&self) -> Coord {
        Coord::new(MESSAGE_LOG_PADDING_PX as isize, MESSAGE_LOG_PADDING_PX as isize)
    }

    fn display_wrapped_message_fullscreen_internal(&mut self, wrapped: &Vec<TextMessage>, offset: usize) -> Coord {
        let mut cursor = self.fullscreen_initial_cursor();

        let end_idx = cmp::min(wrapped.len(), offset + self.fullscreen_log_num_rows());

        for line in &wrapped[offset..end_idx] {
            cursor = self.render_text_message(MESSAGE_LOG_PLAIN_COLOUR, line, cursor);
            cursor.y += MESSAGE_LOG_PADDING_PX as isize;
        }

        if let Some(scroll_bar) = self.scroll_bar_rect(wrapped.len(), offset, true) {
            self.sdl_renderer.set_draw_color(rgb24_to_sdl_colour(SCROLL_BAR_COLOUR));
            self.sdl_renderer.fill_rect(scroll_bar).expect("Failed to draw scroll bar");
        }

        cursor
    }

    fn fullscreen_log_num_rows(&self) -> usize {
        self.display_log_num_lines
    }

    fn draw_overlay_cell(&mut self, cell: &CellDrawInfo, screen_coord: Coord, overlay: Rect, textures: &GameTextures) {
        let dest = self.screen_rect(screen_coord);
        let info = self.to_sdl_info(cell);

        self.sdl_renderer.copy(&textures.colour,
                               Some(overlay),
                               Some(dest)).expect(RENDERING_FAILED_MSG);

        if let Some(fg_rect) = info.fg {
            let texture = if info.visible {
                &textures.colour
            } else {
                &textures.greyscale
            };
            self.sdl_renderer.copy(texture, Some(fg_rect), Some(dest)).expect(RENDERING_FAILED_MSG);
        }
        if let Some(health_overlay) = info.health_overlay {
            self.draw_health_bar(screen_coord, health_overlay);
        }
    }

    fn draw_health_bar(&mut self, coord: Coord, health_overlay: HitPoints) {
        if !health_overlay.is_full() {
            let red = rgba32_to_sdl_colour(HEALTH_BAR_RED);
            let green = rgba32_to_sdl_colour(HEALTH_BAR_GREEN);

            let health_bar_green_px = if health_overlay.umax() == 0 {
                0
            } else {
                (self.tile_width_px() * health_overlay.ucurrent()) / health_overlay.umax()
            };

            let y_coord = coord.y as i32 * self.tile_height_px() as i32
                + (self.tile_height_px() - self.health_bar_height_px()) as i32;

            let health_bar_red_rect = Rect::new(coord.x as i32 * self.tile_width_px() as i32
                                                    + health_bar_green_px as i32,
                                                y_coord,
                                                (self.tile_width_px() - health_bar_green_px) as u32,
                                                self.health_bar_height_px() as u32);

            self.sdl_renderer.set_draw_color(red);
            self.sdl_renderer.fill_rect(health_bar_red_rect).expect("Failed to draw health bar red rect");

            if health_bar_green_px > 0 {
                let health_bar_green_rect = Rect::new(coord.x as i32 * self.tile_width_px() as i32,
                                                      y_coord,
                                                      health_bar_green_px as u32,
                                                      self.health_bar_height_px() as u32);

                self.sdl_renderer.set_draw_color(green);
                self.sdl_renderer.fill_rect(health_bar_green_rect).expect("Failed to draw health bar green rect");
            }
        }
    }

    fn draw_cell(&mut self, cell: &CellDrawInfo, coord: Coord, textures: &GameTextures) {
        let rect = self.screen_rect(coord);
        let info = self.to_sdl_info(cell);

        self.sdl_renderer.copy(&textures.colour, Some(self.tileset.extra.blank), Some(rect)).expect(RENDERING_FAILED_MSG);

        let texture = if info.visible {
            &textures.colour
        } else {
            &textures.greyscale
        };

        if let Some(bg_rect) = info.bg {
            self.sdl_renderer.copy(texture, Some(bg_rect), Some(rect)).expect(RENDERING_FAILED_MSG);
        }

        if let Some(fg_rect) = info.fg {
            self.sdl_renderer.copy(texture, Some(fg_rect), Some(rect)).expect(RENDERING_FAILED_MSG);
        }

        if let Some(health_overlay) = info.health_overlay {
            self.draw_health_bar(coord, health_overlay);
        }
    }
}

impl<'a, 'b> SdlKnowledgeRenderer<'a, 'b> {

    pub fn new<P: AsRef<path::Path>, Q: AsRef<path::Path>>(
        video: &VideoSubsystem,
        title: &str,
        game_width: usize,
        game_height: usize,
        tile_path: P,
        tileset: Tileset,
        hud_path: Q,
        hud: Hud,
        font: Font<'a, 'b>,
        scale: usize,
        zoom: usize) -> result::Result<Self, SdlKnowledgeRendererError> {

        let renderer = SdlKnowledgeRendererInternal::new(video, title, game_width, game_height,
                                                         tileset, hud_path, hud, font, scale, zoom)?;

        let buffers = RendererBuffers::new(game_width, game_height, MESSAGE_LOG_NUM_LINES);

        let game_textures = GameTextures::new(&renderer.sdl_renderer, tile_path);

        Ok(SdlKnowledgeRenderer {
            buffers: buffers,
            renderer: renderer,
            textures: game_textures,
        })
    }

    fn draw_message_log_internal(&mut self) {
        self.renderer.clear_message_log();
        let mut cursor = self.renderer.message_log_position + Coord::new(LEFT_PADDING_PX as isize, MESSAGE_LOG_PADDING_PX as isize);

        for line in &self.buffers.message_log {
            cursor = self.renderer.render_message(line, cursor);
        }
    }

    fn draw_internal(&mut self) {
        for (coord, cell) in izip!(self.buffers.tiles.coord_iter(), self.buffers.tiles.iter()) {
            self.renderer.draw_cell(cell, coord, &self.textures);
        }
    }

    fn draw_overlay_internal(&mut self, overlay: &RenderOverlay) {
        match *overlay {
            RenderOverlay::Death => {
                for (coord, cell) in izip!(self.buffers.tiles.coord_iter(), self.buffers.tiles.iter()) {
                    let death_rect = self.renderer.tileset.extra.death;
                    self.renderer.draw_overlay_cell(cell, coord, death_rect, &self.textures);
                }
            }
        }
    }

    fn draw_hud_component(&mut self, symbol: Rect, text: String, mut cursor: usize) -> usize {
        let symbol_rect = Rect::new((self.renderer.hud_position.x + cursor as isize) as i32,
                                    (self.renderer.hud_position.y + HUD_TOP_PADDING_PX as isize) as i32,
                                    self.renderer.hud_height_px() as u32,
                                    self.renderer.hud_height_px() as u32);

        self.renderer.sdl_renderer.copy(&self.renderer.hud_texture,
                                        Some(symbol),
                                        Some(symbol_rect)).expect("Failed to render symbol");

        cursor += self.renderer.hud_padded_height_px();

        let sdl_colour = rgb24_to_sdl_colour(HUD_TEXT_COLOUR);
        let surface = self.renderer.font.render(text.as_ref()).solid(sdl_colour).expect("Failed to create text surface");
        let texture = self.renderer.sdl_renderer.create_texture_from_surface(&surface).expect("Failed to create text texture");
        let text_width = text.len() * self.renderer.hud_height_px(); // square fonts

        let text_rect = Rect::new((self.renderer.hud_position.x + cursor as isize) as i32,
                                  (self.renderer.hud_position.y + HUD_TOP_PADDING_PX as isize) as i32,
                                  text_width as u32,
                                  self.renderer.hud_height_px() as u32);

        self.renderer.sdl_renderer.copy(&texture, None, Some(text_rect)).expect("Failed to render text");

        cursor += text_width + self.renderer.hud_height_px();

        cursor
    }
}

impl<'a, 'b> KnowledgeRenderer for SdlKnowledgeRenderer<'a, 'b> {
    fn width(&self) -> usize {
        self.renderer.width
    }

    fn height(&self) -> usize {
        self.renderer.height
    }

    fn world_offset(&self) -> Coord {
        self.renderer.scroll_position
    }

    fn update_game_window_buffer(&mut self, knowledge: &DrawableKnowledgeLevel, turn_id: u64, player_position: Coord) {
        self.renderer.scroll_position = self.centre_offset(player_position);
        self.buffers.tiles.update(knowledge, turn_id, self.renderer.scroll_position);
    }

    fn draw_game_window(&mut self) {
        self.renderer.clear_game();
        self.draw_internal();
    }

    fn draw_game_window_with_overlay(&mut self, overlay: &RenderOverlay) {
        self.draw_game_window();
        self.draw_overlay_internal(overlay);
    }

    fn draw_log(&mut self) {
        self.draw_message_log_internal();
    }

    fn update_log_buffer(&mut self, messages: &MessageLog, language: &Box<Language>) {
        for (log_entry, message) in izip!(messages.tail(MESSAGE_LOG_NUM_LINES), &mut self.buffers.message_log) {
            message.clear();
            language.translate_repeated(log_entry.message, log_entry.repeated, message);
        }
    }

    fn fullscreen_log(&mut self, message_log: &MessageLog, offset: usize, language: &Box<Language>) {
        self.renderer.clear_screen();

        let mut cursor = Coord::new(MESSAGE_LOG_PADDING_PX as isize, MESSAGE_LOG_PADDING_PX as isize);
        let mut message = Message::new();

        let messages = message_log.tail_with_offset(self.fullscreen_log_num_rows(), offset);

        for log_entry in messages {
            message.clear();
            language.translate_repeated(log_entry.message, log_entry.repeated, &mut message);
            cursor = self.renderer.render_message(&message, cursor);
        }

        if let Some(scroll_bar_rect) = self.renderer.scroll_bar_rect(message_log.len(), offset, false) {
            self.renderer.sdl_renderer.set_draw_color(rgb24_to_sdl_colour(SCROLL_BAR_COLOUR));
            self.renderer.sdl_renderer.fill_rect(scroll_bar_rect).expect("Failed to draw scroll bar");
        }
    }

    fn fullscreen_log_num_rows(&self) -> usize {
        self.renderer.fullscreen_log_num_rows()
    }

    fn fullscreen_log_num_cols(&self) -> usize {
        self.renderer.display_log_num_cols - 1
    }

    fn fullscreen_wrapped_translated_message(&mut self, wrapped: &Vec<TextMessage>, offset: usize) {
        self.renderer.clear_screen();
        self.renderer.display_wrapped_message_fullscreen_internal(wrapped, offset);
    }


    fn draw_hud(&mut self, entity: EntityRef, _language: &Box<Language>) {
        self.renderer.clear_hud();
        let mut cursor = LEFT_PADDING_PX;

        let hit_points = entity.hit_points().expect("Entity missing hit_points");
        let hit_points_text = format!("{}/{}", hit_points.current(), hit_points.max());
        let hit_points_symbol = self.renderer.hud.health;
        cursor = self.draw_hud_component(hit_points_symbol, hit_points_text, cursor);

        let engine = entity.engine_health().expect("Entity missing engine_health");
        let engine_text = format!("{}/{}", engine.current(), engine.max());
        let engine_symbol = self.renderer.hud.engine;
        cursor = self.draw_hud_component(engine_symbol, engine_text, cursor);

        let tyres = entity.tyre_health().expect("Entity missing tyre_health");
        let tyres_text = format!("{}/{}", tyres.current(), tyres.max());
        let tyre_symbol = self.renderer.hud.tyres;
        cursor = self.draw_hud_component(tyre_symbol, tyres_text, cursor);

        let armour = entity.armour().expect("Entity missing armour");
        let armour_text = format!("{}", armour);
        let armour_symbol = self.renderer.hud.armour;
        cursor = self.draw_hud_component(armour_symbol, armour_text, cursor);

        let speed = entity.current_speed().expect("Entity missing current_speed");
        let max_speed = entity.player_max_speed().expect("Entity missing max_speed");
        let speed_text = format!("{}/{}", speed, max_speed);
        let speed_symbol = self.renderer.hud.speed;
        cursor = self.draw_hud_component(speed_symbol, speed_text, cursor);

        let letters = entity.letter_count().expect("Entity missing armour");
        let letters_text = format!("{}", letters);
        let letters_symbol = self.renderer.hud.letter;
        self.draw_hud_component(letters_symbol, letters_text, cursor);
    }

    fn fullscreen_menu<T>(&mut self, prelude: Option<MessageType>, menu: &SelectMenu<T>, state: &SelectMenuState, language: &Box<Language>) {

        let mut message = Message::new();
        let mut wrapped = Vec::new();

        self.renderer.clear_screen();

        let mut cursor = if let Some(message_type) = prelude {
            language.translate(message_type, &mut message);

            self.fullscreen_wrap(&message, &mut wrapped);

            let mut cursor = self.renderer.display_wrapped_message_fullscreen_internal(&wrapped, 0);

            cursor.y += self.renderer.text_line_padded_height_px() as isize;

            cursor
        } else {
            self.renderer.fullscreen_initial_cursor()
        };

        for (item_state, item) in state.iter(menu) {
            message.clear();
            language.translate(MessageType::Menu(item.message()), &mut message);

            wrapped.clear();
            self.fullscreen_wrap(&message, &mut wrapped);

            let colour = if item_state == SelectMenuItemState::Selected {
                MENU_SELECTED_COLOUR
            } else {
                MENU_DESELECTED_COLOUR
            };

            cursor = self.renderer.render_text_message(colour, &wrapped[0], cursor);
            cursor.y += self.renderer.text_padding_px() as isize;
        }
    }

    fn publish(&mut self) {
        self.renderer.sdl_renderer.present();
    }

    fn log_num_lines(&self) -> usize {
        MESSAGE_LOG_NUM_LINES
    }

    fn reset_buffers(&mut self) {
        self.buffers.reset();
    }
}
