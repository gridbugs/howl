use std::path;
use std::fs;
use std::io::Read;
use toml;

const CONFIG_FILE: &'static str = "config.toml";

#[derive(Debug)]
pub struct GameConfig {
    pub graphics: GraphicsConfig,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            graphics: GraphicsConfig::default(),
        }
    }
}

#[derive(Debug)]
pub struct GraphicsConfig {
    pub scale: usize,
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        GraphicsConfig {
            scale: 1,
        }
    }
}

impl GraphicsConfig {
    pub fn from_toml(table: &toml::Table) -> Self {
        let mut scale = table.get("scale").and_then(|s| s.as_integer()).unwrap_or(1);

        if scale < 1 {
            scale = 1;
        }

        GraphicsConfig {
            scale: scale as usize,
        }
    }
}

impl GameConfig {
    pub fn from_toml(table: toml::Table) -> Self {
        let graphics = table.get("graphics")
            .and_then(|g| g.as_table())
            .map_or_else(Default::default, GraphicsConfig::from_toml);

        GameConfig {
            graphics: graphics,
        }
    }

    pub fn from_file(path: &path::Path) -> Self {
        let mut toml_str = String::new();
        fs::File::open(path).ok().and_then(|mut file| {
            match file.read_to_string(&mut toml_str) {
                Ok(_) => Some(toml::Parser::new(toml_str.as_ref())),
                Err(_) => None,
            }
        }).and_then(|mut parser| {
            parser.parse()
        }).map(Self::from_toml).unwrap_or_default()
    }

    pub fn from_user_dir(path: &path::Path) -> Self {
        Self::from_file(path.join(CONFIG_FILE).as_path())
    }
}
