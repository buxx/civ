use std::{fs, io, path::PathBuf};

use async_std::channel::Sender;
use common::space::D2Size;
use common::world::slice::Slice;
use common::world::{Chunk, CtxTile, World};
use thiserror::Error;

use common::{space::window::Window, world::Tile};

use common::utils::{slice, Progress};

#[derive(Error, Debug, Clone)]
pub enum WorldReaderError {
    #[error("Failed to init world: {0}")]
    InitWorldError(InitWorldError),
    #[error("World not initialized")]
    NotInitialized,
}

#[derive(Error, Debug, Clone)]
pub enum InitWorldError {
    #[error("Disk access error : {0}")]
    Io(io::ErrorKind),
    #[error("World.ron load error : {0}")]
    InvalidWorld(#[from] ron::de::SpannedError),
    #[error("Chunk decoding error : {0}")]
    InvalidChunk(String),
}

// TODO: rename ... ?
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

    pub fn from(
        source: PathBuf,
        progress: &Option<Sender<Progress<WorldReaderError>>>,
    ) -> Result<Self, WorldReaderError> {
        let mut self_ = Self {
            source,
            width: 0,
            height: 0,
            tiles: vec![],
        };

        let world: World = ron::from_str(
            &fs::read_to_string(self_.source.join("world.ron"))
                .map_err(|e| WorldReaderError::InitWorldError(InitWorldError::Io(e.kind())))?,
        )
        .map_err(|e| WorldReaderError::InitWorldError(InitWorldError::InvalidWorld(e)))?;

        let chunked_width = world.width / world.chunk_size;
        let chunked_height = world.height / world.chunk_size;

        self_.tiles.clear();
        self_.width = world.width;
        self_.height = world.height;
        let mut done = 0;
        let expected = world.width * world.height;

        for chunk_x in 0..chunked_width {
            for chunk_y in 0..chunked_height {
                let file_name = format!("{}_{}.ct", chunk_x, chunk_y);
                let chunk: Chunk =
                    bincode::deserialize(&fs::read(self_.source.join(file_name)).map_err(|e| {
                        WorldReaderError::InitWorldError(InitWorldError::Io(e.kind()))
                    })?)
                    .map_err(|e| {
                        WorldReaderError::InitWorldError(InitWorldError::InvalidChunk(
                            e.to_string(),
                        ))
                    })?;

                done += chunk.tiles.len();
                self_.tiles.extend(chunk.tiles);

                let progress_ = done as f32 / expected as f32;
                progress
                    .as_ref()
                    .map(|s| s.send_blocking(Progress::InProgress(progress_)));
            }
        }

        progress
            .as_ref()
            .map(|s| s.send_blocking(Progress::Finished));
        Ok(self_)
    }

    pub fn tile(&self, x: u64, y: u64) -> Option<&Tile> {
        let index = y * self.width + x;
        self.tiles.get(index as usize)
    }

    pub fn shape(&self) -> u64 {
        self.tiles.len() as u64
    }

    pub fn size(&self) -> D2Size {
        D2Size::new(self.width as usize, self.height as usize)
    }

    pub fn slice(&self, window: &Window) -> Slice<CtxTile<Tile>> {
        let tiles = slice(&self.tiles, window, self.width as i64, self.height as i64);
        Slice::new(
            *window.start(),
            (window.end().x - window.start().x + 1) as u64,
            (window.end().y - window.start().y + 1) as u64,
            tiles
                .into_iter()
                .map(|t| t.into())
                .collect::<Vec<CtxTile<Tile>>>(),
        )
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn height(&self) -> u64 {
        self.height
    }
}
