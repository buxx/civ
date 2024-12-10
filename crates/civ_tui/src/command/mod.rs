pub mod status;
pub mod window;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Command {
    #[clap(flatten)]
    pub global_opts: GlobalOpts,

    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Debug, Args)]
pub struct GlobalOpts {
    #[clap(long, short, action)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Status,
    Window {
        #[clap(subcommand)]
        subcommand: WindowSubCommand,
    },
}
#[derive(Debug, Subcommand)]
pub enum WindowSubCommand {
    Set {
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
    },
}
