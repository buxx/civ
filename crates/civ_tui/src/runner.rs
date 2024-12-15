use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
    thread,
};

use bon::Builder;
use clap::Parser;
use common::network::message::{ClientToServerMessage, ServerToClientMessage};
use crossbeam::channel::{Receiver, Sender};

use crate::{
    command::{self, Command, CommandContext, SubCommand, UnitSubCommand, WindowSubCommand},
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
        // FIXME clean
        let from_server_receiver = self.from_server_receiver.clone();
        let state = Arc::clone(&self.state);
        thread::spawn(move || {
            while let Ok(message) = from_server_receiver.recv() {
                match message {
                    ServerToClientMessage::State(message) => state
                        .lock()
                        .expect("Assume state is always accessible")
                        .apply(message),
                }
            }
        });

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
                            SubCommand::Errors => {
                                command::errors::errors(self.into());
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
                            SubCommand::Cities => command::city::cities(self.into()),
                            SubCommand::City { id } => command::city::city(self.into(), id),
                            SubCommand::Units => command::unit::units(self.into()),
                            SubCommand::Unit { id, subcommand } => {
                                match subcommand {
                                    Some(command) => match command {
                                        UnitSubCommand::Detail => {
                                            command::unit::detail(self.into(), id)
                                        }
                                        UnitSubCommand::Settle => todo!(),
                                    },
                                    None => command::unit::detail(self.into(), id),
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
        let state = self
            .state
            .lock()
            .expect("Assume state is always accessible");
        if state.errors().is_empty() {
            print!("---> ");
        } else {
            print!("-!-> ");
        }

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
