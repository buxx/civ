use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
use home::home_dir;

pub mod assets;
pub mod bridge;
pub mod gui;
// #[cfg(target_arch = "wasm32")]
// pub mod cookies;
#[cfg(feature = "debug")]
pub mod debug;
pub mod tile;

#[cfg(not(target_arch = "wasm32"))]
pub fn app_dir() -> Option<PathBuf> {
    home_dir().map(|p| p.join(".civ"))
}
