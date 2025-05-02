use std::thread;

use async_std::channel::unbounded;
use civ_world::generator::random::RandomGenerator;
use civ_world::writer::FilesWriter;
use civ_world::{run, Args, WorldGeneratorError};
use clap::Parser;
use common::utils::Progress;

mod generator;
mod writer;

fn main() -> Result<(), WorldGeneratorError> {
    let args = Args::parse();
    let (progress_sender, progress_receiver) = unbounded();

    thread::spawn(move || {
        let target = args.target.clone();
        let writer = FilesWriter::new(target.clone());
        let world = args.into();
        let _ = run()
            // TODO: Choose generator type by arg
            .generator(RandomGenerator)
            .target(&target)
            .world(&world)
            .writer(&writer)
            .progress(progress_sender)
            .call();
    });

    while let Ok(progress) = progress_receiver.recv_blocking() {
        match progress {
            Progress::InProgress(value) => println!("{}%", (value * 100.) as usize),
            Progress::Finished => break,
            Progress::Error(error) => {
                println!("Error: {}", error);
                break;
            }
        }
    }

    Ok(())
}
