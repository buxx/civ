use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};

use bon::Builder;
use clap::Parser;
use common::network::message::{ClientToServerMessage, ServerToClientMessage};
use crossbeam::channel::{Receiver, Sender};

use crate::{
    command::{self, Command, CommandContext, SubCommand, WindowSubCommand},
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
            self.print_prompt();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                // TODO
                .expect("error: unable to read user input");

            if !input.trim().is_empty() {
                let mut args = vec!["tui".to_string()];
                args.extend(shellwords::split(&input).unwrap());

                match Command::try_parse_from(args) {
                    Ok(command) => {
                        match command.subcommand {
                            SubCommand::Exit => break,
                            SubCommand::Status => {
                                command::status::status(self.into());
                            }
                            SubCommand::Window { subcommand } => {
                                match subcommand {
                                    WindowSubCommand::Set {
                                        start_x,
                                        start_y,
                                        end_x,
                                        end_y,
                                    } => {
                                        command::window::set(
                                            self.into(),
                                            start_x,
                                            start_y,
                                            end_x,
                                            end_y,
                                        );
                                    }
                                };
                            }
                        };
                    }
                    Err(error) => {
                        println!("{}", error)
                    }
                }
            }
        }

        self.context
            .lock()
            .expect("Assume contexte is accessible")
            .require_stop();
    }

    fn print_prompt(&mut self) {
        print!("> ");
        io::stdout().flush().unwrap();
    }
}

#[allow(clippy::from_over_into)]
impl Into<CommandContext> for &mut Runner {
    fn into(self) -> CommandContext {
        CommandContext::new(
            Arc::clone(&self.context),
            Arc::clone(&self.state),
            self.from_server_receiver.clone(),
            self.to_server_sender.clone(),
        )
    }
}
