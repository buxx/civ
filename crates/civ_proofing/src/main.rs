use std::{num::ParseIntError, thread, time::Duration};

use clap::Parser;
use client::Client;
use common::game::nation::flag::Flag;
use context::Context;
use log::error;
use strum::IntoEnumIterator;
use thiserror::Error;

mod client;
mod context;
mod utils;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// Server address
    #[arg(short, long, default_value = "127.0.0.1:9876")]
    pub address: String,
    /// Number of client stuff iteration
    pub iteration: usize,
    /// Number of simulated clients
    pub clients: usize,
    /// Distance between placements
    pub distance: u64,
    /// Interval between clients creation
    #[arg(long, value_parser = |arg: &str| -> Result<Duration, ParseIntError> {Ok(Duration::from_millis(arg.parse()?))}, default_value = "250")]
    pub create_client_interval: Duration,
    /// Time to wait server connection before give up
    #[arg(long, short, value_parser = |arg: &str| -> Result<Duration, ParseIntError> {Ok(Duration::from_secs(arg.parse()?))}, default_value = "5")]
    pub connect_timeout: Duration,
    /// Time to wait server placed client
    #[arg(long, short, value_parser = |arg: &str| -> Result<Duration, ParseIntError> {Ok(Duration::from_secs(arg.parse()?))}, default_value = "10")]
    pub placed_timeout: Duration,
    /// Time to wait server placed client
    #[arg(long, short, value_parser = |arg: &str| -> Result<Duration, ParseIntError> {Ok(Duration::from_secs(arg.parse()?))}, default_value = "10")]
    pub game_slice_timeout: Duration,
    #[arg(long, value_parser = |arg: &str| -> Result<Duration, ParseIntError> {Ok(Duration::from_secs(arg.parse()?))}, default_value = "60")]
    pub city_timeout: Duration,
}

#[derive(Debug, Error)]
enum ProofingError {
    #[error("Too many clients (client number is limited by flags)")]
    TooManyClient,
}

fn main() -> Result<(), ProofingError> {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::init_from_env(env);
    let args = Arguments::parse();
    let context = Context::new(args.clone());

    let mut flags = Flag::iter();
    let mut threads = vec![];
    for i in 0..args.clients {
        let flag = flags.next().ok_or(ProofingError::TooManyClient)?;
        let client = Client::new(flag);
        let context = context.clone();
        threads.push(thread::spawn(move || {
            if let Err(error) = client::run(context, client.clone()) {
                error!("Client {} error: {}", i, error)
            }
        }));
        thread::sleep(args.create_client_interval);
    }

    for (i, thread) in threads.into_iter().enumerate() {
        if let Err(e) = thread.join() {
            error!("Cant't wait client {} thread: {:?}", i, e);
        }
    }

    Ok(())
}
