use async_std::channel::{unbounded, Receiver, Sender};
use bevy::prelude::*;
use bevy_async_task::AsyncTaskRunner;
use bon::Builder;
#[cfg(target_arch = "wasm32")]
use common::network::message::ClientToServerNetworkMessage;
use common::network::message::{
    ClientToServerMessage, ServerToClientEstablishmentMessage, ServerToClientInGameMessage,
    ServerToClientMessage,
};
#[cfg(target_arch = "wasm32")]
use futures::join;
#[cfg(target_arch = "wasm32")]
use web_sys::{window, UrlSearchParams};

#[cfg(target_arch = "wasm32")]
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
use async_wsocket::{
    futures_util::{SinkExt, StreamExt},
    ConnectionMode, Sink, Stream, Url, WsMessage,
};

#[cfg(target_arch = "wasm32")]
use crate::{inject::Injection, state::Client};

#[cfg(target_arch = "wasm32")]
const SERVER: Option<&str> = std::option_env!("SERVER");

#[allow(dead_code)]
#[derive(Resource)]
pub struct ClientToServerReceiverResource(pub Receiver<ClientToServerMessage>);

#[derive(Resource)]
pub struct ClientToServerSenderResource(pub Sender<ClientToServerMessage>);

#[derive(Resource)]
pub struct ServerToClientReceiverResource(pub Receiver<ServerToClientMessage>);

#[allow(dead_code)]
#[derive(Resource)]
pub struct ServerToClientSenderResource(pub Sender<ServerToClientMessage>);

#[derive(Event)]
pub struct ServerMessage(pub ServerToClientMessage);

#[derive(Event)]
pub struct EstablishmentMessage(pub ServerToClientEstablishmentMessage);

#[derive(Event)]
pub struct InGameMessage(pub ServerToClientInGameMessage);

#[derive(Default, Builder)]
pub struct NetworkPlugin {
    to_server_channels: Option<(
        Sender<ClientToServerMessage>,
        Receiver<ClientToServerMessage>,
    )>,
    from_server_channels: Option<(
        Sender<ServerToClientMessage>,
        Receiver<ServerToClientMessage>,
    )>,
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        let (to_server_sender, to_server_receiver): (
            Sender<ClientToServerMessage>,
            Receiver<ClientToServerMessage>,
        ) = self.to_server_channels.clone().unwrap_or(unbounded());

        let (from_server_sender, from_server_receiver): (
            Sender<ServerToClientMessage>,
            Receiver<ServerToClientMessage>,
        ) = self.from_server_channels.clone().unwrap_or(unbounded());

        app.insert_resource(ServerToClientSenderResource(from_server_sender))
            .insert_resource(ServerToClientReceiverResource(from_server_receiver))
            .insert_resource(ClientToServerSenderResource(to_server_sender))
            .insert_resource(ClientToServerReceiverResource(to_server_receiver))
            .add_systems(Startup, setup_network)
            .add_systems(Update, react_server);
    }
}

#[cfg(target_arch = "wasm32")]
fn setup_network(
    mut task_runner: AsyncTaskRunner<'_, ()>,
    to_server_receiver: Res<ClientToServerReceiverResource>,
    from_server_sender: Res<ServerToClientSenderResource>,
) {
    task_runner.start(websocket_client(
        to_server_receiver.0.clone(),
        from_server_sender.0.clone(),
    ));
}

#[cfg(not(target_arch = "wasm32"))]
fn setup_network(
    mut _task_runner: AsyncTaskRunner<'_, ()>,
    _to_server_receiver: Res<ClientToServerReceiverResource>,
    _from_server_sender: Res<ServerToClientSenderResource>,
) {
    // This is a fake network implemented, for now, to simplify examples
}

#[cfg(target_arch = "wasm32")]
async fn websocket_client(
    to_server_receiver: Receiver<ClientToServerMessage>,
    from_server_sender: Sender<ServerToClientMessage>,
) {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let location = document.location().unwrap();
    let query = location.search().unwrap();
    let params = UrlSearchParams::new_with_str(&query).unwrap();
    let port = params.get("port").unwrap_or("9877".to_string());

    let url = Url::parse(&format!("{}:{}", SERVER.unwrap_or("ws://127.0.0.1"), port)).unwrap();
    info!("Open ws on {}", url);
    let (tx, rx) = async_wsocket::connect(&url, &ConnectionMode::Direct, Duration::from_secs(120))
        .await
        .unwrap();

    join!(
        listen_to(tx, to_server_receiver),
        listen_from(rx, from_server_sender)
    );

    info!("Close websocket client");
}

#[cfg(target_arch = "wasm32")]
async fn listen_to(mut tx: Sink, to_server_receiver: Receiver<ClientToServerMessage>) {
    while let Ok(message) = to_server_receiver.recv().await {
        let bytes = bincode::serialize(&message).unwrap();
        tx.send(WsMessage::Binary(bytes)).await.unwrap();
    }

    info!("Websocket server message sender closed")
}

#[cfg(target_arch = "wasm32")]
async fn listen_from(mut rx: Stream, from_server_sender: Sender<ServerToClientMessage>) {
    while let Some(msg) = rx.next().await {
        if let Ok(WsMessage::Binary(bytes)) = msg {
            let message: ServerToClientMessage = bincode::deserialize(&bytes).unwrap();
            from_server_sender.send(message).await.unwrap();
        }
    }

    info!("Websocket server message listener closed")
}

fn react_server(mut commands: Commands, receiver: Res<ServerToClientReceiverResource>) {
    while let Ok(message) = receiver.0.try_recv() {
        commands.trigger(ServerMessage(message));
    }
}
