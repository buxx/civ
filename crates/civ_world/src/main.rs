use std::{io, path::PathBuf};

use bincode::ErrorKind;
use clap::Parser;
use generator::Generator;
use thiserror::Error;
use world::World;

mod generator;
mod world;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    target: PathBuf,
    width: usize,
    height: usize,
    #[arg(short, long, default_value_t = 5000)]
    chunk_size: usize,
}

pub fn main() -> Result<(), WorldGeneratorError> {
    let args = Args::parse();

    if args.width % args.chunk_size > 0 || args.height % args.chunk_size > 0 {
        return Err(WorldGeneratorError::NotChunkSizeMultiplier(args.chunk_size));
    }

    if args.target.exists() {
        return Err(WorldGeneratorError::TargetAlreadyExist);
    }

    let world = World::builder()
        .chunk_size(args.chunk_size)
        .width(args.width)
        .height(args.height)
        .build();
    Generator::new(world, args.target).generate()?;

    Ok(())
}

#[derive(Error, Debug)]
pub enum WorldGeneratorError {
    #[error("Please use chunk size multiplier for height and with ({0})")]
    NotChunkSizeMultiplier(usize),
    #[error("Target path already exist")]
    TargetAlreadyExist,
    #[error("Disk error: {0}")]
    DiskError(#[from] io::Error),
    #[error("Serialization error: {0}")]
    RonError(#[from] ron::Error),
    #[error("Serialization error: {0}")]
    BincodeError(#[from] Box<ErrorKind>),
}
