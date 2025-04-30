use std::path::PathBuf;

use bon::Builder;
use derive_more::Constructor;

use crate::Args;

// TODO: For now, contain same than Args, but will contains climatic info, etc
#[derive(Debug, Builder, Clone, Constructor)]
pub struct WorldConfig {
    target: PathBuf,
    width: usize,
    height: usize,
    #[builder(default = 5000)]
    chunk_size: usize,
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
