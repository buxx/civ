use std::{io, path::PathBuf};

use async_std::channel::Sender;
use bon::{builder, Builder};
use clap::Parser;
use common::{utils::Progress, world::World};
use generator::Generator;
use thiserror::Error;
use writer::FilesWriter;

pub mod config;
mod generator;
mod writer;

#[derive(Parser, Debug, Builder)]
#[command(version, about, long_about = None)]
pub struct Args {
    target: PathBuf,
    width: usize,
    height: usize,
    #[builder(default = 5000)]
    #[arg(short, long, default_value_t = 5000)]
    chunk_size: usize,
}

#[builder]
pub fn run(
    args: Args,
    progress: Option<Sender<Progress<WorldGeneratorError>>>,
) -> Result<(), WorldGeneratorError> {
    if let Err(error) = run_(args, progress.clone()) {
        progress.map(|p| p.send_blocking(Progress::Error(error.clone())));
        return Err(error);
    }

    Ok(())
}

pub fn run_(
    args: Args,
    progress: Option<Sender<Progress<WorldGeneratorError>>>,
) -> Result<(), WorldGeneratorError> {
    if args.width % args.chunk_size > 0 || args.height % args.chunk_size > 0 {
        return Err(WorldGeneratorError::NotChunkSizeMultiplier(args.chunk_size));
    }

    if args.target.exists() {
        return Err(WorldGeneratorError::TargetAlreadyExist);
    }

    let world = World::builder()
        .chunk_size(args.chunk_size as u64)
        .width(args.width as u64)
        .height(args.height as u64)
        .build();
    Generator::new(
        world,
        Box::new(FilesWriter::new(args.target.clone())),
        args.target,
    )
    .generate(progress)?;

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
