use std::{io, path::PathBuf};

use async_std::channel::Sender;
use bon::{builder, Builder};
use clap::Parser;
use common::{utils::Progress, world::World};
use generator::Generator;
use thiserror::Error;
use writer::Writer;

pub mod config;
pub mod generator;
pub mod writer;

#[derive(Parser, Debug, Builder)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub target: PathBuf,
    pub width: usize,
    pub height: usize,
    #[builder(default = 5000)]
    #[arg(short, long, default_value_t = 5000)]
    pub chunk_size: usize,
}

#[builder]
pub fn run<T: Generator>(
    generator: T,
    world: &World,
    target: &PathBuf,
    writer: &dyn Writer,
    progress: Option<Sender<Progress<WorldGeneratorError>>>,
) -> Result<(), WorldGeneratorError> {
    if let Err(error) = run_(generator, world, target, writer, progress.clone()) {
        progress.map(|p| p.send_blocking(Progress::Error(error.clone())));
        return Err(error);
    }

    Ok(())
}

pub fn run_<T: Generator>(
    generator: T,
    world: &World,
    target: &PathBuf,
    writer: &dyn Writer,
    progress: Option<Sender<Progress<WorldGeneratorError>>>,
) -> Result<(), WorldGeneratorError> {
    if world.width % world.chunk_size > 0 || world.height % world.chunk_size > 0 {
        return Err(WorldGeneratorError::NotChunkSizeMultiplier(
            world.chunk_size as usize,
        ));
    }

    if target.exists() {
        return Err(WorldGeneratorError::TargetAlreadyExist);
    }

    let world = World::builder()
        .chunk_size(world.chunk_size)
        .width(world.width)
        .height(world.height)
        .build();
    generator.generate(&world, target, writer, progress)?;

    Ok(())
}

#[derive(Error, Debug, Clone)]
pub enum WorldGeneratorError {
    #[error("Please use chunk size multiplier for height and with ({0})")]
    NotChunkSizeMultiplier(usize),
    #[error("Target path already exist")]
    TargetAlreadyExist,
    #[error("Disk error: {0}")]
    DiskError(io::ErrorKind),
    #[error("Serialization (ron) error: {0}")]
    RonError(#[from] ron::Error),
    #[error("Serialization (bin) error: {0}")]
    BincodeError(String),
}

impl From<Args> for World {
    fn from(value: Args) -> Self {
        Self::builder()
            .width(value.width as u64)
            .height(value.height as u64)
            .chunk_size(value.chunk_size as u64)
            .build()
    }
}
