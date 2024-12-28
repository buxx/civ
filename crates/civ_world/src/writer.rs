use std::{fs, path::PathBuf};

use common::world::Chunk;

use crate::WorldGeneratorError;

pub trait Writer {
    fn write_chunk(&self, chunk: Chunk) -> Result<(), WorldGeneratorError>;
}

pub struct FilesWriter {
    target: PathBuf,
}

impl FilesWriter {
    pub fn new(target: PathBuf) -> Self {
        Self { target }
    }
}

impl Writer for FilesWriter {
    fn write_chunk(&self, chunk: Chunk) -> Result<(), WorldGeneratorError> {
        let file_name = format!("{}_{}.ct", chunk.x, chunk.y);
        fs::write(self.target.join(file_name), bincode::serialize(&chunk)?)?;
        Ok(())
    }
}
