use std::error::Error as StdError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreateTaskError {
    #[error("Action is no longer possible: {0}")]
    IncoherentContext(String, Option<Box<dyn StdError>>),
    #[error("Action is no longer possible: {0}")]
    GamePlay(GamePlayError),
}

#[derive(Error, Debug)]
pub enum GamePlayError {
    #[error("Cant settle: {0}")]
    CantSettle(String),
}
