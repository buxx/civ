use async_std::channel::Sender;
use common::{
    utils::Progress,
    world::{Chunk, TerrainType, Tile, World},
};
use std::{fs, path::PathBuf};
use weighted_rand::builder::{NewBuilder, WalkerTableBuilder};

use crate::{writer::Writer, WorldGeneratorError};

pub struct Generator {
    world: World,
    writer: Box<dyn Writer>,
    target: PathBuf,
}

impl Generator {
    pub fn new(world: World, writer: Box<dyn Writer>, target: PathBuf) -> Self {
        Self {
            world,
            writer,
            target,
        }
    }

    pub fn generate(
        &self,
        progress: Option<Sender<Progress<WorldGeneratorError>>>,
    ) -> Result<(), WorldGeneratorError> {
        progress
            .as_ref()
            .map(|s| s.send_blocking(Progress::InProgress(0.)));
        std::fs::create_dir_all(&self.target)
            .map_err(|e| WorldGeneratorError::DiskError(e.kind()))?;
        fs::write(
            self.target.join("world.ron"),
            ron::ser::to_string_pretty(&self.world, ron::ser::PrettyConfig::default())?,
        )
        .map_err(|e| WorldGeneratorError::DiskError(e.kind()))?;

        let chunked_width = self.world.width / self.world.chunk_size;
        let chunked_height = self.world.height / self.world.chunk_size;
        let mut done = 0;
        let expected = self.world.width * self.world.height;

        for chunk_y in 0..chunked_height {
            for chunk_x in 0..chunked_width {
                let chunk = self.generate_chunk(chunk_x, chunk_y)?;
                done += chunk.tiles.len();

                self.writer.write_chunk(chunk)?;

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

    fn generate_chunk(&self, chunk_x: u64, chunk_y: u64) -> Result<Chunk, WorldGeneratorError> {
        // TODO: write a real terrain generator ...
        let mut tiles = vec![];
        let terrains = [TerrainType::GrassLand, TerrainType::Plain];
        let index_weights = [25, 100];

        let builder = WalkerTableBuilder::new(&index_weights);
        let wa_table = builder.build();

        for _ in 0..self.world.chunk_size {
            for _ in 0..self.world.chunk_size {
                tiles.push(Tile::new(terrains[wa_table.next()]));
            }
        }

        Ok(Chunk {
            x: chunk_x,
            y: chunk_y,
            tiles,
        })
    }
}
