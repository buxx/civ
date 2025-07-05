use bevy::prelude::*;

use common::network::message::{
    ClientStateMessage, ServerToClientEstablishmentMessage, ServerToClientInGameMessage,
    ServerToClientMessage,
};
use hexx::hex;

use crate::{
    assets::tile::{absolute_layout, TILE_SIZE},
    bridge::MessageReceivedFromServerEvent,
    ingame::{GameFrameResource, GameFrameUpdated, GameSliceResource, GameWindowResource},
    menu::state::MenuStateResource,
    state::AppState,
};

use super::{GameSliceUpdated, GameWindowUpdated};

// FIXME BS NOW: split into events (macro ?)
// TODO: To improve performances, separate each state message in event to lock ResMut only when needed
pub fn react_server_message(
    trigger: Trigger<MessageReceivedFromServerEvent>,
    mut commands: Commands,
    mut state: ResMut<MenuStateResource>,
    mut frame: ResMut<GameFrameResource>,
    mut game_slice: ResMut<GameSliceResource>,
    mut window: ResMut<GameWindowResource>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    match &trigger.event().0 {
        ServerToClientMessage::Establishment(message) => match message {
            ServerToClientEstablishmentMessage::ServerResume(resume, flag) => {
                state.join.resume = Some(resume.clone());
                state.join.flag = *flag;

                if flag.is_some() {
                    next_state.set(AppState::InGame);
                }
            }
            ServerToClientEstablishmentMessage::TakePlaceRefused(_reason) => {
                todo!()
            }
        },
        ServerToClientMessage::InGame(message) => {
            match message {
                ServerToClientInGameMessage::State(message) => match message {
                    ClientStateMessage::SetGameFrame(frame_) => {
                        frame.0 = Some(*frame_);
                        commands.trigger(GameFrameUpdated(*frame_));
                    }
                    // FIXME BS NOW: when first received, we must set camera translation
                    ClientStateMessage::SetGameSlice(game_slice_) => {
                        game_slice.0 = Some(game_slice_.clone());
                        commands.trigger(GameSliceUpdated);
                    }
                    ClientStateMessage::SetWindow(window_) => {
                        window.0 = Some(window_.clone());
                        commands.trigger(GameWindowUpdated);
                    }
                    ClientStateMessage::SetCity(city) => {
                        if let Some(ref mut slice) = &mut (game_slice.0) {
                            slice.cities_mut().retain(|c| c.id() != city.id());
                            slice.cities_mut().push(city.clone());
                        }
                        commands.trigger(GameSliceUpdated);
                    }
                    ClientStateMessage::RemoveCity(city_id) => {
                        if let Some(ref mut slice) = &mut (game_slice.0) {
                            slice.cities_mut().retain(|c| c.id() != city_id);
                        }
                        commands.trigger(GameSliceUpdated);
                    }
                    ClientStateMessage::SetUnit(unit) => {
                        if let Some(ref mut slice) = &mut (game_slice.0) {
                            slice.units_mut().retain(|u| u.id() != unit.id());
                            slice.units_mut().push(unit.clone());
                        }
                        commands.trigger(GameSliceUpdated);
                    }
                    ClientStateMessage::RemoveUnit(unit_id) => {
                        if let Some(ref mut slice) = &mut (game_slice.0) {
                            slice.units_mut().retain(|u| u.id() != unit_id);
                        }
                        commands.trigger(GameSliceUpdated);
                    }
                },
                ServerToClientInGameMessage::Notification(_level, _) => {}
            }
        }
    }
}

pub fn on_game_window_updated(
    trigger: Trigger<GameWindowUpdated>,
    window: Res<GameWindowResource>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    if let Some(window) = &window.0 {
        let center = window.center();
        let position = absolute_layout().hex_to_world_pos(hex(center.x as i32, center.y as i32));
        camera.single_mut().translation = Vec3::new(position.x, position.y, 0.);
    }
}
