use bevy::prelude::*;
use bon::Builder;
use common::{
    game::{nation::flag::Flag, server::ServerResume},
    network::Client as ClientBase,
};

use crate::inject::Injection;

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
pub struct Client(pub ClientBase);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct InjectionResource(pub Injection);

#[derive(Resource, Default, Debug)]
pub struct Server {
    resume: Option<ServerResume>,
    flag: Option<Option<Flag>>,
}

impl Server {
    pub fn resume(&self) -> Option<&ServerResume> {
        self.resume.as_ref()
    }

    pub fn flag(&self) -> Option<&Option<Flag>> {
        self.flag.as_ref()
    }

    pub fn set_resume(&mut self, resume: Option<ServerResume>) {
        self.resume = resume;
    }

    pub fn set_flag(&mut self, flag: Option<Option<Flag>>) {
        self.flag = flag;
    }
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
            .insert_resource(Client::default())
            .insert_resource(Server::default());
    }
}
