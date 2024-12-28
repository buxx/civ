use std::{fs, io, path::PathBuf};

use thiserror::Error;

use crate::space::window::Window;

use super::{Chunk, Tile, World};

pub trait WorldReader {
    type Error_;

    fn init(&mut self) -> Result<(), WorldReaderError<Self::Error_>> {
        Ok(())
    }
    fn shape(&self) -> u64;
    fn width(&self) -> u64;
    fn height(&self) -> u64;
    fn tile(&self, x: u64, y: u64) -> Option<&Tile>;
    fn window_tiles(&self, window: &Window) -> Vec<&Tile>;
}

#[derive(Error, Debug)]
pub enum WorldReaderError<T> {
    #[error("Failed to init world: {0}")]
    InitWorldError(T),
    #[error("World not initialized")]
    NotInitialized,
}

pub struct FullMemoryWorldReader {
    source: PathBuf,
    width: u64,
    height: u64,
    tiles: Vec<Tile>,
}

impl FullMemoryWorldReader {
    pub fn new(source: PathBuf) -> Self {
        Self {
            source,
            width: 0,
            height: 0,
            tiles: vec![],
        }
    }
}

#[derive(Error, Debug)]
pub enum FullMemoryWorldReaderError {
    #[error("Disk access error : {0}")]
    Io(#[from] io::Error),
    #[error("World.ron load error : {0}")]
    InvalidWorld(#[from] ron::de::SpannedError),
    #[error("Chunk decoding error : {0}")]
    InvalidChunk(#[from] Box<bincode::ErrorKind>),
}

impl WorldReader for FullMemoryWorldReader {
    type Error_ = FullMemoryWorldReaderError;

    fn init(&mut self) -> Result<(), WorldReaderError<Self::Error_>> {
        let world: World = ron::from_str(
            &fs::read_to_string(self.source.join("world.ron"))
                .map_err(|e| WorldReaderError::InitWorldError(FullMemoryWorldReaderError::Io(e)))?,
        )
        .map_err(|e| {
            WorldReaderError::InitWorldError(FullMemoryWorldReaderError::InvalidWorld(e))
        })?;

        let chunked_width = world.width / world.chunk_size;
        let chunked_height = world.height / world.chunk_size;

        self.tiles.clear();
        self.width = world.width;
        self.height = world.height;

        for chunk_x in 0..chunked_width {
            for chunk_y in 0..chunked_height {
                let file_name = format!("{}_{}.ct", chunk_x, chunk_y);
                let chunk: Chunk =
                    bincode::deserialize(&fs::read(self.source.join(file_name)).map_err(|e| {
                        WorldReaderError::InitWorldError(FullMemoryWorldReaderError::Io(e))
                    })?)
                    .map_err(|e| {
                        WorldReaderError::InitWorldError(FullMemoryWorldReaderError::InvalidChunk(
                            e,
                        ))
                    })?;
                self.tiles.extend(chunk.tiles);
            }
        }

        Ok(())
    }

    fn tile(&self, x: u64, y: u64) -> Option<&Tile> {
        let index = y * self.width + x;
        self.tiles.get(index as usize)
    }

    fn shape(&self) -> u64 {
        self.tiles.len() as u64
    }

    fn window_tiles(&self, window: &Window) -> Vec<&Tile> {
        tiles_from_window(&self.tiles, window, self.width)
    }

    fn width(&self) -> u64 {
        self.width
    }

    fn height(&self) -> u64 {
        self.height
    }
}

pub fn tiles_from_window<'a>(
    world_tiles: &'a Vec<Tile>,
    window: &Window,
    world_width: u64,
) -> Vec<&'a Tile> {
    let mut tiles = vec![];

    for y in window.start_y()..window.end_y() + 1 {
        let line_start_index = y * world_width + window.start_x();
        let line_end_index = y * world_width + window.end_x();
        // FIXME: manage window outside world
        let line_tiles = &world_tiles[line_start_index as usize..(line_end_index + 1) as usize];
        tiles.extend(line_tiles);
    }

    tiles
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{space::window::DisplayStep, world::TerrainType};

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
