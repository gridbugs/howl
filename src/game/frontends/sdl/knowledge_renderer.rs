use std::path;
use std::result;
use std::cmp;

use sdl2::VideoSubsystem;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};
use sdl2::image::{LoadTexture, LoadSurface};
use sdl2::pixels::{Color, Palette, PixelFormatEnum};
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
    tileset: Tileset,
    message_log_position: Coord,
    clear_colour: Color,
    game_rect: Rect,
    message_log: Vec<Message>,
}

#[derive(Debug)]
pub enum SdlKnowledgeRendererError {
    WindowCreationFailure,
    RendererInitialisationFailure,
    TileLoadFailure,
}

impl<'a> SdlKnowledgeRenderer<'a> {

    fn index_to_rgb(index: u8) -> (u8, u8, u8) {
        let r = index >> 5;
        let g = (index >> 2) & 7;
        let b_small = index & 3;
        let b = if b_small == 0 {
            0
        } else {
            (b_small * 2) + 1
        };

        (r, g, b)
    }

    fn create_greyscale_tile_texture(renderer: &Renderer, tile_path: &path::PathBuf) -> result::Result<Texture, String> {
        let mut rgb332_colours = Vec::new();
        for i in 0u32..256 {

            let (r, g, b) = Self::index_to_rgb(i as u8);

            rgb332_colours.push(Color::RGB(r as u8 * 36, g as u8 * 36, b as u8 * 36));
        }
        let rgb332_palette = Palette::with_colors(&rgb332_colours)?;

        let mut greyscale_colours = Vec::new();
        for i in 0i32..256 {

            let (r, g, b) = Self::index_to_rgb(i as u8);

            let max = cmp::max(r, cmp::max(g, b));

            let normalised = if max == 0 {
                0
            } else {
                32 + 6 * max
            };

            let r = normalised;
            let g = normalised;
            let b = normalised;

            greyscale_colours.push(Color::RGB(r as u8, g as u8, b as u8));
        }
        let greyscale_palette = Palette::with_colors(&greyscale_colours)?;

        let mut dummy_surface = Surface::new(1, 1, PixelFormatEnum::Index8)?;
        dummy_surface.set_palette(&rgb332_palette)?;

        let pixel_format = dummy_surface.pixel_format();

        let tile_surface = Surface::from_file(&tile_path)?;
        let mut greyscale_tile_surface = tile_surface.convert(&pixel_format)?;
        greyscale_tile_surface.set_palette(&greyscale_palette)?;

        renderer.create_texture_from_surface(greyscale_tile_surface).map_err(|e| format!("{}", e))
    }

    pub fn new(video: &VideoSubsystem,
               title: &str,
               game_width: usize,
               game_height: usize,
               tile_path: path::PathBuf,
               tileset: Tileset,
               font: Font<'a>) -> result::Result<Self, SdlKnowledgeRendererError> {

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
            tileset: tileset,
            clear_colour: Color::RGB(0, 0, 0),
            game_rect: Rect::new(0, 0, game_width_px as u32, game_height_px as u32),
            message_log: message_log,
            message_log_position: Coord::new(0, (game_height_px + MESSAGE_LOG_PADDING_TOP_PX) as isize),
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

    fn clear_internal(&mut self) {
        self.sdl_renderer.set_draw_color(self.clear_colour);
        self.sdl_renderer.draw_rect(self.game_rect).expect("Failed to clear screen");
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
                if let Some(cell) = self.buffer.get(coord) {
                    let rect = self.screen_rect(coord);
                    let info = self.to_sdl_info(cell);

                    self.sdl_renderer.copy(&self.tile_texture, Some(aim_line_bg), Some(rect)).expect(RENDERING_FAILED_MSG);
                    if let Some(fg_rect) = info.fg {
                        self.sdl_renderer.copy(&self.tile_texture, Some(fg_rect), Some(rect)).expect(RENDERING_FAILED_MSG);
                    }
                }
            }
        }
    }

    fn draw_message_log_internal(&mut self) {

        let mut cursor = self.message_log_position;

        for line in &self.message_log {
            for part in line {
                let (colour, string) = match part {
                    &MessagePart::Plain(ref s) => (MESSAGE_LOG_PLAIN_COLOUR, s),
                    &MessagePart::Colour(c, ref s) => (c, s),
                };

                let sdl_colour = rgb24_to_sdl_colour(colour);
                let surface = self.font.render(string.as_ref()).solid(sdl_colour).expect("Failed to create text surface");
                let texture = self.sdl_renderer.create_texture_from_surface(&surface).expect("Failed to create text texture");

                // assume fixed-width, square font
                let text_width = string.len() * MESSAGE_LOG_LINE_HEIGHT_PX;
                let text_rect = Rect::new(cursor.x as i32, cursor.y as i32, text_width as u32,
                                          MESSAGE_LOG_LINE_HEIGHT_PX as u32);
                self.sdl_renderer.copy(&texture, None, Some(text_rect)).expect("Failed to render text");
                cursor.x += text_width as isize;
            }
            cursor.x = self.message_log_position.x;
            cursor.y += MESSAGE_LOG_LINE_HEIGHT_PX as isize;
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
        Coord::new(0, 0)
    }

    fn update(&mut self, knowledge: &DrawableKnowledgeLevel, turn_id: u64, _position: Coord) {
        // TODO handle scrolling
        self.buffer.update(knowledge, turn_id, None);
    }

    fn draw(&mut self) {
        self.clear_internal();
        self.draw_internal();
        self.draw_message_log_internal();
        self.sdl_renderer.present();
    }

    fn draw_with_overlay(&mut self, overlay: &RenderOverlay) {
        self.clear_internal();
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
}

fn rgb24_to_sdl_colour(rgb24: Rgb24) -> Color {
    Color::RGB(rgb24.red, rgb24.green, rgb24.blue)
}
