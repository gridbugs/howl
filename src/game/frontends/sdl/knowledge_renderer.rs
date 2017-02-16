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
use game::frontends::sdl::{Tileset, ExtraTileType, Hud};

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

const SCROLL_BAR_COLOUR: Rgb24 = Rgb24 { red: 255, green: 255, blue: 255 };
const SCROLL_BAR_WIDTH_PX: usize = 16;

const HEALTH_BAR_GREEN: Rgba32 = Rgba32 { red: 0, green: 255, blue: 0, alpha: 255 };
const HEALTH_BAR_RED: Rgba32 = Rgba32 { red: 255, green: 0, blue: 0, alpha: 255 };
const HEALTH_BAR_HEIGHT_PX: usize = 2;

const MENU_SELECTED_COLOUR: Rgb24 = Rgb24 { red: 255, green: 255, blue: 255 };
const MENU_DESELECTED_COLOUR: Rgb24 = Rgb24 { red: 127, green: 127, blue: 127 };

struct SdlCellInfo {
    fg: Option<Rect>,
    bg: Option<Rect>,
    moon: bool,
    visible: bool,
    health_overlay: Option<HitPoints>,
}

pub struct SdlKnowledgeRenderer<'a> {
    buffer: TileBuffer,
    sdl_renderer: Renderer<'static>,
    tile_texture: Texture,
    greyscale_tile_texture: Texture,
    font: Font<'a>,
    width: usize,
    height: usize,
    tile_width_px: usize,
    tile_height_px: usize,
    game_width_px: usize,
    game_height_px: usize,
    width_px: usize,
    height_px: usize,
    tileset: Tileset,
    clear_colour: Color,
    game_rect: Rect,
    screen_rect: Rect,
    message_log_position: Coord,
    message_log_rect: Rect,
    message_log: Vec<Message>,
    display_log_num_lines: usize,
    display_log_num_cols: usize,
    scroll_position: Coord,
    hud_position: Coord,
    hud_texture: Texture,
    hud: Hud,
    hud_rect: Rect,
    scale: usize,
}

#[derive(Debug)]
pub enum SdlKnowledgeRendererError {
    WindowCreationFailure,
    RendererInitialisationFailure,
    TileLoadFailure,
    HudLoadFailure,
}

impl<'a> SdlKnowledgeRenderer<'a> {

