use std::{fs, io, path::PathBuf};

use thiserror::Error;

use common::space::window::Window;

use common::world::{Chunk, Tile, World};

#[derive(Error, Debug)]
pub enum WorldReaderError {
    #[error("Failed to init world: {0}")]
    InitWorldError(InitWorldError),
    #[error("World not initialized")]
    NotInitialized,
}

#[derive(Error, Debug)]
pub enum InitWorldError {
    #[error("Disk access error : {0}")]
    Io(#[from] io::Error),
    #[error("World.ron load error : {0}")]
    InvalidWorld(#[from] ron::de::SpannedError),
    #[error("Chunk decoding error : {0}")]
    InvalidChunk(#[from] Box<bincode::ErrorKind>),
}

pub struct WorldReader {
    source: PathBuf,
    width: u64,
    height: u64,
    tiles: Vec<Tile>,
}

impl WorldReader {
    pub fn new(source: PathBuf, width: u64, height: u64, tiles: Vec<Tile>) -> Self {
        Self {
            source,
            width,
            height,
            tiles,
        }
    }

    pub fn from(source: PathBuf) -> Result<Self, WorldReaderError> {
        let mut self_ = Self {
            source,
            width: 0,
            height: 0,
            tiles: vec![],
        };

        let world: World = ron::from_str(
            &fs::read_to_string(self_.source.join("world.ron"))
                .map_err(|e| WorldReaderError::InitWorldError(InitWorldError::Io(e)))?,
        )
        .map_err(|e| WorldReaderError::InitWorldError(InitWorldError::InvalidWorld(e)))?;

        let chunked_width = world.width / world.chunk_size;
        let chunked_height = world.height / world.chunk_size;

        self_.tiles.clear();
        self_.width = world.width;
        self_.height = world.height;

        for chunk_x in 0..chunked_width {
            for chunk_y in 0..chunked_height {
                let file_name = format!("{}_{}.ct", chunk_x, chunk_y);
                let chunk: Chunk = bincode::deserialize(
                    &fs::read(self_.source.join(file_name))
                        .map_err(|e| WorldReaderError::InitWorldError(InitWorldError::Io(e)))?,
                )
                .map_err(|e| WorldReaderError::InitWorldError(InitWorldError::InvalidChunk(e)))?;
                self_.tiles.extend(chunk.tiles);
            }
        }

        Ok(self_)
    }

    pub fn tile(&self, x: u64, y: u64) -> Option<&Tile> {
        let index = y * self.width + x;
        self.tiles.get(index as usize)
    }

    pub fn shape(&self) -> u64 {
        self.tiles.len() as u64
    }

    pub fn window_tiles(&self, window: &Window) -> Vec<&Tile> {
        tiles_from_window(&self.tiles, window, self.width)
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn height(&self) -> u64 {
        self.height
    }
}

pub fn tiles_from_window<'a>(
    world_tiles: &'a [Tile],
    window: &Window,
    world_width: u64,
) -> Vec<&'a Tile> {
    let mut tiles = vec![];

    for y in window.start_y()..window.end_y() + 1 {
        let line_start_index = y * world_width + window.start_x();
        let line_end_index = y * world_width + window.end_x();
        // FIXME: manage window outside world
        let line_tiles = &world_tiles[line_start_index as usize..=line_end_index as usize];
        tiles.extend(line_tiles);
    }

    tiles
}

#[cfg(test)]
mod test {
    use common::{space::window::DisplayStep, world::TerrainType};
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_tiles_from_window() {
        // GIVEN
        let world_tiles = vec![
            // line 0
            Tile {
                type_: TerrainType::GrassLand,
            },
            Tile {
                type_: TerrainType::GrassLand,
            },
            Tile {
                type_: TerrainType::GrassLand,
            },
            Tile {
                type_: TerrainType::GrassLand,
            },
            // line 1
            Tile {
                type_: TerrainType::GrassLand,
            },
            Tile {
                type_: TerrainType::Plain,
            },
            Tile {
                type_: TerrainType::Plain,
            },
            Tile {
                type_: TerrainType::GrassLand,
            },
            // line 2
            Tile {
                type_: TerrainType::GrassLand,
            },
            Tile {
                type_: TerrainType::Plain,
            },
            Tile {
                type_: TerrainType::Plain,
            },
            Tile {
                type_: TerrainType::GrassLand,
            },
            // line 3
            Tile {
                type_: TerrainType::GrassLand,
            },
            Tile {
                type_: TerrainType::GrassLand,
            },
            Tile {
                type_: TerrainType::GrassLand,
            },
            Tile {
                type_: TerrainType::GrassLand,
            },
        ];
        let world_width = 4;
        let window = Window::new(1, 1, 3, 3, DisplayStep::Close);

        // WHEN
        let window_tiles: Vec<Tile> = tiles_from_window(&world_tiles, &window, world_width)
            .into_iter()
            .cloned()
            .collect();

        // THEN
        assert_eq!(
            window_tiles,
            vec![
                //
                Tile {
                    type_: TerrainType::Plain,
                },
                Tile {
                    type_: TerrainType::Plain,
                },
                Tile {
                    type_: TerrainType::GrassLand,
                },
                //
                Tile {
                    type_: TerrainType::Plain,
                },
                Tile {
                    type_: TerrainType::Plain,
                },
                Tile {
                    type_: TerrainType::GrassLand,
                },
                //
                Tile {
                    type_: TerrainType::GrassLand,
                },
                Tile {
                    type_: TerrainType::GrassLand,
                },
                Tile {
                    type_: TerrainType::GrassLand,
                },
            ]
        );
    }
}
