use std::path;
use std::result;
use std::cmp;
use std::slice;
use std::mem;

use sdl2::VideoSubsystem;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};
use sdl2::image::{LoadTexture, LoadSurface};
use sdl2::pixels::Color;
use sdl2::ttf::Font;
use sdl2::surface::Surface;

use game::*;
use game::frontends::sdl::{Tileset, ExtraTileType};

use coord::Coord;
use colour::Rgb24;

const RENDERING_FAILED_MSG: &'static str = "Rendering failed";
const MESSAGE_LOG_NUM_LINES: usize = 4;
const MESSAGE_LOG_LINE_HEIGHT_PX: usize = 16;
const MESSAGE_LOG_HEIGHT_PX: usize = MESSAGE_LOG_LINE_HEIGHT_PX * MESSAGE_LOG_NUM_LINES;
const MESSAGE_LOG_PLAIN_COLOUR: Rgb24 = Rgb24 { red: 255, green: 255, blue: 255 };
const MESSAGE_LOG_PADDING_TOP_PX: usize = 16;

const SCROLL_BAR_COLOUR: Rgb24 = Rgb24 { red: 255, green: 255, blue: 255 };
const SCROLL_BAR_WIDTH_PX: usize = 16;

struct SdlCellInfo {
    fg: Option<Rect>,
    bg: Option<Rect>,
    moon: bool,
    visible: bool,
}

pub struct SdlKnowledgeRenderer<'a> {
    buffer: TileBuffer,
    sdl_renderer: Renderer<'static>,
    tile_texture: Texture,
    greyscale_tile_texture: Texture,
    font: Font<'a>,
    width: usize,
    height: usize,
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
    scroll: bool,
    scroll_position: Coord,
}

