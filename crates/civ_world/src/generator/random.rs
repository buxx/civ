use common::world::{Chunk, TerrainType, Tile, World};
use weighted_rand::builder::{NewBuilder, WalkerTableBuilder};

use crate::WorldGeneratorError;

use super::Generator;

pub struct RandomGenerator;

impl Generator for RandomGenerator {
    fn generate_chunk(
        &self,
        world: &World,
        chunk_x: u64,
        chunk_y: u64,
    ) -> Result<Chunk, WorldGeneratorError> {
        // TODO: write a real terrain generator ...
        let mut tiles = vec![];
        let terrains = [TerrainType::GrassLand, TerrainType::Plain];
        let index_weights = [25, 100];

        let builder = WalkerTableBuilder::new(&index_weights);
        let wa_table = builder.build();

        for _ in 0..world.chunk_size {
            for _ in 0..world.chunk_size {
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
