use std::path::Path;
use std::fs::{self, File};
use std::io::{Read, Write};
use bincode::{self, Infinite};
use game::*;

const SAVE_FILE: &'static str = "save";

pub fn save<P: AsRef<Path>>(user_path: P, game_state: GameState) -> GameState {
    let serializable = SerializableGameState::from(game_state);

    let encoded: Vec<u8> = bincode::serialize(&serializable, Infinite).expect("Failed to serialize game state");

    File::create(user_path.as_ref().join(SAVE_FILE))
        .and_then(|mut f| f.write_all(&encoded))
        .expect("Failed to save game");

    GameState::from(serializable)
}

pub fn load<P: AsRef<Path>>(user_path: P) -> Option<GameState> {
    if let Ok(mut f) = File::open(user_path.as_ref().join(SAVE_FILE)) {
        let mut encoded = Vec::new();
        f.read_to_end(&mut encoded).expect("Failed to read save file");
        let serializable: SerializableGameState = bincode::deserialize(&encoded).expect("Failed to parse save file");
        Some(GameState::from(serializable))
    } else {
        None
    }
}

pub fn delete<P: AsRef<Path>>(user_path: P) -> bool {
    fs::remove_file(user_path.as_ref().join(SAVE_FILE)).is_ok()
}
