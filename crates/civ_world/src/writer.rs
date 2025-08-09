use std::{fs, path::PathBuf};

use common::world::Chunk;

use crate::WorldGeneratorError;

pub trait Writer {
    #[allow(unused)]
    fn write_chunk(&self, chunk: Chunk) -> Result<(), WorldGeneratorError>;
}

pub struct FilesWriter {
    target: PathBuf,
}

impl FilesWriter {
    #[allow(unused)]
    pub fn new(target: PathBuf) -> Self {
        Self { target }
    }
}

impl Writer for FilesWriter {
    fn write_chunk(&self, chunk: Chunk) -> Result<(), WorldGeneratorError> {
        let file_name = format!("{}_{}.ct", chunk.x, chunk.y);
        fs::write(
            self.target.join(file_name),
            bincode::serialize(&chunk)
                .map_err(|e| WorldGeneratorError::BincodeError(e.to_string()))?,
        )
        .map_err(|e| WorldGeneratorError::DiskError(e.kind()))?;
        Ok(())
    }
}
