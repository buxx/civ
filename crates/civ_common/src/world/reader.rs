use std::{fs, io, path::PathBuf};

use thiserror::Error;

use super::{Chunk, Tile, World};

pub trait WorldReader {
    type Error_;

    fn init(&mut self) -> Result<(), WorldReaderError<Self::Error_>> {
        Ok(())
    }
    fn tile(&self, x: u64, y: u64) -> Option<&Tile>;
}

#[derive(Error, Debug)]
pub enum WorldReaderError<T> {
    #[error("Failed to init world: {0}")]
    InitWorldError(T),
}

pub struct FullMemoryWorldReader {
    source: PathBuf,
    tiles: Vec<Tile>,
}

impl FullMemoryWorldReader {
    pub fn new(source: PathBuf) -> Self {
        Self {
            source,
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

    fn tile(&self, _x: u64, _y: u64) -> Option<&Tile> {
        todo!()
    }
}
