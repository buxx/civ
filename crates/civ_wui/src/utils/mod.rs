use std::path::PathBuf;

use home::home_dir;

pub mod assets;
#[cfg(target_arch = "wasm32")]
pub mod cookies;
#[cfg(feature = "debug")]
pub mod debug;
pub mod tile;

pub fn app_dir() -> Option<PathBuf> {
    home_dir().map(|p| p.join(".civ"))
}
