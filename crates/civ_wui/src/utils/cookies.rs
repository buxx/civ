use common::game::PlayerId;
use std::{error::Error as BaseError, str::FromStr};
use thiserror::Error;
use wasm_cookies::CookieOptions;

const COOKIE_PLAYER_ID: &str = "player_id";
const COOKIE_KEEP_CONNECTED: &str = "keep_connected";

pub struct Cookies;

#[derive(Debug, Error)]
pub enum CookiesError {
    #[error("Decode error: {0}")]
    Decode(Box<dyn BaseError>),
}

// TODO: Simplify code with generics
impl Cookies {
    fn options() -> CookieOptions<'static> {
        CookieOptions::default()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_player_id(&self) -> Result<Option<PlayerId>, CookiesError> {
        match wasm_cookies::get(COOKIE_PLAYER_ID) {
            Some(Err(error)) => Err(CookiesError::Decode(Box::new(error))),
            Some(Ok(value)) => Ok(Some(
                PlayerId::from_str(&value)
                    .map_err(|error| CookiesError::Decode(Box::new(error)))?,
            )),
            None => Ok(None),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn set_player_id(&self, player_id: &PlayerId) -> Result<(), CookiesError> {
        wasm_cookies::set(COOKIE_PLAYER_ID, &player_id.to_string(), &Self::options());
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn set_keep_connected(&self, value: bool) -> Result<(), CookiesError> {
        wasm_cookies::set(COOKIE_KEEP_CONNECTED, &value.to_string(), &Self::options());
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_keep_connected(&self) -> Result<Option<bool>, CookiesError> {
        match wasm_cookies::get(COOKIE_KEEP_CONNECTED) {
            Some(Err(error)) => Err(CookiesError::Decode(Box::new(error))),
            Some(Ok(value)) => Ok(Some(
                bool::from_str(&value).map_err(|error| CookiesError::Decode(Box::new(error)))?,
            )),
            None => Ok(None),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_player_id(&self) -> Result<Option<PlayerId>, CookiesError> {
        // This is a fake network implemented, for now, to simplify examples
        Ok(None)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_player_id(&self, player_id: &PlayerId) -> Result<(), CookiesError> {
        // This is a fake network implemented, for now, to simplify examples
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_keep_connected(&self, value: bool) -> Result<(), CookiesError> {
        // This is a fake network implemented, for now, to simplify examples
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_keep_connected(&self) -> Result<Option<bool>, CookiesError> {
        // This is a fake network implemented, for now, to simplify examples
        Ok(None)
    }
}
