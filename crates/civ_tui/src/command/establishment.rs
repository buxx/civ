use std::str::FromStr;

use common::{
    game::nation::flag::Flag,
    network::message::{
        ClientToServerEstablishmentMessage, ClientToServerGameMessage, ClientToServerMessage,
    },
};

use super::{CommandContext, CommandError, InvalidInputError};

pub fn place(context: CommandContext, input: &str) -> Result<(), CommandError> {
    let flag = Flag::from_str(input).map_err(|_| {
        CommandError::InvalidInput(InvalidInputError::InvalidFlag(input.to_string()))
    })?;

    context
        .to_server_sender
        .send(ClientToServerMessage::Game(
            ClientToServerGameMessage::Establishment(
                ClientToServerEstablishmentMessage::TakePlace(flag),
            ),
        ))
        .unwrap();

    Ok(())
}
