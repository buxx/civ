use civ_server::{start, Args, Error};
use clap::Parser;

fn main() -> Result<(), Error> {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    let args = Args::parse();
    env_logger::init_from_env(env);
    start(args)
}
