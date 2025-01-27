use std::{fs, io, path::PathBuf};

use thiserror::Error;

use crate::space::window::Window;

use crate::world::{Chunk, Tile, World};

use super::CtxTile;

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
    // FIXME: not required as attribute ?
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

    pub fn window_tiles(&self, window: &Window) -> Vec<CtxTile<&Tile>> {
        tiles_from_window(&self.tiles, window, self.width as i64, self.height as i64)
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn height(&self) -> u64 {
        self.height
    }
}

// TODO: generic for Units and Cities
pub fn tiles_from_window<'a>(
    world_tiles: &'a [Tile],
    window: &Window,
    world_width: i64,
    world_height: i64,
) -> Vec<CtxTile<&'a Tile>> {
    let row_width = (window.end().x - window.start().x + 1) as usize;
    let mut tiles = Vec::with_capacity(window.shape() as usize);

    for y in window.start().y..=window.end().y {
        if y < 0 || y >= world_height {
            tiles.resize(tiles.len() + row_width, CtxTile::Outside);
            continue;
        }

        let start_x = window.start().x;
        let end_x = window.end().x;

        // Left out-of-bounds padding
        let left_padding = (-start_x).max(0).min(row_width as i64) as usize;
        tiles.resize(tiles.len() + left_padding, CtxTile::Outside);

        // Visible tiles within world bounds
        if start_x < world_width && end_x >= 0 {
            let clamped_start_x = start_x.max(0).min(world_width - 1);
            let clamped_end_x = end_x.min(world_width - 1);

            if clamped_start_x <= clamped_end_x {
                let line_start_idx = (y * world_width + clamped_start_x) as usize;
                let line_end_idx = (y * world_width + clamped_end_x) as usize;

                tiles.extend(
                    world_tiles[line_start_idx..=line_end_idx]
                        .iter()
                        .map(CtxTile::Visible),
                );
            }
        }

        // Right out-of-bounds padding
        let right_padding = ((end_x + 1) - world_width).max(0).min(row_width as i64) as usize;
        tiles.resize(tiles.len() + right_padding, CtxTile::Outside);
    }

    tiles
}

// FIXME BS NOW: add tests with CtxTile::Outside
#[cfg(test)]
mod test {
    use crate::{geo::ImaginaryWorldPoint, space::window::DisplayStep, world::TerrainType};
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
        let world_height = 4;
        let window = Window::new((1, 1).into(), (3, 3).into(), DisplayStep::Close);

        // WHEN
        let window_tiles: Vec<CtxTile<Tile>> =
            tiles_from_window(&world_tiles, &window, world_width, world_height)
                .into_iter()
                .map(|t| t.into())
                .collect();

        // THEN
        assert_eq!(
            window_tiles,
            vec![
                //
                CtxTile::Visible(Tile {
                    type_: TerrainType::Plain,
                }),
                CtxTile::Visible(Tile {
                    type_: TerrainType::Plain,
                }),
                CtxTile::Visible(Tile {
                    type_: TerrainType::GrassLand,
                }),
                //
                CtxTile::Visible(Tile {
                    type_: TerrainType::Plain,
                }),
                CtxTile::Visible(Tile {
                    type_: TerrainType::Plain,
                }),
                CtxTile::Visible(Tile {
                    type_: TerrainType::GrassLand,
                }),
                //
                CtxTile::Visible(Tile {
                    type_: TerrainType::GrassLand,
                }),
                CtxTile::Visible(Tile {
                    type_: TerrainType::GrassLand,
                }),
                CtxTile::Visible(Tile {
                    type_: TerrainType::GrassLand,
                }),
            ]
        );
    }

    #[rstest]
    fn test_tiles_from_window_outside() {
        // GIVEN
        let world_tiles = vec![
            // line 0
            Tile {
                type_: TerrainType::GrassLand,
            },
            Tile {
                type_: TerrainType::Plain,
            },
            // line 1
            Tile {
                type_: TerrainType::GrassLand,
            },
            Tile {
                type_: TerrainType::Plain,
            },
        ];
        let world_width = 2;
        let world_height = 2;
        let window = Window::new(
            ImaginaryWorldPoint::new(-1, -1),
            ImaginaryWorldPoint::new(2, 2),
            DisplayStep::Close,
        );

        // WHEN
        let window_tiles: Vec<CtxTile<Tile>> =
            tiles_from_window(&world_tiles, &window, world_width, world_height)
                .into_iter()
                .map(|t| t.into())
                .collect();

        // THEN
        assert_eq!(
            window_tiles,
            vec![
                //
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                //
                CtxTile::Outside,
                CtxTile::Visible(Tile {
                    type_: TerrainType::GrassLand,
                }),
                CtxTile::Visible(Tile {
                    type_: TerrainType::Plain,
                }),
                CtxTile::Outside,
                //
                CtxTile::Outside,
                CtxTile::Visible(Tile {
                    type_: TerrainType::GrassLand,
                }),
                CtxTile::Visible(Tile {
                    type_: TerrainType::Plain,
                }),
                CtxTile::Outside,
                //
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
            ]
        );
    }

    #[rstest]
    fn test_tiles_from_window_non_reg_slice() {
        // GIVEN
        let world_tiles = vec![
            Tile {
                type_: TerrainType::GrassLand,
            };
            100
        ];
        let world_width = 10;
        let world_height = 10;
        let window = Window::new(
            ImaginaryWorldPoint::new(-6, 3),
            ImaginaryWorldPoint::new(-2, 7),
            DisplayStep::Close,
        );

        // WHEN-THEN
        tiles_from_window(&world_tiles, &window, world_width, world_height);
    }

    #[rstest]
    fn test_tiles_from_window_non_reg_slice2() {
        // GIVEN
        let world_tiles = vec![
            Tile {
                type_: TerrainType::GrassLand,
            };
            100
        ];
        let world_width = 10;
        let world_height = 10;
        let window = Window::new(
            ImaginaryWorldPoint::new(11, 3),
            ImaginaryWorldPoint::new(15, 7),
            DisplayStep::Close,
        );

        // WHEN-THEN
        tiles_from_window(&world_tiles, &window, world_width, world_height);
    }

    #[rstest]
    fn test_tiles_from_window_outside2() {
        // GIVEN
        let world_tiles = vec![
            Tile {
                type_: TerrainType::GrassLand,
            };
            100
        ];
        let world_width = 10;
        let world_height = 10;
        let window = Window::new(
            ImaginaryWorldPoint::new(10, 3),
            ImaginaryWorldPoint::new(14, 7),
            DisplayStep::Close,
        );

        // WHEN-THEN
        // WHEN
        let window_tiles: Vec<CtxTile<Tile>> =
            tiles_from_window(&world_tiles, &window, world_width, world_height)
                .into_iter()
                .map(|t| t.into())
                .collect();

        // THEN
        assert_eq!(
            window_tiles,
            vec![
                //
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                //
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                //
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                //
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                //
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
            ]
        );
    }
}
