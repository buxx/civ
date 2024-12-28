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

    // FIXME: write tests
    fn window_tiles(&self, window: &Window) -> Vec<&Tile> {
        let mut tiles = vec![];

        for y in window.start_y()..window.end_y() {
            let line_start_index = y * self.width + window.start_x();
            let line_end_index = y * self.width + window.end_x();
            // FIXME: manage window outside world
            let line_tiles = &self.tiles[line_start_index as usize..line_end_index as usize];
            tiles.extend(line_tiles);
        }

        tiles
    }

    fn width(&self) -> u64 {
        self.width
    }

    fn height(&self) -> u64 {
        self.height
    }
}
