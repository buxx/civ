use std::path::PathBuf;
use std::thread;

use async_std::channel::{unbounded, Receiver};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use civ_server::config::ServerConfig;
// TODO: not in wasm32
use civ_server::{bridge::direct::DirectBridgeBuilder, start as start_server, Args as ServerArgs};
use civ_world::config::WorldConfig;
use civ_world::generator::random::RandomGenerator;
use civ_world::writer::FilesWriter;
use civ_world::{self, WorldGeneratorError};
use common::game::nation::flag::Flag;
use common::game::GameFrame;
use common::network::message::ClientToServerEstablishmentMessage;
use common::network::Client;
use common::utils::Progress;
use derive_more::Constructor;
use uuid::Uuid;

use crate::bridge::EmbeddedServerReady;
use crate::menu::state::MenuStateResource;
use crate::to_server;
use crate::utils::gui::window::IntoResolution;
use crate::{
    menu::single::{SingleState, StartSingleEvent},
    utils::app_dir,
};

use super::{
    ClientToServerSenderResource, ServerToClientReceiverResource, StartEmbeddedServer,
    StartEmbeddedServerReceiverResource, WorldGenerationProgressReceiverResource,
};

pub enum SingleConfiguration {
    FromScratch(FromScratchConfig),
    #[allow(unused)]
    LoadFrom(LoadFromConfig),
}

impl SingleConfiguration {
    pub fn from_state(_state: &SingleState) -> Self {
        // TODO: save it somewhere for restore game
        let game_dir = app_dir().unwrap().join(Uuid::new_v4().to_string());
        let snapshot = game_dir.join("snapshot.civ");
        let world = game_dir.join("world");
        Self::FromScratch(FromScratchConfig {
            world: WorldConfig::builder()
                .target(world)
                .width(100)
                .height(100)
                .chunk_size(100)
                .build(), // TODO
            // TODO: specific config ?
            server: ServerConfig::new(
                Some(snapshot),
                GameFrame(120000), // TODO
                "".to_string(),
                "".to_string(),
            ),
        })
    }

    fn snapshot(&self) -> Option<&PathBuf> {
        match self {
            SingleConfiguration::FromScratch(config) => config.server.snapshot(),
            SingleConfiguration::LoadFrom(_config) => todo!(),
        }
    }

    fn snapshot_interval(&self) -> &GameFrame {
        match self {
            SingleConfiguration::FromScratch(config) => config.server.snapshot_interval(),
            SingleConfiguration::LoadFrom(_config) => todo!(),
        }
    }

    fn flag(&self) -> Flag {
        // FIXME
        Flag::Abkhazia
    }

    fn target(&self) -> &PathBuf {
        match self {
            SingleConfiguration::FromScratch(config) => &config.world.target,
            SingleConfiguration::LoadFrom(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, Constructor)]
pub struct FromScratchConfig {
    world: WorldConfig,
    server: ServerConfig,
}

pub struct LoadFromConfig;

pub fn start_single(
    _trigger: Trigger<StartSingleEvent>,
    state: Res<MenuStateResource>,
    mut progress: ResMut<WorldGenerationProgressReceiverResource>,
) {
    info!("Start single ...");
    let conf = SingleConfiguration::from_state(&state.0.single);
    match conf {
        SingleConfiguration::FromScratch(config) => {
            create_single(config.clone(), &mut progress.0);
        }
        SingleConfiguration::LoadFrom(_config) => todo!(),
    };
}

fn create_single(
    config: FromScratchConfig,
    progress: &mut Option<Receiver<Progress<WorldGeneratorError>>>,
) {
    let (progress_sender, progress_receiver) = unbounded();
    *progress = Some(progress_receiver);

    thread::spawn(move || {
        let writer = FilesWriter::new(config.world.target.clone());
        let target = config.world.target.clone();
        let world = config.world.into();
        let _ = civ_world::run()
            .generator(RandomGenerator)
            .target(&target)
            .world(&world)
            .writer(&writer)
            .progress(progress_sender)
            .call();
    });
}

pub fn listen_world_generation_progress(
    mut commands: Commands,
    progress: Res<WorldGenerationProgressReceiverResource>,
    mut state: ResMut<MenuStateResource>,
) {
    if let Some(progress) = &progress.0 {
        if let Ok(progress) = progress.try_recv() {
            match progress {
                Progress::InProgress(value) => {
                    info!("World generation progress {:?} ...", &value);
                    state.0.progress = Some(("Generate world".to_string(), value));
                }
                Progress::Finished => {
                    info!("World generation finished");
                    state.0.progress = None;
                    commands.trigger(StartEmbeddedServer);
                }
                Progress::Error(error) => {
                    // FIXME (gui display this error)
                    info!("World generation error: {}", &error);
                    state.0.progress = None;
                }
            }
        }
    }
}

pub fn start_embedded_server(
    _trigger: Trigger<StartEmbeddedServer>,
    mut to_server_sender: ResMut<ClientToServerSenderResource>,
    mut from_server_receiver: ResMut<ServerToClientReceiverResource>,
    mut progress: ResMut<StartEmbeddedServerReceiverResource>,
    mut state: ResMut<MenuStateResource>,
) {
    let client = Client::default();

    state.0.connecting = true;
    let conf = SingleConfiguration::from_state(&state.0.single);

    let (progress_sender, progress_receiver) = unbounded();
    progress.0 = Some(progress_receiver);

    info!("Start embedded server ...");
    let args = ServerArgs::builder()
        .world(conf.target().clone())
        .maybe_snapshot(conf.snapshot().cloned())
        .snapshot_interval(conf.snapshot_interval().0)
        .tcp_listen_address("".to_string())
        .ws_listen_address("".to_string())
        .build();
    let (client_to_server_sender, client_to_server_receiver) = unbounded();
    let (server_to_client_sender, server_to_client_receiver) = unbounded();
    thread::spawn(move || {
        let bridge =
            DirectBridgeBuilder::new(client, client_to_server_receiver, server_to_client_sender);
        let _ = start_server()
            .args(args)
            .bridge_builder(&bridge)
            .progress(progress_sender)
            .call();
    });

    to_server_sender.0 = client_to_server_sender;
    from_server_receiver.0 = server_to_client_receiver;
}

pub fn listen_start_embedded_server_progress(
    mut commands: Commands,
    progress: Res<StartEmbeddedServerReceiverResource>,
    mut state: ResMut<MenuStateResource>,
) {
    if let Some(progress) = &progress.0 {
        if let Ok(progress) = progress.try_recv() {
            match progress {
                Progress::InProgress(value) => {
                    info!("Start embedded server progress {:?} ...", &value);
                    state.0.progress = Some(("Load world".to_string(), value));
                }
                Progress::Finished => {
                    info!("Start embedded server finished");
                    state.0.progress = None;
                    commands.trigger(EmbeddedServerReady);
                }
                Progress::Error(error) => {
                    // FIXME (gui display this error)
                    info!("Start embedded server error: {}", &error);
                    state.0.progress = None;
                }
            }
        }
    }
}

pub fn join_embedded_server(
    _trigger: Trigger<EmbeddedServerReady>,
    mut commands: Commands,
    state: Res<MenuStateResource>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<&GlobalTransform>,
) {
    let conf = SingleConfiguration::from_state(&state.0.single);
    let Ok(window) = windows.single() else { return };
    let Ok(cam_transform) = cameras.single() else {
        return;
    };
    let resolution = (window, cam_transform).resolution();

    to_server!(
        commands,
        ClientToServerEstablishmentMessage::TakePlace(conf.flag(), resolution)
    );
}
