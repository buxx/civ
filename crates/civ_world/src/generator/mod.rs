use std::{fs, path::PathBuf};

use common::world::{Chunk, TerrainType, Tile, World};

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

    fn generate_chunk(&self, chunk_x: usize, chunk_y: usize) -> Result<Chunk, WorldGeneratorError> {
        // TODO: write a real terrain generator ...
        let mut tiles = vec![];

        for _ in 0..self.world.chunk_size {
            for _ in 0..self.world.chunk_size {
                tiles.push(Tile {
                    type_: TerrainType::GrassLand,
                });
            }
        }

        Ok(Chunk {
            x: chunk_x,
            y: chunk_y,
            tiles,
        })
    }
}
