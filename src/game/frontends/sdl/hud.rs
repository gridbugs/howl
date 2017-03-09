use std::result;
use toml;
use sdl2::rect::Rect;

#[derive(Debug)]
pub enum HudError {
    SymbolNotFound,
    InvalidSpec,
}

pub type HudResult<T> = result::Result<T, HudError>;


pub struct Hud {
    pub health: Rect,
    pub speed: Rect,
    pub engine: Rect,
    pub tyres: Rect,
    pub armour: Rect,
    pub letter: Rect,
    pub money: Rect,
}

impl Hud {

    fn get_rect(symbol_table: &toml::value::Table, name: &str, width: i32, height: i32) -> HudResult<Rect> {
        let symbol = symbol_table.get(name).ok_or(HudError::SymbolNotFound)?
            .as_table().ok_or(HudError::InvalidSpec)?;

        let x = symbol.get("x").ok_or(HudError::InvalidSpec)?.
            as_integer().ok_or(HudError::InvalidSpec)? as i32;
        let y = symbol.get("y").ok_or(HudError::InvalidSpec)?.
            as_integer().ok_or(HudError::InvalidSpec)? as i32;

        Ok(Rect::new(x * width, y * height, width as u32, height as u32))
    }

    pub fn new(table: toml::value::Table) -> HudResult<Self> {
        let symbol_width = table.get("symbol_width").ok_or(HudError::InvalidSpec)?
            .as_integer().ok_or(HudError::InvalidSpec)? as i32;
        let symbol_height = table.get("symbol_height").ok_or(HudError::InvalidSpec)?
            .as_integer().ok_or(HudError::InvalidSpec)? as i32;

        let symbol_table = table.get("symbols").ok_or(HudError::InvalidSpec)?
            .as_table().ok_or(HudError::InvalidSpec)?;

        Ok(Hud {
            health: Self::get_rect(symbol_table, "Health", symbol_width, symbol_height)?,
            speed: Self::get_rect(symbol_table, "Speed", symbol_width, symbol_height)?,
            engine: Self::get_rect(symbol_table, "Engine", symbol_width, symbol_height)?,
            tyres: Self::get_rect(symbol_table, "Tyres", symbol_width, symbol_height)?,
            armour: Self::get_rect(symbol_table, "Armour", symbol_width, symbol_height)?,
            letter: Self::get_rect(symbol_table, "Letter", symbol_width, symbol_height)?,
            money: Self::get_rect(symbol_table, "Money", symbol_width, symbol_height)?,
        })
    }
}
