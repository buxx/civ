use std::thread;

use async_std::channel::{unbounded, Receiver};
use bevy::prelude::*;
use civ_server::config::ServerConfig;
use civ_world::config::WorldConfig;
use civ_world::{self, WorldGeneratorError};
use common::game::GameFrame;
use common::utils::Progress;
use uuid::Uuid;

use crate::menu::state::MenuStateResource;
use crate::{
    menu::single::{SingleState, StartSingleEvent},
    utils::app_dir,
};

use super::{WorldGenerated, WorldGenerationProgressReceiverResource};

pub enum SingleConfiguration {
    FromScratch(FromScratchConfig),
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
                .width(500)
                .height(500)
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
}

#[derive(Debug, Clone)]
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
    let world = config.world.clone();
    let (progress_sender, progress_receiver) = unbounded();
    *progress = Some(progress_receiver);

    thread::spawn(move || {
        let _ = civ_world::run()
            .args(world.into())
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
                    state.0.progress = Some(value);
                }
                Progress::Finished => {
                    info!("World generation finished");
                    state.0.progress = None;
                    commands.trigger(WorldGenerated);
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

pub fn listen_world_generated(_trigger: Trigger<WorldGenerated>, state: Res<MenuStateResource>) {
    let conf = SingleConfiguration::from_state(&state.0.single);
    todo!();
}
