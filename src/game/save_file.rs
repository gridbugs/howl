use std::path;
use std::fs;
use std::io::{Read, Write};
use bincode::{self, SizeLimit};
use game::*;

const SAVE_FILE: &'static str = "save";

pub fn save(user_path: &path::Path, game_state: GameState) -> GameState {
    let serializable = SerializableGameState::from(game_state);

    let encoded = bincode::serialize(&serializable, SizeLimit::Infinite).expect("Failed to serialize game state");

    fs::File::create(user_path.join(SAVE_FILE))
        .and_then(|mut f| f.write_all(&encoded))
        .expect("Failed to save game");

    GameState::from(serializable)
}

pub fn load(user_path: &path::Path) -> Option<GameState> {
    if let Ok(mut f) = fs::File::open(user_path.join(SAVE_FILE)) {
        let mut encoded = Vec::new();
        f.read_to_end(&mut encoded).expect("Failed to read save file");
        let serializable: SerializableGameState = bincode::deserialize(&encoded).expect("Failed to parse save file");
        Some(GameState::from(serializable))
    } else {
        None
    }
}

pub fn delete(user_path: &path::Path) -> bool {
    fs::remove_file(user_path.join(SAVE_FILE)).is_ok()
}
