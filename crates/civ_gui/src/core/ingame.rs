use bevy::prelude::*;

use common::network::message::{
    ClientStateMessage, ServerToClientEstablishmentMessage, ServerToClientInGameMessage,
    ServerToClientMessage,
};

use crate::{
    assets::tile::TILE_SIZE,
    bridge::MessageReceivedFromServerEvent,
    core::{establishment::react_server_resume_message, state::react_state_message},
    ingame::{GameFrameResource, GameSliceResource, GameWindowResource},
    menu::state::MenuStateResource,
    state::AppState,
    utils::screen::Isometric,
};

use super::GameWindowUpdated;

// FIXME BS NOW: split into events (macro ?)
// TODO: To improve performances, separate each state message in event to lock ResMut only when needed
pub fn react_server_message(
    trigger: On<MessageReceivedFromServerEvent>,
    mut commands: Commands,
    mut state: ResMut<MenuStateResource>,
    mut frame: ResMut<GameFrameResource>,
    mut game_slice: ResMut<GameSliceResource>,
    mut window: ResMut<GameWindowResource>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !matches!(
        trigger.event(),
        MessageReceivedFromServerEvent(ServerToClientMessage::InGame(
            ServerToClientInGameMessage::State(ClientStateMessage::SetGameFrame(_))
        ))
    ) {
        debug!("Received event from server: {:?}", &trigger.event().0);
    }

    match &trigger.event().0 {
        ServerToClientMessage::Establishment(message) => match message {
            ServerToClientEstablishmentMessage::ServerResume(resume, flag) => {
                react_server_resume_message(resume, flag, &mut state, &mut next_state)
            }
            ServerToClientEstablishmentMessage::TakePlaceRefused(_reason) => {
                todo!()
            }
        },
        ServerToClientMessage::InGame(message) => match message {
            ServerToClientInGameMessage::State(message) => {
                react_state_message(
                    message,
                    &mut game_slice,
                    &mut frame,
                    &mut window,
                    &mut commands,
                );
            }
            ServerToClientInGameMessage::Notification(_level, _) => {}
        },
    }
}

pub fn on_game_window_updated(
    _trigger: On<GameWindowUpdated>,
    window: Res<GameWindowResource>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    if let Some(window) = &window.0 {
        let center = window.center();
        let position = center.iso(TILE_SIZE);
        let Ok(mut camera) = camera.single_mut() else {
            return;
        };
        camera.translation = Vec3::new(position.x, position.y, 0.);
    }
}
