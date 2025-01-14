use thiserror::Error;

use crate::game::unit::UnitType;

#[derive(Error, Debug)]
pub enum CreateTaskError {
    #[error("Action is not possible: {0}")]
    GamePlay(GamePlayReason),
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

#[derive(Error, Debug)]
pub enum GamePlayReason {
    #[error("Cant settle: {0}")]
    CantSettle(CantSettleReason),
    #[error("City no longer exist")]
    CityNoLongerExist,
    #[error("Unit no longer exist")]
    UnitNoLongerExist,
    #[error("Player no longer exist")]
    PlayerNoLongerExist,
}

#[derive(Error, Debug)]
pub enum CantSettleReason {
    #[error("{0} can't settle")]
    WrongUnitType(UnitType),
}
