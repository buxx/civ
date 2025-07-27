use derive_more::Constructor;
use serde::{Deserialize, Serialize};

pub mod window;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Constructor)]
pub struct D2Size {
    width: usize,
    height: usize,
}

impl D2Size {
    pub fn len(&self) -> usize {
        self.width * self.height
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CityVec2dIndex(pub usize);

impl std::fmt::Display for CityVec2dIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UnitVec2dIndex(pub usize, pub usize);

impl std::fmt::Display for UnitVec2dIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}.{}", self.0, self.1))
    }
}
