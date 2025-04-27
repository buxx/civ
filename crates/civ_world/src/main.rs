use civ_world::{run, Args, WorldGeneratorError};
use clap::Parser;

mod generator;
mod writer;

fn main() -> Result<(), WorldGeneratorError> {
    run().args(Args::parse()).call()
}
