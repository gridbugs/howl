use std::collections::HashMap;
use std::result;

use sdl2::rect::Rect;
use toml;

use game::*;

#[derive(Debug)]
pub enum TilesetError {
    TileNotFound,
    InvalidSpec,
}

pub type TilesetResult<T> = result::Result<T, TilesetError>;

#[derive(Debug)]
pub struct ExtraTiles {
    pub blank: Rect,
    pub death: Rect,
}

#[derive(Debug, Clone, Copy)]
pub enum SimpleTile {
    Background(Rect),
    Foreground(Rect),
    Full {
        background: Rect,
        foreground: Rect,
    },
}

impl SimpleTile {
    pub fn foreground(&self) -> Option<Rect> {
        match self {
            &SimpleTile::Foreground(r) => Some(r),
            &SimpleTile::Full { foreground, .. } => Some(foreground),
            _ => None,
        }
    }
    pub fn background(&self) -> Option<Rect> {
        match self {
            &SimpleTile::Background(r) => Some(r),
            &SimpleTile::Full { background, .. } => Some(background),
            _ => None,
        }
    }
}

fn new_rect(x: i32, y: i32, width: i32, height: i32, padding: i32) -> Rect {
    Rect::new(x * width + padding, y * height + padding, (width - padding * 2) as u32, (height - padding * 2) as u32)
}

fn rect_from_toml_value(value: &toml::Value, width: i32, height: i32, padding: i32) -> Option<Rect> {
    value.as_table().and_then(|table| rect_from_toml(table, width, height, padding))
}

fn rect_from_toml(table: &toml::value::Table, width: i32, height: i32, padding: i32) -> Option<Rect> {
    let x = match table.get("x").and_then(|x| x.as_integer()) {
        Some(x) => x as i32,
        None => return None,
    };
    let y = match table.get("y").and_then(|y| y.as_integer()) {
        Some(y) => y as i32,
        None => return None,
    };

    Some(new_rect(x, y, width, height, padding))
}

fn extra_rect(table: &toml::value::Table, name: &str, width: i32, height: i32, padding: i32) -> TilesetResult<Rect> {
    let sub_table = table.get(name).ok_or(TilesetError::TileNotFound)?
        .as_table().ok_or(TilesetError::InvalidSpec)?;
    let x = sub_table.get("x").ok_or(TilesetError::InvalidSpec)?.
        as_integer().ok_or(TilesetError::InvalidSpec)? as i32;
    let y = sub_table.get("y").ok_or(TilesetError::InvalidSpec)?.
        as_integer().ok_or(TilesetError::InvalidSpec)? as i32;

    Ok(new_rect(x, y, width, height, padding))
}

impl SimpleTile {

    fn from_toml_value(value: &toml::Value, width: i32, height: i32, padding: i32) -> Option<Self> {
        value.as_table().and_then(|table| Self::from_toml(table, width, height, padding))
    }

    fn from_toml(table: &toml::value::Table, width: i32, height: i32, padding: i32) -> Option<Self> {
        if table.contains_key("foreground") && table.contains_key("background") {
            let fg = match table.get("foreground").and_then(|fg| rect_from_toml_value(fg, width, height, padding)) {
                Some(fg) => fg,
                None => return None,
            };
            let bg = match table.get("background").and_then(|bg| rect_from_toml_value(bg, width, height, padding)) {
                Some(bg) => bg,
                None => return None,
            };

            Some(SimpleTile::Full {
                foreground: fg,
                background: bg,
            })
        } else if table.contains_key("foreground") {
            let fg = match table.get("foreground").and_then(|fg| rect_from_toml_value(fg, width, height, padding)) {
                Some(fg) => fg,
                None => return None,
            };

            Some(SimpleTile::Foreground(fg))
        } else if table.contains_key("background") {
            let bg = match table.get("background").and_then(|bg| rect_from_toml_value(bg, width, height, padding)) {
                Some(bg) => bg,
                None => return None,
            };

            Some(SimpleTile::Background(bg))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ComplexTile {
    Simple(SimpleTile),
    Wall {
        front: SimpleTile,
        back: SimpleTile,
    },
}

impl ComplexTile {
    fn from_toml(table: &toml::value::Table, width: i32, height: i32, padding: i32) -> Option<Self> {
        if table.contains_key("front") && table.contains_key("back") {
            let maybe_front = table.get("front").and_then(|front_table| {
                SimpleTile::from_toml_value(front_table, width, height, padding)
            });
            let front = if let Some(front) = maybe_front {
                front
            } else {
                return None;
            };

            let maybe_back = table.get("back").and_then(|back_table| {
                SimpleTile::from_toml_value(back_table, width, height, padding)
            });
            let back = if let Some(back) = maybe_back {
                back
            } else {
                return None;
            };

            Some(ComplexTile::Wall {
                front: front,
                back: back,
            })
        } else {
            SimpleTile::from_toml(table, width, height, padding).map(ComplexTile::Simple)
        }
    }
}

#[derive(Debug)]
pub struct Tileset {
    pub extra: ExtraTiles,
    tiles: HashMap<TileType, ComplexTile>,
    tile_width: usize,
    tile_height: usize,
}

impl Tileset {
    pub fn new(table: toml::value::Table) -> TilesetResult<Self> {

        let tile_width = table.get("tile_width").ok_or(TilesetError::InvalidSpec)?
            .as_integer().ok_or(TilesetError::InvalidSpec)? as i32;

        let tile_height = table.get("tile_height").ok_or(TilesetError::InvalidSpec)?
            .as_integer().ok_or(TilesetError::InvalidSpec)? as i32;

        let tile_padding = table.get("tile_padding").ok_or(TilesetError::InvalidSpec)?
            .as_integer().ok_or(TilesetError::InvalidSpec)? as i32;

        let tile_table = table.get("tiles").ok_or(TilesetError::InvalidSpec)?
            .as_table().ok_or(TilesetError::InvalidSpec)?;

        let mut tile_map = HashMap::new();

        for (key, tile_desc_toml) in tile_table.iter() {
            let tile_type = TileType::from_str(key).ok_or(TilesetError::TileNotFound)?;
            let tile_desc = tile_desc_toml.as_table().ok_or(TilesetError::InvalidSpec)?;
            let tile = ComplexTile::from_toml(tile_desc, tile_width, tile_height, tile_padding).ok_or(TilesetError::InvalidSpec)?;
            tile_map.insert(tile_type, tile);
        }

        let extra_table = table.get("extra").ok_or(TilesetError::InvalidSpec)?
            .as_table().ok_or(TilesetError::InvalidSpec)?;

        let extra = ExtraTiles {
            blank: extra_rect(&extra_table, "Blank",  tile_width, tile_height, tile_padding)?,
            death: extra_rect(&extra_table, "Death",  tile_width, tile_height, tile_padding)?,
        };

        Ok(Tileset {
            tiles: tile_map,
            extra: extra,
            tile_width: tile_width as usize,
            tile_height: tile_height as usize,
        })
    }

    pub fn tile_width(&self) -> usize {
        self.tile_width
    }

    pub fn tile_height(&self) -> usize {
        self.tile_height
    }

    pub fn resolve(&self, tile_type: TileType) -> &ComplexTile {
        self.tiles.get(&tile_type).expect(format!("Couldn't find tile for {:?}", tile_type).as_ref())
    }
}
