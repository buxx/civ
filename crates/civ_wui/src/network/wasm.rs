use crate::{inject::Injection, state::Client};
use async_std::channel::{unbounded, Receiver, Sender};
use async_wsocket::{
    futures_util::{SinkExt, StreamExt},
    ConnectionMode, Sink, Stream, Url, WsMessage,
};
use bevy::prelude::*;
use bevy_async_task::AsyncTaskRunner;
use bon::Builder;
use common::network::message::ClientToServerNetworkMessage;
use common::network::message::{
    ClientToServerMessage, ServerToClientEstablishmentMessage, ServerToClientInGameMessage,
    ServerToClientMessage,
};
use futures::join;
use std::time::Duration;
use web_sys::{window, UrlSearchParams};

use super::{
    ClientToServerReceiverResource, NetworkConfig, NetworkConfigResource,
    ServerToClientSenderResource,
};

pub fn setup_network(
    mut task_runner: AsyncTaskRunner<'_, ()>,
    network_config: Res<NetworkConfigResource>,
    to_server_receiver: Res<ClientToServerReceiverResource>,
    from_server_sender: Res<ServerToClientSenderResource>,
) {
    task_runner.start(websocket_client(
        network_config.0.clone(),
        to_server_receiver.0.clone(),
        from_server_sender.0.clone(),
    ));
}

async fn websocket_client(
    network_config: NetworkConfig,
    to_server_receiver: Receiver<ClientToServerMessage>,
    from_server_sender: Sender<ServerToClientMessage>,
) {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let location = document.location().unwrap();
    let query = location.search().unwrap();
    let params = UrlSearchParams::new_with_str(&query).unwrap();

    let url = Url::parse(&format!(
        "ws://{}:{}",
        &network_config.server_host, &network_config.server_port
    ))
    .unwrap();
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

async fn listen_to(mut tx: Sink, to_server_receiver: Receiver<ClientToServerMessage>) {
    while let Ok(message) = to_server_receiver.recv().await {
        let bytes = bincode::serialize(&message).unwrap();
        tx.send(WsMessage::Binary(bytes)).await.unwrap();
    }

    info!("Websocket server message sender closed")
}

async fn listen_from(mut rx: Stream, from_server_sender: Sender<ServerToClientMessage>) {
    while let Some(msg) = rx.next().await {
        if let Ok(WsMessage::Binary(bytes)) = msg {
            let message: ServerToClientMessage = bincode::deserialize(&bytes).unwrap();
            from_server_sender.send(message).await.unwrap();
        }
    }

    info!("Websocket server message listener closed")
}
