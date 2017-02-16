use std::result;
use behaviour;

pub type GameResult<T> = result::Result<T, GameError>;

#[derive(Debug)]
pub enum GameError {
    ScheduleEmpty,
    BehaviourError(behaviour::Error),
}

impl From<behaviour::Error> for GameError {
    fn from(e: behaviour::Error) -> Self {
        GameError::BehaviourError(e)
    }
}

pub type ExternalResult<T> = result::Result<T, String>;

impl From<GameError> for String {
    fn from(e: GameError) -> Self {
        format!("{:?}", e)
    }
}