    fn create_greyscale_tile_texture(renderer: &Renderer, tile_path: &path::PathBuf) -> result::Result<Texture, String> {
        let tile_surface = Surface::from_file(&tile_path)?;

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

    pub fn new(video: &VideoSubsystem,
               title: &str,
               game_width: usize,
               game_height: usize,
               tile_path: path::PathBuf,
               tileset: Tileset,
               hud_path: path::PathBuf,
               hud: Hud,
               font: Font<'a>,
               scale: usize) -> result::Result<Self, SdlKnowledgeRendererError> {

        let game_width_px = (game_width * tileset.tile_width()) as usize * scale;
        let game_height_px = (game_height * tileset.tile_height()) as usize * scale;
        let width_px = game_width_px as u32;
        let height_px = (game_height_px + MESSAGE_LOG_HEIGHT_PX * scale + HUD_TOTAL_HEIGHT_PX * scale) as u32;
        let window = video.window(title, width_px, height_px)
            .build()
            .map_err(|_| SdlKnowledgeRendererError::WindowCreationFailure)?;

        let mut renderer = window.renderer()
            .build()
            .map_err(|_| SdlKnowledgeRendererError::RendererInitialisationFailure)?;

        renderer.set_blend_mode(BlendMode::Blend);

        let tile_texture = renderer.load_texture(&tile_path).map_err(|_| SdlKnowledgeRendererError::TileLoadFailure)?;
        let hud_texture = renderer.load_texture(&hud_path).map_err(|_| SdlKnowledgeRendererError::HudLoadFailure)?;
        let greyscale_tile_texture = Self::create_greyscale_tile_texture(&renderer, &tile_path).unwrap();

        let mut message_log = Vec::new();
        for _ in 0..MESSAGE_LOG_NUM_LINES {
            message_log.push(Message::new());
        }

        let message_log_position = Coord::new(0, (game_height_px + HUD_TOTAL_HEIGHT_PX * scale) as isize);
        let message_log_rect = Rect::new(message_log_position.x as i32,
                                         message_log_position.y as i32,
                                         width_px,
                                         (MESSAGE_LOG_HEIGHT_PX * scale) as u32);

        let hud_position = Coord::new(0, game_height_px as isize);

        Ok(SdlKnowledgeRenderer {
            buffer: TileBuffer::new(game_width, game_height),
            sdl_renderer: renderer,
            tile_texture: tile_texture,
            greyscale_tile_texture: greyscale_tile_texture,
            font: font,
            width: game_width,
            height: game_height,
            tile_width_px: tileset.tile_width(),
            tile_height_px: tileset.tile_height(),
            game_width_px: width_px as usize,
            game_height_px: game_height_px,
            width_px: width_px as usize,
            height_px: height_px as usize,
            tileset: tileset,
            clear_colour: Color::RGB(0, 0, 0),
            game_rect: Rect::new(0, 0, game_width_px as u32, game_height_px as u32),
            screen_rect: Rect::new(0, 0, width_px, height_px),
            message_log_position: message_log_position,
            message_log_rect: message_log_rect,
            message_log: message_log,
            scroll_position: Coord::new(0, 0),
            display_log_num_lines: ((height_px as usize) / (MESSAGE_LOG_LINE_HEIGHT_PX + MESSAGE_LOG_PADDING_PX)) / scale,
            display_log_num_cols: ((width_px as usize - MESSAGE_LOG_PADDING_PX * 2) / MESSAGE_LOG_LINE_HEIGHT_PX) / scale, // square fonts only
            hud_position: hud_position,
            hud_texture: hud_texture,
            hud: hud,
            hud_rect: Rect::new(hud_position.x as i32, hud_position.y as i32, width_px, (HUD_TOTAL_HEIGHT_PX * scale) as u32),
            scale: scale,
        })
    }

    fn padded_text_line_height(&self) -> usize {
       (MESSAGE_LOG_LINE_HEIGHT_PX + MESSAGE_LOG_PADDING_PX) * self.scale
    }

    fn text_padding(&self) -> usize {
        MESSAGE_LOG_PADDING_PX * self.scale
    }

    fn tile_width(&self) -> usize {
        self.tileset.tile_width()
    }

    fn tile_height(&self) -> usize {
        self.tileset.tile_height()
    }

    fn screen_rect(&self, coord: Coord) -> Rect {
        let width = self.scale as i32 * self.tile_width() as i32;
        let height = self.scale as i32 * self.tile_height() as i32;

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
            moon: cell.moon,
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

    fn draw_internal(&mut self) {

        let blank = *self.tileset.resolve_extra(ExtraTileType::Blank);
        let moon = *self.tileset.resolve_extra(ExtraTileType::Moon);

        for (coord, cell) in izip!(self.buffer.coord_iter(), self.buffer.iter()) {
            let rect = self.screen_rect(coord);
            let info = self.to_sdl_info(cell);

            self.sdl_renderer.copy(&self.tile_texture, Some(blank), Some(rect)).expect(RENDERING_FAILED_MSG);

            let texture = if info.visible {
                &self.tile_texture
            } else {
                &self.greyscale_tile_texture
            };

            if let Some(bg_rect) = info.bg {
                self.sdl_renderer.copy(texture, Some(bg_rect), Some(rect)).expect(RENDERING_FAILED_MSG);
            }


            if let Some(fg_rect) = info.fg {
                self.sdl_renderer.copy(texture, Some(fg_rect), Some(rect)).expect(RENDERING_FAILED_MSG);
            }

            if let Some(health_overlay) = info.health_overlay {
                if !health_overlay.is_full() {
                    let red = Self::rgba32_to_sdl_colour(HEALTH_BAR_RED);
                    let green = Self::rgba32_to_sdl_colour(HEALTH_BAR_GREEN);

                    let health_bar_green_px = if health_overlay.umax() == 0 {
                        0
                    } else {
                        (self.scale * self.tile_width_px * health_overlay.ucurrent()) / health_overlay.umax()
                    };

                    let health_bar_green_rect = Rect::new(rect.x(),
                                                          rect.y() + (self.tile_height_px * self.scale - HEALTH_BAR_HEIGHT_PX * self.scale) as i32,
                                                          health_bar_green_px as u32,
                                                          (HEALTH_BAR_HEIGHT_PX * self.scale) as u32);
                    let health_bar_red_rect = Rect::new(rect.x() + health_bar_green_px as i32,
                                                        health_bar_green_rect.y(),
                                                        (self.tile_width_px * self.scale - health_bar_green_px) as u32,
                                                        (HEALTH_BAR_HEIGHT_PX * self.scale) as u32);

                    self.sdl_renderer.set_draw_color(red);
                    self.sdl_renderer.fill_rect(health_bar_red_rect).expect("Failed to draw health bar red rect");
                    self.sdl_renderer.set_draw_color(green);
                    self.sdl_renderer.fill_rect(health_bar_green_rect).expect("Failed to draw health bar green rect");
                }
            }

            if info.moon && info.visible {
                self.sdl_renderer.copy(&self.tile_texture, Some(moon), Some(rect)).expect(RENDERING_FAILED_MSG);
            }
        }
    }

    fn draw_overlay_internal(&mut self, overlay: &RenderOverlay) {
        let aim_line_bg = *self.tileset.resolve_extra(ExtraTileType::AimLine);
        if let Some(ref aim_line) = overlay.aim_line {
            for coord in aim_line.iter() {
                let screen_coord = self.world_to_screen(coord);
                if let Some(cell) = self.buffer.get(screen_coord) {
                    let rect = self.screen_rect(screen_coord);
                    let info = self.to_sdl_info(cell);

                    self.sdl_renderer.copy(&self.tile_texture, Some(aim_line_bg), Some(rect)).expect(RENDERING_FAILED_MSG);
                    if let Some(fg_rect) = info.fg {
                        let texture = if info.visible {
                            &self.tile_texture
                        } else {
                            &self.greyscale_tile_texture
                        };
                        self.sdl_renderer.copy(texture, Some(fg_rect), Some(rect)).expect(RENDERING_FAILED_MSG);
                    }
                }
            }
        } else if let Some(examine_cursor) = overlay.examine_cursor {
            let screen_coord = self.world_to_screen(examine_cursor);
            if let Some(cell) = self.buffer.get(screen_coord) {
                let rect = self.screen_rect(screen_coord);
                let info = self.to_sdl_info(cell);

                self.sdl_renderer.copy(&self.tile_texture, Some(aim_line_bg), Some(rect)).expect(RENDERING_FAILED_MSG);
                if let Some(fg_rect) = info.fg {
                    let texture = if info.visible {
                        &self.tile_texture
                    } else {
                        &self.greyscale_tile_texture
                    };
                    self.sdl_renderer.copy(texture, Some(fg_rect), Some(rect)).expect(RENDERING_FAILED_MSG);
                }
            }
        }
    }

    fn rgb24_to_sdl_colour(rgb24: Rgb24) -> Color {
        Color::RGB(rgb24.red, rgb24.green, rgb24.blue)
    }

    fn rgba32_to_sdl_colour(rgba32: Rgba32) -> Color {
        Color::RGBA(rgba32.red, rgba32.green, rgba32.blue, rgba32.alpha)
    }

    fn render_message_part(renderer: &mut Renderer, font: &Font, scale: usize, part: &MessagePart, cursor: Coord) -> Coord {
        match part.as_text() {
            Some(text_part) => Self::render_text_message_part(renderer, font, scale, MESSAGE_LOG_PLAIN_COLOUR, text_part, cursor),
            None => cursor,
        }
    }

    fn render_text_message_part(renderer: &mut Renderer, font: &Font, scale: usize, plain_colour: Rgb24, part: &TextMessagePart, mut cursor: Coord) -> Coord {
        let (colour, string) = match *part {
            TextMessagePart::Plain(ref s) => (plain_colour, s),
            TextMessagePart::Colour(c, ref s) => (c, s),
        };

        let sdl_colour = Self::rgb24_to_sdl_colour(colour);
        let surface = font.render(string.as_ref()).solid(sdl_colour).expect("Failed to create text surface");
        let texture = renderer.create_texture_from_surface(&surface).expect("Failed to create text texture");

        // assume fixed-width, square font
        let text_width = string.len() * MESSAGE_LOG_LINE_HEIGHT_PX * scale;
        let text_rect = Rect::new(cursor.x as i32, cursor.y as i32, text_width as u32,
                                  (MESSAGE_LOG_LINE_HEIGHT_PX * scale) as u32);
        renderer.copy(&texture, None, Some(text_rect)).expect("Failed to render text");
        cursor.x += text_width as isize;

        cursor
    }

    fn render_message(renderer: &mut Renderer, font: &Font, scale: usize, message: &Message, cursor: Coord) -> Coord {
        let mut tmp_cursor = cursor;
        for part in message {
            tmp_cursor = Self::render_message_part(renderer, font, scale, part, tmp_cursor);
        }
        tmp_cursor.x = cursor.x;
        tmp_cursor.y += (MESSAGE_LOG_LINE_HEIGHT_PX * scale) as isize;

        tmp_cursor
    }

    fn render_text_message(renderer: &mut Renderer, font: &Font, scale: usize, plain_colour: Rgb24, message: &TextMessage, cursor: Coord) -> Coord {
        let mut tmp_cursor = cursor;
        for part in message {
            tmp_cursor = Self::render_text_message_part(renderer, font, scale, plain_colour, part, tmp_cursor);
        }
        tmp_cursor.x = cursor.x;
        tmp_cursor.y += (MESSAGE_LOG_LINE_HEIGHT_PX * scale) as isize;

        tmp_cursor
    }

    fn draw_message_log_internal(&mut self) {
        self.clear_message_log();
        let mut cursor = self.message_log_position + Coord::new(MESSAGE_LOG_PADDING_PX as isize, MESSAGE_LOG_PADDING_PX as isize);

        for line in &self.message_log {
            cursor = Self::render_message(&mut self.sdl_renderer, &self.font, self.scale, line, cursor);
            cursor.y += MESSAGE_LOG_PADDING_PX as isize;
        }
    }

    fn scroll_bar_rect(&self, num_messages: usize, offset: usize, from_top: bool) -> Option<Rect> {
        let num_lines = self.fullscreen_log_num_rows();
        if num_messages > num_lines {
            let scroll_bar_height_px = (self.height_px * num_lines) / num_messages;
            let remaining_px = self.height_px - scroll_bar_height_px;
            let max_offset = num_messages - num_lines;
            let scroll_px = (offset * remaining_px) / max_offset;
            let scroll_bar_top_px = if from_top {
                scroll_px
            } else {
                remaining_px - scroll_px
            };
            let scroll_bar_left_px = self.width_px - SCROLL_BAR_WIDTH_PX * self.scale;
            Some(Rect::new(scroll_bar_left_px as i32, scroll_bar_top_px as i32,
                           (SCROLL_BAR_WIDTH_PX * self.scale) as u32, scroll_bar_height_px as u32))
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
            cursor = Self::render_text_message(&mut self.sdl_renderer, &self.font, self.scale, MESSAGE_LOG_PLAIN_COLOUR, line, cursor);
            cursor.y += MESSAGE_LOG_PADDING_PX as isize;
        }

        if let Some(scroll_bar) = self.scroll_bar_rect(wrapped.len(), offset, true) {
            self.sdl_renderer.set_draw_color(Self::rgb24_to_sdl_colour(SCROLL_BAR_COLOUR));
            self.sdl_renderer.fill_rect(scroll_bar).expect("Failed to draw scroll bar");
        }

        cursor
    }
}

impl<'a> KnowledgeRenderer for SdlKnowledgeRenderer<'a> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn world_offset(&self) -> Coord {
        self.scroll_position
    }

    fn update_game_window_buffer(&mut self, knowledge: &DrawableKnowledgeLevel, turn_id: u64, player_position: Coord) {
        self.scroll_position = self.centre_offset(player_position);
        self.buffer.update(knowledge, turn_id, self.scroll_position);
    }

    fn draw_game_window(&mut self) {
        self.clear_game();
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
        for (log_entry, message) in izip!(messages.tail(MESSAGE_LOG_NUM_LINES), &mut self.message_log) {
            message.clear();
            language.translate_repeated(log_entry.message, log_entry.repeated, message);
        }
    }

    fn fullscreen_log(&mut self, message_log: &MessageLog, offset: usize, language: &Box<Language>) {
        self.clear_screen();

        let mut cursor = Coord::new(MESSAGE_LOG_PADDING_PX as isize, MESSAGE_LOG_PADDING_PX as isize);
        let mut message = Message::new();

        let messages = message_log.tail_with_offset(self.fullscreen_log_num_rows(), offset);

        for log_entry in messages {
            message.clear();
            language.translate_repeated(log_entry.message, log_entry.repeated, &mut message);
            cursor = Self::render_message(&mut self.sdl_renderer, &self.font, self.scale, &message, cursor);
            cursor.y += MESSAGE_LOG_PADDING_PX as isize;
        }

        if let Some(scroll_bar_rect) = self.scroll_bar_rect(message_log.len(), offset, false) {
            self.sdl_renderer.set_draw_color(Self::rgb24_to_sdl_colour(SCROLL_BAR_COLOUR));
            self.sdl_renderer.fill_rect(scroll_bar_rect).expect("Failed to draw scroll bar");
        }
    }

    fn fullscreen_log_num_rows(&self) -> usize {
        self.display_log_num_lines
    }

    fn fullscreen_log_num_cols(&self) -> usize {
        self.display_log_num_cols - 1
    }

    fn fullscreen_wrapped_translated_message(&mut self, wrapped: &Vec<TextMessage>, offset: usize) {
        self.clear_screen();
        self.display_wrapped_message_fullscreen_internal(wrapped, offset);
    }

    fn draw_hud(&mut self, entity: EntityRef, _language: &Box<Language>) {
        self.clear_hud();
        let sdl_colour = Self::rgb24_to_sdl_colour(HUD_TEXT_COLOUR);
        let mut cursor = HUD_TOP_PADDING_PX;

        let health_rect = Rect::new((self.hud_position.x + cursor as isize) as i32,
                                    (self.hud_position.y + HUD_TOP_PADDING_PX as isize) as i32,
                                    (HUD_HEIGHT_PX * self.scale) as u32,
                                    (HUD_HEIGHT_PX * self.scale) as u32);

        self.sdl_renderer.copy(&self.hud_texture, Some(self.hud.health), Some(health_rect)).expect("Failed to render symbol");

        cursor += (HUD_HEIGHT_PX + HUD_TOP_PADDING_PX) * self.scale;

        let hit_points = entity.hit_points().expect("Entity missing hit_points");

        let health_text = format!("{}/{}", hit_points.current(), hit_points.max());
        let surface = self.font.render(health_text.as_ref()).solid(sdl_colour).expect("Failed to create text surface");
        let texture = self.sdl_renderer.create_texture_from_surface(&surface).expect("Failed to create text texture");

        let text_width = health_text.len() * HUD_HEIGHT_PX * self.scale;
        let text_rect = Rect::new((self.hud_position.x + cursor as isize) as i32,
                                  (self.hud_position.y + HUD_TOP_PADDING_PX as isize) as i32,
                                  text_width as u32,
                                  (HUD_HEIGHT_PX  * self.scale) as u32);

        self.sdl_renderer.copy(&texture, None, Some(text_rect)).expect("Failed to render text");
    }

    fn fullscreen_menu<T>(&mut self, prelude: Option<MessageType>, menu: &Menu<T>, state: &MenuState, language: &Box<Language>) {

        let mut message = Message::new();
        let mut wrapped = Vec::new();

        self.clear_screen();

        let mut cursor = if let Some(message_type) = prelude {
            language.translate(message_type, &mut message);

            self.fullscreen_wrap(&message, &mut wrapped);

            let mut cursor = self.display_wrapped_message_fullscreen_internal(&wrapped, 0);

            cursor.y += self.padded_text_line_height() as isize;

            cursor
        } else {
            self.fullscreen_initial_cursor()
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

            cursor = Self::render_text_message(&mut self.sdl_renderer, &self.font, self.scale, colour, &wrapped[0], cursor);
            cursor.y += self.text_padding() as isize;
        }
    }

    fn publish(&mut self) {
        self.sdl_renderer.present();
    }

    fn log_num_lines(&self) -> usize {
        MESSAGE_LOG_NUM_LINES
    }
}