#[derive(Debug)]
pub enum SdlKnowledgeRendererError {
    WindowCreationFailure,
    RendererInitialisationFailure,
    TileLoadFailure,
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
               font: Font<'a>,
               scroll: bool) -> result::Result<Self, SdlKnowledgeRendererError> {

        let game_width_px = (game_width * tileset.tile_width()) as usize;
        let game_height_px = (game_height * tileset.tile_height()) as usize;
        let width_px = game_width_px as u32;
        let height_px = (game_height_px + MESSAGE_LOG_HEIGHT_PX + MESSAGE_LOG_PADDING_TOP_PX) as u32;
        let window = video.window(title, width_px, height_px)
            .build()
            .map_err(|_| SdlKnowledgeRendererError::WindowCreationFailure)?;

        let renderer = window.renderer()
            .build()
            .map_err(|_| SdlKnowledgeRendererError::RendererInitialisationFailure)?;

        let tile_texture = renderer.load_texture(&tile_path).map_err(|_| SdlKnowledgeRendererError::TileLoadFailure)?;
        let greyscale_tile_texture = Self::create_greyscale_tile_texture(&renderer, &tile_path).unwrap();

        let mut message_log = Vec::new();
        for _ in 0..MESSAGE_LOG_NUM_LINES {
            message_log.push(Message::new());
        }

        let message_log_position = Coord::new(0, (game_height_px + MESSAGE_LOG_PADDING_TOP_PX) as isize);
        let message_log_rect = Rect::new(message_log_position.x as i32,
                                         message_log_position.y as i32,
                                         width_px,
                                         MESSAGE_LOG_HEIGHT_PX as u32);


        Ok(SdlKnowledgeRenderer {
            buffer: TileBuffer::new(game_width, game_height),
            sdl_renderer: renderer,
            tile_texture: tile_texture,
            greyscale_tile_texture: greyscale_tile_texture,
            font: font,
            width: game_width,
            height: game_height,
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
            scroll: scroll,
            scroll_position: Coord::new(0, 0),
            display_log_num_lines: (height_px as usize) / MESSAGE_LOG_LINE_HEIGHT_PX,
        })
    }

    fn tile_width(&self) -> usize {
        self.tileset.tile_width()
    }

    fn tile_height(&self) -> usize {
        self.tileset.tile_height()
    }

    fn screen_rect(&self, coord: Coord) -> Rect {
        let width = self.tile_width() as i32;
        let height = self.tile_height() as i32;

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
        self.sdl_renderer.fill_rect(self.game_rect).expect("Failed to clear screen");
    }

    fn clear_message_log(&mut self) {
        self.sdl_renderer.set_draw_color(self.clear_colour);
        self.sdl_renderer.fill_rect(self.message_log_rect).expect("Failed to clear message_log");
    }

    fn clear_screen(&mut self) {
        self.sdl_renderer.set_draw_color(self.clear_colour);
        self.sdl_renderer.fill_rect(self.screen_rect).expect("Failed to clear message_log");
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

            if info.moon && info.visible{
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

    fn render_message_part(renderer: &mut Renderer, font: &Font, part: &MessagePart, mut cursor: Coord) -> Coord {

        let text_part = match part.as_text() {
            Some(text_part) => text_part,
            None => return cursor,
        };

        let (colour, string) = match *text_part {
            TextMessagePart::Plain(ref s) => (MESSAGE_LOG_PLAIN_COLOUR, s),
            TextMessagePart::Colour(c, ref s) => (c, s),
        };

        let sdl_colour = Self::rgb24_to_sdl_colour(colour);
        let surface = font.render(string.as_ref()).solid(sdl_colour).expect("Failed to create text surface");
        let texture = renderer.create_texture_from_surface(&surface).expect("Failed to create text texture");

        // assume fixed-width, square font
        let text_width = string.len() * MESSAGE_LOG_LINE_HEIGHT_PX;
        let text_rect = Rect::new(cursor.x as i32, cursor.y as i32, text_width as u32,
                                  MESSAGE_LOG_LINE_HEIGHT_PX as u32);
        renderer.copy(&texture, None, Some(text_rect)).expect("Failed to render text");
        cursor.x += text_width as isize;

        cursor
    }

    fn render_message(renderer: &mut Renderer, font: &Font, reset_x: isize, message: &Message, mut cursor: Coord) -> Coord {
        for part in message {
            cursor = Self::render_message_part(renderer, font, part, cursor);
        }
        cursor.x = reset_x;
        cursor.y += MESSAGE_LOG_LINE_HEIGHT_PX as isize;

        cursor
    }

    fn draw_message_log_internal(&mut self) {

        self.clear_message_log();
        let mut cursor = self.message_log_position;

        for line in &self.message_log {
            cursor = Self::render_message(&mut self.sdl_renderer, &self.font, self.message_log_position.x, line, cursor);
        }
    }

    fn scroll_bar_rect(&self, num_messages: usize, offset: usize) -> Option<Rect> {
        let num_lines = self.display_log_num_lines();
        if num_messages > num_lines {
            let scroll_bar_height_px = (self.height_px * num_lines) / num_messages;
            let remaining_px = self.height_px - scroll_bar_height_px;
            let max_offset = num_messages - num_lines;
            let scroll_bar_top_px = remaining_px - ((offset * remaining_px) / max_offset);
            let scroll_bar_left_px = self.width_px - SCROLL_BAR_WIDTH_PX;
            Some(Rect::new(scroll_bar_left_px as i32, scroll_bar_top_px as i32,
                           SCROLL_BAR_WIDTH_PX as u32, scroll_bar_height_px as u32))
        } else {
            None
        }
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

    fn update(&mut self, knowledge: &DrawableKnowledgeLevel, turn_id: u64, position: Coord) {
        let scroll_position = if self.scroll {
            self.centre_offset(position)
        } else {
            Coord::new(0, 0)
        };

        self.scroll_position = self.buffer.update(knowledge, turn_id, scroll_position);
    }

    fn draw(&mut self) {
        self.clear_game();
        self.draw_internal();
        self.draw_message_log_internal();
        self.sdl_renderer.present();
    }

    fn draw_with_overlay(&mut self, overlay: &RenderOverlay) {
        self.clear_game();
        self.draw_internal();
        self.draw_overlay_internal(overlay);
        self.draw_message_log_internal();
        self.sdl_renderer.present();
    }

    fn update_log(&mut self, messages: &MessageLog, language: &Box<Language>) {
        for (log_entry, message) in izip!(messages.tail(MESSAGE_LOG_NUM_LINES), &mut self.message_log) {
            language.translate_repeated(log_entry.message, log_entry.repeated, message);
        }
    }

    fn display_log(&mut self, message_log: &MessageLog, offset: usize, language: &Box<Language>) {
        self.clear_screen();

        let scroll_bar_rect = self.scroll_bar_rect(message_log.len(), offset);

        let mut cursor = Coord::new(0, 0);
        let mut message = Message::new();

        let messages = message_log.tail_with_offset(self.display_log_num_lines(), offset);

        for log_entry in messages {
            language.translate_repeated(log_entry.message, log_entry.repeated, &mut message);
            cursor = Self::render_message(&mut self.sdl_renderer, &self.font, self.message_log_position.x, &message, cursor);
        }

        if let Some(scroll_bar_rect) = scroll_bar_rect {
            self.sdl_renderer.set_draw_color(Self::rgb24_to_sdl_colour(SCROLL_BAR_COLOUR));
            self.sdl_renderer.fill_rect(scroll_bar_rect).expect("Failed to draw scroll bar");
        }

        self.sdl_renderer.present();
    }

    fn display_log_num_lines(&self) -> usize {
        self.display_log_num_lines
    }
}
