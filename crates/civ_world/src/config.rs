use std::path::PathBuf;

use bon::Builder;
use common::world::World;
use derive_more::Constructor;

use crate::Args;

// TODO: For now, contain same than Args, but will contains climatic info, etc
#[derive(Debug, Builder, Clone, Constructor)]
pub struct WorldConfig {
    pub target: PathBuf,
    pub width: usize,
    pub height: usize,
    #[builder(default = 5000)]
    pub chunk_size: usize,
}

impl From<WorldConfig> for Args {
    fn from(value: WorldConfig) -> Self {
        let WorldConfig {
            target,
            width,
            height,
            chunk_size,
        } = value;
        Self {
            target,
            width,
            height,
            chunk_size,
        }
    }
}

impl From<WorldConfig> for World {
    fn from(value: WorldConfig) -> Self {
        Self::builder()
            .width(value.width as u64)
            .height(value.height as u64)
            .chunk_size(value.chunk_size as u64)
            .build()
    }
}
