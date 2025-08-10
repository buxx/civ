use bevy::prelude::*;

use common::network::message::{
    ClientStateMessage, ServerToClientEstablishmentMessage, ServerToClientInGameMessage,
    ServerToClientMessage,
};
use hexx::hex;

use crate::{
    assets::tile::absolute_layout,
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
        ServerToClientMessage::InGame(message) => match message {
            ServerToClientInGameMessage::State(message) => match message {
                ClientStateMessage::SetGameFrame(frame_) => {
                    frame.0 = Some(*frame_);
                    commands.trigger(GameFrameUpdated(*frame_));
                }
                ClientStateMessage::SetGameSlice(game_slice_) => {
                    game_slice.0 = Some(game_slice_.clone());
                    commands.trigger(GameSliceUpdated);
                }
                ClientStateMessage::SetWindow(window_) => {
                    window.0 = Some(*window_);
                    commands.trigger(GameWindowUpdated);
                }
                ClientStateMessage::SetCity(city) => {
                    // FIXME BS NOW: must update cities_map (can be a new city!)
                    if let Some(ref mut slice) = &mut (game_slice.0) {
                        slice
                            .cities_mut()
                            .set(city.geo().point(), Some(city.clone()));
                    }
                    commands.trigger(GameSliceUpdated);
                }
                ClientStateMessage::RemoveCity(point, _) => {
                    // FIXME BS NOW: must update cities_map
                    if let Some(ref mut slice) = &mut (game_slice.0) {
                        slice.cities_mut().set(point, None);
                    }
                    commands.trigger(GameSliceUpdated);
                }
                ClientStateMessage::SetUnit(unit) => {
                    // FIXME BS NOW: must update units_map (can be a new unit)
                    if let Some(ref mut slice) = &mut (game_slice.0) {
                        // FIXME BS NOW: this geo is possibly the new one if moved ! Add "previous_point" to SetUnit ?
                        if let Some(Some(units)) = slice.units_mut().get_mut(unit.geo().point()) {
                            if let Some(index) = units.iter().position(|u| u.id() == unit.id()) {
                                units[index] = unit.clone();
                            }
                        }
                    }
                    commands.trigger(GameSliceUpdated);
                }
                ClientStateMessage::RemoveUnit(point, unit_id) => {
                    // FIXME BS NOW: must update units_map
                    if let Some(ref mut slice) = &mut (game_slice.0) {
                        let mut is_empty = false;
                        if let Some(Some(units)) = slice.units_mut().get_mut(point) {
                            units.retain(|u| u.id() != unit_id);
                            is_empty = units.is_empty();
                        }

                        if is_empty {
                            slice.units_mut().set(point, None);
                        }
                    }
                    commands.trigger(GameSliceUpdated);
                }
            },
            ServerToClientInGameMessage::Notification(_level, _) => {}
        },
    }
}

pub fn on_game_window_updated(
    _trigger: Trigger<GameWindowUpdated>,
    window: Res<GameWindowResource>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    if let Some(window) = &window.0 {
        let center = window.center();
        let position = absolute_layout().hex_to_world_pos(hex(center.x as i32, center.y as i32));
        camera.single_mut().translation = Vec3::new(position.x, position.y, 0.);
    }
}
