use bevy::prelude::*;
use bon::Builder;
use common::network::ClientId;

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

#[derive(Resource, Default, Deref, DerefMut, Clone)]
pub struct ClientIdResource(pub ClientId);

#[derive(Builder)]
pub struct StatePlugin {
    init_state: Option<AppState>,
    client_id: Option<ClientIdResource>,
}

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(self.init_state.clone().unwrap_or_default())
            .add_sub_state::<InGame>()
            .insert_resource(
                self.client_id
                    .as_ref()
                    .unwrap_or(&ClientIdResource::default())
                    .clone(),
            );
    }
}
