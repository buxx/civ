use bevy::prelude::*;
use common::network::Client as ClientBase;

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

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_sub_state::<InGame>()
            // TODO: client_id from cookie
            .insert_resource(Client::default());
    }
}
