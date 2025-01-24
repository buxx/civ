use async_std::channel::{Receiver, Sender};
use bevy::prelude::*;
use bevy_async_task::AsyncTaskRunner;
use common::network::message::{ClientToServerMessage, ServerToClientMessage};
use futures::join;

use std::time::Duration;

use async_wsocket::{
    futures_util::{SinkExt, StreamExt},
    ConnectionMode, Sink, Stream, Url, WsMessage,
};

#[derive(Resource)]
pub struct ClientToServerReceiverResource(pub Receiver<ClientToServerMessage>);

#[derive(Resource)]
pub struct ClientToServerSenderResource(pub Sender<ClientToServerMessage>);

#[derive(Resource)]
pub struct ServerToClientReceiverResource(pub Receiver<ServerToClientMessage>);

#[derive(Resource)]
pub struct ServerToClientSenderResource(pub Sender<ServerToClientMessage>);

pub fn setup_network(
    mut task_runner: AsyncTaskRunner<'_, ()>,
    to_server_receiver: Res<ClientToServerReceiverResource>,
    from_server_sender: Res<ServerToClientSenderResource>,
) {
    task_runner.start(websocket(
        to_server_receiver.0.clone(),
        from_server_sender.0.clone(),
    ));
}

async fn websocket(
    to_server_receiver: Receiver<ClientToServerMessage>,
    from_server_sender: Sender<ServerToClientMessage>,
) {
    info!("Open ws on ws://127.0.0.1:9877");
    let url = Url::parse("ws://127.0.0.1:9877").unwrap();
    let (tx, rx) = async_wsocket::connect(&url, &ConnectionMode::Direct, Duration::from_secs(120))
        .await
        .unwrap();

    // listen_to(tx, to_server_receiver).await;
    // listen_from(rx, from_server_sender).await;

    join!(
        listen_to(tx, to_server_receiver),
        listen_from(rx, from_server_sender)
    );

    // info!("END: {:?}", rx.next().await);
    info!("END");
}

async fn listen_to(mut tx: Sink, to_server_receiver: Receiver<ClientToServerMessage>) {
    while let Ok(message) = to_server_receiver.recv().await {
        // info!("Send");
        let bytes = bincode::serialize(&message).unwrap();
        tx.send(WsMessage::Binary(bytes)).await.unwrap();
    }

    error!("Exit to_server")
}

async fn listen_from(mut rx: Stream, from_server_sender: Sender<ServerToClientMessage>) {
    while let Some(msg) = rx.next().await {
        if let Ok(WsMessage::Binary(bytes)) = msg {
            let message: ServerToClientMessage = bincode::deserialize(&bytes).unwrap();
            // info!("Received {:?}", &message);
            from_server_sender.send(message).await.unwrap();
        }
    }

    error!("Exit listen_from")
}
