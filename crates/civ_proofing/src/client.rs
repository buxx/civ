use std::{io, thread};

use common::{
    game::{nation::flag::Flag, PlayerId},
    network::{
        client::NetworkClient,
        message::{
            ClientStateMessage, ClientToServerEstablishmentMessage, ClientToServerGameMessage,
            ClientToServerInGameMessage, ClientToServerMessage, ClientToServerUnitMessage,
            ServerToClientEstablishmentMessage, ServerToClientInGameMessage, ServerToClientMessage,
        },
        ClientId,
    },
};
use crossbeam::channel::{unbounded, Receiver, SendError, Sender};
use strum_macros::{Display, EnumString};
use thiserror::Error;

use crate::{
    context::Context,
    utils::{wait, wait_and_get, TimeoutReached},
};

#[derive(Debug, Clone)]
pub struct Client {
    client_id: ClientId,
    player_id: PlayerId,
    flag: Flag,
    from_server_sender: Sender<ServerToClientMessage>,
    from_server_receiver: Receiver<ServerToClientMessage>,
    to_server_sender: Sender<ClientToServerMessage>,
    to_server_receiver: Receiver<ClientToServerMessage>,
}

impl Client {
    pub fn new(flag: Flag) -> Self {
        let (to_server_sender, to_server_receiver): (
            Sender<ClientToServerMessage>,
            Receiver<ClientToServerMessage>,
        ) = unbounded();
        let (from_server_sender, from_server_receiver): (
            Sender<ServerToClientMessage>,
            Receiver<ServerToClientMessage>,
        ) = unbounded();

        Self {
            client_id: Default::default(),
            player_id: Default::default(),
            flag,
            from_server_sender,
            from_server_receiver,
            to_server_sender,
            to_server_receiver,
        }
    }
}

#[derive(Debug, Error)]
pub enum RunError {
    #[error("Timeout: {0}")]
    Timeout(#[from] TimeoutReached<TimeoutKind>),
    #[error("Error when send through a channel: {0}")]
    Channel(#[from] SendError<ClientToServerMessage>),
    #[error("Error when starting network: {0}")]
    Network(io::Error),
    #[error("Unexpected workflow: {0}")]
    Workflow(WorkflowError),
}

#[derive(Debug, Display, EnumString)]
pub enum TimeoutKind {
    Connect,
    Placed,
    GameSlice,
    City,
}

#[derive(Debug, Display, EnumString)]
pub enum WorkflowError {
    SettleNotFoundInGameSlice,
}

pub fn run(context: Context, client: Client) -> Result<(), RunError> {
    let network = NetworkClient::new(
        client.client_id,
        client.player_id,
        &context.args().address,
        context.stop().clone(),
        context.connected().clone(),
        client.to_server_receiver.clone(),
        client.from_server_sender.clone(),
    )
    .map_err(RunError::Network)?;
    thread::spawn(|| {
        network.run();
    });

    // Wait to be connected to the server
    wait(TimeoutKind::Connect, context.args().connect_timeout, || {
        context.is_connected()
    })?;

    // Send take place request
    client.to_server_sender.send(ClientToServerMessage::Game(
        ClientToServerGameMessage::Establishment(ClientToServerEstablishmentMessage::TakePlace(
            client.flag,
        )),
    ))?;

    // Wait for our game slice
    let game_slice = wait_and_get(
        TimeoutKind::GameSlice,
        context.args().game_slice_timeout,
        || {
            if let Ok(ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
                ClientStateMessage::SetGameSlice(game_slice),
            ))) = client.from_server_receiver.try_recv()
            {
                return Some(game_slice);
            }
            None
        },
    )?;

    let Some(settler) = game_slice.units().first() else {
        return Err(RunError::Workflow(WorkflowError::SettleNotFoundInGameSlice));
    };

    // Settle
    client.to_server_sender.send(ClientToServerMessage::Game(
        ClientToServerGameMessage::InGame(ClientToServerInGameMessage::Unit(
            *settler.id(),
            ClientToServerUnitMessage::Settle("MyCityName".to_string()),
        )),
    ))?;

    // Wait for city
    let city = wait_and_get(TimeoutKind::City, context.args().city_timeout, || {
        if let Ok(ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
            ClientStateMessage::SetCity(city),
        ))) = client.from_server_receiver.try_recv()
        {
            return Some(city);
        }
        None
    })?;

    Ok(())
}
