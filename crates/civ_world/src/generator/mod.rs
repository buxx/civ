use common::world::{Chunk, TerrainType, Tile, World};
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

    pub fn generate(&self) -> Result<(), WorldGeneratorError> {
        std::fs::create_dir_all(&self.target)?;
        fs::write(
            self.target.join("world.ron"),
            ron::ser::to_string_pretty(&self.world, ron::ser::PrettyConfig::default())?,
        )?;

        let chunked_width = self.world.width / self.world.chunk_size;
        let chunked_height = self.world.height / self.world.chunk_size;

        for chunk_x in 0..chunked_width {
            for chunk_y in 0..chunked_height {
                let chunk = self.generate_chunk(chunk_x, chunk_y)?;
                self.writer.write_chunk(chunk)?;
            }
        }

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
