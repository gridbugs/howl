use std::collections::HashMap;
use std::result;

use tile::*;
use sdl2::rect::Rect;
use toml;

#[derive(Debug)]
pub enum TilesetError {
    TileNotFound(String),
    MissingField(String),
    IncorrectType(String),
}

pub type TilesetResult<T> = result::Result<T, TilesetError>;

#[derive(Debug)]
pub struct ExtraTiles {
    pub blank: Rect,
    pub death: Rect,
}

#[derive(Debug)]
pub struct SimpleTile {
    channels: [Option<Rect>; NUM_TILE_CHANNELS],
}

#[derive(Debug)]
pub enum ComplexTile {
    Simple(SimpleTile),
    Wall {
        front: SimpleTile,
        back: SimpleTile,
    },
}

impl SimpleTile {
    pub fn rect(&self, channel: usize) -> Option<Rect> {
        self.channels.get(channel).and_then(|r| *r)
    }
}

impl ComplexTile {
    fn from_toml(table: &toml::value::Table, width: i32, height: i32) -> TilesetResult<Self> {
        unimplemented!()
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
        let tile_width = table.get("tile_width").ok_or(TilesetError::MissingField("tile_width".to_string()))?
            .as_integer().ok_or(TilesetError::IncorrectType("tile_width".to_string()))? as i32;

        let tile_height = table.get("tile_height").ok_or(TilesetError::MissingField("tile_height".to_string()))?
            .as_integer().ok_or(TilesetError::IncorrectType("tile_width".to_string()))? as i32;

        let tile_table = table.get("tiles").ok_or(TilesetError::MissingField("tiles".to_string()))?
            .as_table().ok_or(TilesetError::IncorrectType("tiles".to_string()))?;

        let extra_table = table.get("extra").ok_or(TilesetError::MissingField("extra".to_string()))?
            .as_table().ok_or(TilesetError::IncorrectType("extra".to_string()))?;

        let mut tile_map = HashMap::new();

        for (key, tile_desc_toml) in tile_table.iter() {
            let tile_type = TileType::from_str(key).ok_or(TilesetError::TileNotFound(key.to_string()))?;
            let tile_desc = tile_desc_toml.as_table().ok_or(TilesetError::IncorrectType(key.to_string()))?;
            let tile = ComplexTile::from_toml(tile_desc, tile_width, tile_height)?;
            tile_map.insert(tile_type, tile);
        }

        let extra = ExtraTiles {
            blank: extra_rect(&extra_table, "Blank",  tile_width, tile_height)?,
            death: extra_rect(&extra_table, "Death",  tile_width, tile_height)?,
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

fn extra_rect(table: &toml::value::Table, name: &str, width: i32, height: i32) -> TilesetResult<Rect> {
    let sub_table = table.get(name).ok_or(TilesetError::TileNotFound(name.to_string()))?
        .as_array().ok_or(TilesetError::IncorrectType(name.to_string()))?;
    unimplemented!()
}
