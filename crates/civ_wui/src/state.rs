use bevy::prelude::*;
use common::{
    game::{nation::flag::Flag, server::ServerResume},
    network::Client as ClientBase,
};

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
    Map,
    World,
}

#[derive(Resource, Default, Deref)]
pub struct Client {
    _value: ClientBase,
}

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

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_sub_state::<InGame>()
            .insert_resource(Client::default()) // TODO: player_id from cookie
            .insert_resource(Server::default());
    }
}
