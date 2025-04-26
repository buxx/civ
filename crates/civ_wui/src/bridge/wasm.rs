use crate::bridge::InternalBridgeMessage;
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
use common::network::ServerAddress;
use futures::join;
use std::time::Duration;
use web_sys::{window, UrlSearchParams};

use super::BridgeMessage;
use super::{ClientToServerReceiverResource, ServerToClientSenderResource};

pub fn connect(
    mut task_runner: AsyncTaskRunner<'_, ()>,
    address: ServerAddress,
    to_server_receiver: Receiver<ClientToServerMessage>,
    from_server_sender: Sender<BridgeMessage>,
) {
    task_runner.start(websocket_client(
        address,
        to_server_receiver.clone(),
        from_server_sender.clone(),
    ));
}

async fn websocket_client(
    address: ServerAddress,
    to_server_receiver: Receiver<ClientToServerMessage>,
    from_server_sender: Sender<BridgeMessage>,
) {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let location = document.location().unwrap();
    let query = location.search().unwrap();
    let params = UrlSearchParams::new_with_str(&query).unwrap();

    let url = Url::parse(&format!("ws://{}", &address)).unwrap();
    info!("Open ws on {}", url);
    let (tx, rx) = async_wsocket::connect(&url, &ConnectionMode::Direct, Duration::from_secs(120))
        .await
        .unwrap();

    // Fake connected
    from_server_sender
        .send(BridgeMessage::Internal(
            InternalBridgeMessage::ConnectionEstablished(address.clone()),
        ))
        .await
        .unwrap();

    join!(
        listen_to_server(tx, to_server_receiver),
        listen_from_server(rx, from_server_sender)
    );

    info!("Close websocket client");
}

async fn listen_to_server(mut tx: Sink, to_server_receiver: Receiver<ClientToServerMessage>) {
    while let Ok(message) = to_server_receiver.recv().await {
        let bytes = bincode::serialize(&message).unwrap();
        tx.send(WsMessage::Binary(bytes)).await.unwrap();
    }

    info!("Websocket server message sender closed")
}

async fn listen_from_server(mut rx: Stream, from_server_sender: Sender<BridgeMessage>) {
    while let Some(msg) = rx.next().await {
        if let Ok(WsMessage::Binary(bytes)) = msg {
            let message: ServerToClientMessage = bincode::deserialize(&bytes).unwrap();
            from_server_sender
                .send(BridgeMessage::Server(message))
                .await
                .unwrap();
        }
    }

    info!("Websocket server message listener closed")
}
