use civ_world::generator::Generator;
use common::world::{Chunk, TerrainType, Tile, World};
use derive_more::Constructor;

#[derive(Debug, Constructor, Clone)]
pub struct PatternGenerator {
    pattern: Vec<TerrainType>,
}

impl Generator for PatternGenerator {
    fn generate_chunk(
        &self,
        world: &World,
        chunk_x: u64,
        chunk_y: u64,
    ) -> Result<Chunk, civ_world::WorldGeneratorError> {
        let mut terrain = self.pattern.iter().cycle();
        let mut tiles = vec![];

        for _ in 0..world.chunk_size {
            for _ in 0..world.chunk_size {
                tiles.push(Tile::new(*terrain.next().unwrap()));
            }
        }

        Ok(Chunk {
            x: chunk_x,
            y: chunk_y,
            tiles,
        })
    }
}
