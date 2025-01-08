use std::{
    io::{self, Write},
    sync::{Arc, RwLock},
    thread,
};

use bon::Builder;
use clap::Parser;
use common::network::message::{ClientToServerMessage, NotificationLevel, ServerToClientMessage};
use crossbeam::channel::{Receiver, Sender};

use crate::{
    command::{
        self, Command, CommandContext, CommandError, SubCommand, UnitSubCommand, WindowSubCommand,
    },
    context::Context,
    error::PublicError,
    state::State,
};

#[derive(Builder)]
pub struct Runner {
    context: Context,
    state: Arc<RwLock<State>>,
    from_server_receiver: Receiver<ServerToClientMessage>,
    to_server_sender: Sender<ClientToServerMessage>,
}

impl Runner {
    pub fn run(&mut self) {
        // TODO clean
        let from_server_receiver = self.from_server_receiver.clone();
        let state = Arc::clone(&self.state);
        thread::spawn(move || {
            while let Ok(message) = from_server_receiver.recv() {
                let mut state = state.write().expect("Assume state is always accessible");
                match message {
                    ServerToClientMessage::State(message) => {
                        state.apply(message);
                    }
                    ServerToClientMessage::Notification(level, message) => {
                        match level {
                            NotificationLevel::Error => {
                                state.push_error(PublicError::ServerNotification(message));
                            }
                            NotificationLevel::Warning => todo!(),
                            NotificationLevel::Info => todo!(),
                        };
                    }
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
                match self.execute(input) {
                    Ok(stop) => {
                        if stop {
                            break;
                        }
                    }
                    Err(error) => println!("{}", error),
                }
            }
        }

        self.context.require_stop();
    }

    fn execute(&mut self, input: String) -> Result<bool, CommandError> {
        let mut args = vec!["tui".to_string()];
        args.extend(shellwords::split(&input).unwrap());

        match Command::try_parse_from(args) {
            Ok(command) => {
                match command.subcommand {
                    SubCommand::Exit => return Ok(true),
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
                                command::window::set(self.into(), start_x, start_y, end_x, end_y);
                            }
                        };
                    }
                    SubCommand::Cities => command::city::cities(self.into())?,
                    SubCommand::City { id, follow } => {
                        command::city::city(self.into(), id, follow)?
                    }
                    SubCommand::Units => command::unit::units(self.into())?,
                    SubCommand::Unit { id, subcommand } => {
                        match subcommand {
                            Some(command) => match command {
                                UnitSubCommand::Detail { follow } => {
                                    command::unit::detail(self.into(), id, follow)?
                                }
                                UnitSubCommand::Settle { city_name } => {
                                    command::unit::settle(self.into(), id, &city_name)?;
                                }
                            },
                            None => command::unit::detail(self.into(), id, false)?,
                        };
                    }
                };
            }
            Err(error) => {
                println!("{}", error)
            }
        }

        Ok(false)
    }

    fn print_prompt(&mut self) {
        let state = self
            .state
            .read()
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
            self.context.clone(),
            Arc::clone(&self.state),
            self.from_server_receiver.clone(),
            self.to_server_sender.clone(),
        )
    }
}
