use async_std::channel::Sender;
use common::{
    utils::Progress,
    world::{Chunk, World},
};
use std::fs;
use std::path::PathBuf;

use crate::{writer::Writer, WorldGeneratorError};

pub mod random;

pub trait Generator {
    fn generate_chunk(
        &self,
        world: &World,
        chunk_x: u64,
        chunk_y: u64,
    ) -> Result<Chunk, WorldGeneratorError>;

    fn generate(
        &self,
        world: &World,
        target: &PathBuf,
        writer: &dyn Writer,
        progress: Option<Sender<Progress<WorldGeneratorError>>>,
    ) -> Result<(), WorldGeneratorError> {
        progress
            .as_ref()
            .map(|s| s.send_blocking(Progress::InProgress(0.)));
        std::fs::create_dir_all(target).map_err(|e| WorldGeneratorError::DiskError(e.kind()))?;
        fs::write(
            target.join("world.ron"),
            ron::ser::to_string_pretty(world, ron::ser::PrettyConfig::default())?,
        )
        .map_err(|e| WorldGeneratorError::DiskError(e.kind()))?;

        let chunked_width = world.width / world.chunk_size;
        let chunked_height = world.height / world.chunk_size;
        let mut done = 0;
        let expected = world.width * world.height;

        for chunk_y in 0..chunked_height {
            for chunk_x in 0..chunked_width {
                let chunk = self.generate_chunk(world, chunk_x, chunk_y)?;
                done += chunk.tiles.len();

                writer.write_chunk(chunk)?;

                let progress_ = done as f32 / expected as f32;
                progress
                    .as_ref()
                    .map(|s| s.send_blocking(Progress::InProgress(progress_)));
            }
        }

        progress
            .as_ref()
            .map(|s| s.send_blocking(Progress::Finished));
        Ok(())
    }
}

pub trait GeneratorConfig {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn chunk_size(&self) -> usize;
    fn target(&self) -> &PathBuf;
}
