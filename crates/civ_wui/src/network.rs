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

#[cfg(not(target_arch = "wasm32"))]
use message_io::node;
#[cfg(not(target_arch = "wasm32"))]
use message_io::node::NodeHandler;
#[cfg(not(target_arch = "wasm32"))]
use message_io::{
    network::{NetEvent, Transport},
    node::NodeEvent,
};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::{Arc, Mutex};
#[cfg(not(target_arch = "wasm32"))]
use std::thread;

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

#[derive(Resource, Deref)]
pub struct NetworkConfigResource(NetworkConfig);

#[derive(Event)]
pub struct ServerMessage(pub ServerToClientMessage);

#[derive(Event)]
pub struct EstablishmentMessage(pub ServerToClientEstablishmentMessage);

#[derive(Event)]
pub struct InGameMessage(pub ServerToClientInGameMessage);

pub const DEFAULT_SERVER_HOST: &str = "127.0.0.1";
#[cfg(target_arch = "wasm32")]
pub const DEFAULT_SERVER_PORT: u16 = 9876;
#[cfg(not(target_arch = "wasm32"))]
pub const DEFAULT_SERVER_PORT: u16 = 9877;

#[derive(Debug, Builder, Clone)]
pub struct NetworkConfig {
    server_host: String,
    server_port: u16,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            server_host: DEFAULT_SERVER_HOST.to_string(),
            server_port: DEFAULT_SERVER_PORT,
        }
    }
}

#[derive(Default, Builder)]
pub struct NetworkPlugin {
    config: NetworkConfig,
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

        app.insert_resource(NetworkConfigResource(self.config.clone()))
            .insert_resource(ServerToClientSenderResource(from_server_sender))
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

enum Signal {
    Connected,
    Disconnected,
    ClientToServerMessageAvailable,
}

#[cfg(not(target_arch = "wasm32"))]
fn setup_network(
    mut _task_runner: AsyncTaskRunner<'_, ()>,
    network_config: Res<NetworkConfigResource>,
    to_server_receiver: Res<ClientToServerReceiverResource>,
    from_server_sender: Res<ServerToClientSenderResource>,
) {
    use std::sync::mpsc::{channel, Receiver as SyncReceiver, Sender as SyncSender};

    use message_io::network::Endpoint;

    let handler: Arc<Mutex<Option<NodeHandler<Signal>>>> = Arc::new(Mutex::new(None));
    let server: Arc<Mutex<Option<Endpoint>>> = Arc::new(Mutex::new(None));

    let handler_ = handler.clone();
    let server_ = server.clone();
    let from_server_sender_ = from_server_sender.0.clone();
    let server_address = format!(
        "{}:{}",
        network_config.server_host, network_config.server_port
    );
    let (bridge_sender, bridge_receiver): (
        SyncSender<ClientToServerMessage>,
        SyncReceiver<ClientToServerMessage>,
    ) = channel();
    thread::spawn(move || {
        let (handler, listener) = node::split();

        let (server, _) = handler
            .network()
            .connect(Transport::FramedTcp, server_address)
            .unwrap();

        *handler_.lock().unwrap() = Some(handler.clone());
        *server_.lock().unwrap() = Some(server);

        listener.for_each(move |event| match event {
            NodeEvent::Network(net_event) => match net_event {
                NetEvent::Connected(_endpoint, _ok) => {
                    //
                    handler.signals().send(Signal::Connected)
                }
                NetEvent::Accepted(_, _) => unreachable!(), // Only generated by listening
                NetEvent::Message(_endpoint, data) => {
                    let message: ServerToClientMessage = bincode::deserialize(data).unwrap();
                    from_server_sender_.send_blocking(message).unwrap();
                }
                NetEvent::Disconnected(_endpoint) => {
                    //
                    handler.signals().send(Signal::Disconnected)
                }
            },
            NodeEvent::Signal(signal) => match signal {
                Signal::Connected => {
                    info!("Connected")
                }
                Signal::Disconnected => {
                    info!("Disconnected")
                }
                Signal::ClientToServerMessageAvailable => {
                    info!("Send message");
                    if let Ok(message) = bridge_receiver.try_recv() {
                        let message = bincode::serialize(&message).unwrap();
                        handler.network().send(server, &message);
                    }
                }
            },
        });
    });

    let to_server_receiver_ = to_server_receiver.0.clone();
    thread::spawn(move || {
        while let Ok(message) = to_server_receiver_.recv_blocking() {
            info!("Pipe message");
            let handler = handler.lock().unwrap();
            let handler = handler.as_ref().unwrap();
            bridge_sender.send(message).unwrap();
            handler
                .signals()
                .send(Signal::ClientToServerMessageAvailable);
        }
    });
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
