use super::{Bridge, BridgeBuildError, BridgeBuilder};
use async_std::channel::{unbounded, Receiver, Sender};
use common::network::message::{ClientToServerMessage, ServerToClientMessage};
use common::network::{Client, ClientId};
use derive_more::Constructor;
use std::sync::{Arc, RwLock};
use std::thread;

use crate::config::ServerConfig;
use crate::context::Context;
use crate::state::State;

#[derive(Debug, Constructor)]
pub struct DirectBridgeBuilder<T: Send + From<ServerToClientMessage> + 'static> {
    client: Client,
    client_to_server_receiver: Receiver<ClientToServerMessage>,
    server_to_client_sender: Sender<T>,
}

impl<T: Send + From<ServerToClientMessage> + 'static> BridgeBuilder<DirectBridge<T>>
    for DirectBridgeBuilder<T>
{
    fn build(
        &self,
        _context: Context,
        _state: Arc<RwLock<State>>,
        _config: &ServerConfig,
    ) -> Result<
        (
            DirectBridge<T>,
            Receiver<(Client, ClientToServerMessage)>,
            Sender<(ClientId, ServerToClientMessage)>,
        ),
        BridgeBuildError,
    > {
        let (client_to_server_sender_proxy, client_to_server_receiver_proxy) = unbounded();
        let (server_to_client_sender_proxy, server_to_client_receiver_proxy) = unbounded();
        let bridge = DirectBridge::new(
            self.client,
            self.client_to_server_receiver.clone(),
            client_to_server_sender_proxy,
            server_to_client_receiver_proxy,
            self.server_to_client_sender.clone(),
        );

        Ok((
            bridge,
            client_to_server_receiver_proxy,
            server_to_client_sender_proxy,
        ))
    }
}

#[derive(Debug, Constructor)]
pub struct DirectBridge<T: Send + From<ServerToClientMessage> + 'static> {
    client: Client,
    client_to_server_receiver: Receiver<ClientToServerMessage>,
    client_to_server_sender_proxy: Sender<(Client, ClientToServerMessage)>,
    server_to_client_receiver_proxy: Receiver<(ClientId, ServerToClientMessage)>,
    server_to_client_sender: Sender<T>,
}

impl<T: Send + From<ServerToClientMessage> + 'static> Bridge for DirectBridge<T> {
    fn run(&mut self) {
        let client_to_server_receiver_ = self.client_to_server_receiver.clone();
        let client_to_server_sender_proxy_ = self.client_to_server_sender_proxy.clone();
        let client_ = self.client;
        let t1 = thread::spawn(move || {
            while let Ok(message) = client_to_server_receiver_.recv_blocking() {
                client_to_server_sender_proxy_
                    .send_blocking((client_, message))
                    .unwrap();
            }
        });

        let server_to_client_receiver_proxy_ = self.server_to_client_receiver_proxy.clone();
        let server_to_client_sender_ = self.server_to_client_sender.clone();
        let t2 = thread::spawn(move || {
            while let Ok(message) = server_to_client_receiver_proxy_.recv_blocking() {
                server_to_client_sender_
                    .send_blocking(message.1.into())
                    .unwrap();
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();
    }
}
