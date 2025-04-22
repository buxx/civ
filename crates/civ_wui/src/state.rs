use bevy::prelude::*;
use bon::Builder;
use common::{
    game::{nation::flag::Flag, server::ServerResume},
    network::Client as ClientBase,
};

use crate::{inject::Injection, network::ServerAddress};

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
pub enum InGame {
    #[default]
    Close,
    // Map,
    // World,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct ClientResource(pub ClientBase);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct InjectionResource(pub Injection);

#[derive(Resource, Default, Debug)]
pub struct ServerResource {
    pub connected: Option<ServerAddress>,
    pub resume: Option<ServerResume>,
    pub flag: Option<Option<Flag>>,
    pub player_id: String,
    pub keep_connected: bool,
}

#[derive(Builder)]
pub struct StatePlugin {
    init_state: Option<AppState>,
    #[builder(default)]
    injection: Injection,
}

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(self.init_state.clone().unwrap_or_default())
            .add_sub_state::<InGame>()
            .insert_resource(self.injection.clone())
            .insert_resource(ClientResource::default())
            .insert_resource(ServerResource::default());
    }
}
