use std::path;

use sdl2::{self, Sdl};
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};
use sdl2::image::{LoadTexture, INIT_PNG};
use sdl2::pixels::Color;

use game::*;
use game::frontends::sdl::{Tileset, ExtraTileType};

use coord::Coord;

const CELL_WIDTH: usize = 7;
const CELL_HEIGHT: usize = 7;

struct SdlCellInfo {
    fg: Option<Rect>,
    bg: Option<Rect>,
    moon: bool,
    visible: bool,
}

pub struct SdlKnowledgeRenderer {
    buffer: TileBuffer,
    sdl: Sdl,
    sdl_renderer: Renderer<'static>,
    tile_texture: Texture,
    width: usize,
    height: usize,
    tileset: Tileset,
}

impl SdlKnowledgeRenderer {
    pub fn new(sdl: Sdl, width: usize, height: usize, tile_path: path::PathBuf, tileset: Tileset) -> Self {

        let width_px = (width * tileset.tile_width()) as u32;
        let height_px = (height * tileset.tile_height()) as u32;
        let window = sdl.video()
            .expect("Failed to connect to video subsystem")
            .window("Howl", width_px, height_px)
            .build()
            .expect("Failed to create window");

        let mut renderer = window.renderer()
            .build()
            .expect("Failed to initialise renderer");

        renderer.set_draw_color(Color::RGB(0, 0, 0));

        sdl2::image::init(INIT_PNG).expect("Failed to initialise image subsystem");

        let tile_texture = renderer.load_texture(&tile_path).expect("Failed to load tiles from file");

        SdlKnowledgeRenderer {
            buffer: TileBuffer::new(width, height),
            sdl: sdl,
            sdl_renderer: renderer,
            tile_texture: tile_texture,
            width: width,
            height: height,
            tileset: tileset,
        }
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
        self.sdl_renderer.clear();
    }

    fn draw_internal(&mut self) {

        let blank = *self.tileset.resolve_extra(ExtraTileType::Blank);
        let moon = *self.tileset.resolve_extra(ExtraTileType::Moon);

        for (coord, cell) in izip!(self.buffer.coord_iter(), self.buffer.iter()) {
            let rect = self.screen_rect(coord);
            let info = self.to_sdl_info(cell);

            if !info.visible {
                self.sdl_renderer.copy(&self.tile_texture, Some(blank), Some(rect)).expect("Rendering failed");
                continue;
            }
            if let Some(bg_rect) = info.bg {
                self.sdl_renderer.copy(&self.tile_texture, Some(bg_rect), Some(rect)).expect("Rendering failed");
            }
            if let Some(fg_rect) = info.fg {
                self.sdl_renderer.copy(&self.tile_texture, Some(fg_rect), Some(rect)).expect("Rendering failed");
            }
            if info.moon {
                self.sdl_renderer.copy(&self.tile_texture, Some(moon), Some(rect)).expect("Rendering failed");
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

                    self.sdl_renderer.copy(&self.tile_texture, Some(aim_line_bg), Some(rect)).expect("Rendering failed");
                    if let Some(fg_rect) = info.fg {
                        self.sdl_renderer.copy(&self.tile_texture, Some(fg_rect), Some(rect)).expect("Rendering failed");
                    }
                }
            }
        }
    }
}

impl KnowledgeRenderer for SdlKnowledgeRenderer {
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
        self.sdl_renderer.present();
    }

    fn draw_with_overlay(&mut self, overlay: &RenderOverlay) {
        self.clear_internal();
        self.draw_internal();
        self.draw_overlay_internal(overlay);
        self.sdl_renderer.present();
    }
}
