use std::thread;

use bevy::prelude::*;
use civ_server::config::ServerConfig;
use civ_world::config::WorldConfig;
use civ_world::{self};
use common::game::GameFrame;
use uuid::Uuid;

use crate::{
    menu::single::{SingleState, StartSingleEvent},
    utils::app_dir,
};

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
                .width(100)
                .height(100)
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

pub fn start_single(trigger: Trigger<StartSingleEvent>) {
    match &trigger.event().0 {
        SingleConfiguration::FromScratch(config) => {
            create_single(config.clone());
        }
        SingleConfiguration::LoadFrom(_config) => todo!(),
    };
}

fn create_single(config: FromScratchConfig) {
    let world = config.world.clone();
    thread::spawn(move || {
        civ_world::run(world.into());
    });
}
