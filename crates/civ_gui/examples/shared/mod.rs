use async_std::channel::{unbounded, Receiver, Sender};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bon::Builder;
#[cfg(feature = "debug")]
use civ_gui::debug::DebugPlugin;
use civ_gui::menu::join::JoinEvent;
use civ_server::game::city::City;
use civ_server::game::unit::Unit;
use civ_server::state::clients::{Clients, PlayerState};
use civ_server::{bridge::direct::DirectBridgeBuilder, start as start_server, Args as ServerArgs};
use civ_world::config::WorldConfig;
use civ_world::generator::Generator;
use civ_world::writer::FilesWriter;
use common::game::nation::flag::Flag;
use common::game::GameFrame;
use common::geo::{GeoVec, ImaginaryWorldPoint};
use common::network::message::ClientToServerMessage;
use common::network::Client;
use common::space::window::{DisplayStep, Window};
use common::space::D2Size;
use common::utils::Progress;
use std::error::Error;
use std::path::PathBuf;
use std::thread;
use uuid::Uuid;

use civ_gui::bridge::{
    BridgeMessage, BridgePlugin, ClientToServerSenderResource, ServerToClientReceiverResource,
};
use civ_gui::context::Context;
use civ_gui::core::CorePlugin;
use civ_gui::ingame::InGamePlugin;
use civ_gui::map::MapPlugin;
use civ_gui::menu::MenuPlugin;
use civ_gui::state::{AppState, ClientIdResource, StatePlugin};
use civ_gui::window::window_plugin;

pub mod world;

#[derive(Builder)]
pub struct Setup<W: Generator> {
    tmp_path: Option<PathBuf>,
    game_path: Option<PathBuf>,
    world_path: Option<PathBuf>,
    world_width: usize,
    world_height: usize,
    chunk_size: usize,
    world_generator: W,
    window_start: ImaginaryWorldPoint,
    window_end: ImaginaryWorldPoint,
    cities: Vec<City>,
    units: Vec<GeoVec<Unit>>,
    client_to_server: Option<(
        Sender<ClientToServerMessage>,
        Receiver<ClientToServerMessage>,
    )>,
    server_to_client: Option<(Sender<BridgeMessage>, Receiver<BridgeMessage>)>,
}

impl<W: Generator> Setup<W> {
    pub fn build_app(self) -> Result<App, Box<dyn Error>> {
        println!("Initialize ...");
        let tmp_path = self.tmp_path.clone().unwrap_or(std::env::temp_dir());
        let game_path = self
            .game_path
            .clone()
            .unwrap_or(tmp_path.join(Uuid::new_v4().to_string()));
        let world_path = self
            .world_path
            .clone()
            .unwrap_or(tmp_path.join(game_path.join("world")));
        let world_config = WorldConfig::new(
            world_path.clone(),
            self.world_width,
            self.world_height,
            self.chunk_size,
        );
        let world_size = D2Size::new(self.world_width, self.world_height);
        let client = Client::default();
        let server_config = ServerArgs::builder()
            .world(world_path.clone())
            .snapshot_interval(0) // As snapshot_path is not set, snapshot_interval will not been used
            .tcp_listen_address("".to_string())
            .ws_listen_address("".to_string())
            .build();
        println!("Game data: {}", game_path.display());

        let (client_to_server_sender, client_to_server_receiver) =
            self.client_to_server.unwrap_or(unbounded());
        let (server_to_client_sender, server_to_client_receiver) =
            self.server_to_client.unwrap_or(unbounded());
        let (progress_sender, progress_receiver) = unbounded();

        // Generate world
        println!("Generate world");
        let writer = FilesWriter::new(world_path.clone());
        let world = world_config.into();
        civ_world::run()
            .generator(self.world_generator)
            .target(&world_path)
            .world(&world)
            .writer(&writer)
            .call()?;

        let window = Window::new(self.window_start, self.window_end, DisplayStep::Close);

        // Start server
        println!("Start server");
        thread::spawn(move || {
            let clients = Clients::new(
                [(
                    *client.player_id(),
                    PlayerState::new(Flag::Abkhazia, window),
                )]
                .into_iter()
                .collect(),
            );
            let state = civ_server::state::State::build_from(
                GameFrame(0),
                world_size,
                clients,
                self.cities,
                self.units,
                &vec![],
            );
            let bridge = DirectBridgeBuilder::new(
                client,
                client_to_server_receiver,
                server_to_client_sender,
            );
            let _ = start_server()
                .args(server_config)
                .state(state)
                .bridge_builder(&bridge)
                .progress(progress_sender)
                .call();
        });

        // Wait server ready
        println!("Wait server is ready ...");
        while let Ok(message) = progress_receiver.recv_blocking() {
            if let Progress::Finished = message {
                break;
            }
        }

        println!("Server ready, start GUI");
        let to_server_sender = ClientToServerSenderResource(client_to_server_sender);
        let from_server_receiver = ServerToClientReceiverResource(server_to_client_receiver);

        let init = move |mut commands: Commands| {
            commands.trigger(JoinEvent(*client.player_id()));
        };

        let mut app = App::new();
        let context = Context::new();
        app.add_plugins((
            DefaultPlugins
                .set(window_plugin())
                .set(ImagePlugin::default_nearest()),
            StatePlugin::builder()
                .init_state(AppState::InGame)
                .client_id(ClientIdResource(*client.client_id()))
                .build(),
            BridgePlugin::builder()
                .to_server_sender(to_server_sender)
                .from_server_receiver(from_server_receiver)
                .build(),
            MenuPlugin::new(context.clone()),
            CorePlugin,
            InGamePlugin::builder().build(),
            MapPlugin,
            FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, init);

        #[cfg(feature = "debug")]
        {
            app.add_plugins(DebugPlugin);
        }

        Ok(app)
    }
}
