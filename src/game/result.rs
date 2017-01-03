use std::result;
use behaviour;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingComponent,
    ScheduleEmpty,
    BehaviourError(behaviour::Error),
}

impl From<behaviour::Error> for Error {
    fn from(e: behaviour::Error) -> Self {
        Error::BehaviourError(e)
    }
}

pub type ExternalResult<T> = result::Result<T, String>;

impl From<Error> for String {
    fn from(e: Error) -> Self {
        format!("{:?}", e)
    }
}
