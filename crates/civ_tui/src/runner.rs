use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};

use bon::Builder;
use clap::Parser;
use common::network::message::{ClientToServerMessage, ServerToClientMessage};
use crossbeam::channel::{Receiver, Sender};

use crate::{
    command::{self, Command, SubCommand, WindowSubCommand},
    context::Context,
    state::State,
};

#[derive(Builder)]
pub struct Runner {
    context: Arc<Mutex<Context>>,
    state: Arc<Mutex<State>>,
    from_server_receiver: Receiver<ServerToClientMessage>,
    to_server_sender: Sender<ClientToServerMessage>,
}

impl Runner {
    pub fn run(&mut self) {
        println!("Type help for help");
        loop {
            let mut input = String::new();
            print!("> ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input)
                // TODO
                .expect("error: unable to read user input");

            let mut args = vec!["tui".to_string()];
            args.extend(shellwords::split(&input).unwrap());
            match Command::try_parse_from(args) {
                Ok(command) => {
                    match command.subcommand {
                        SubCommand::Status => {
                            command::status::status();
                        }
                        SubCommand::Window { subcommand } => {
                            match subcommand {
                                WindowSubCommand::Set {
                                    start_x,
                                    start_y,
                                    end_x,
                                    end_y,
                                } => {
                                    command::window::set(start_x, start_y, end_x, end_y);
                                }
                            };
                        }
                    };
                }
                Err(error) => {
                    println!("KO: {}", error)
                }
            }
        }
    }
}
