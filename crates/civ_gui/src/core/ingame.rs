use bevy::prelude::*;

use common::{
    network::message::{
        ClientStateMessage, ServerToClientEstablishmentMessage, ServerToClientInGameMessage,
        ServerToClientMessage,
    },
    space::{CityVec2dIndex, UnitVec2dIndex},
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
                    if let Some(ref mut slice) = &mut (game_slice.0) {
                        if let Some(index) = slice
                            .cities_mut()
                            .set(city.geo().point(), Some(city.clone()))
                        {
                            slice
                                .cities_map_mut()
                                .insert(*city.id(), CityVec2dIndex(index));
                        }
                    }
                    commands.trigger(GameSliceUpdated);
                }
                ClientStateMessage::RemoveCity(point, city_id) => {
                    if let Some(ref mut slice) = &mut (game_slice.0) {
                        slice.cities_mut().set(point, None);
                        slice.cities_map_mut().remove(city_id);
                    }
                    commands.trigger(GameSliceUpdated);
                }
                ClientStateMessage::SetUnit(unit) => {
                    if let Some(ref mut slice) = &mut (game_slice.0) {
                        let mut new_index: Option<UnitVec2dIndex> = None;
                        // FIXME BS NOW: this geo is possibly the new one if moved ! Add "previous_point" to SetUnit ?
                        if let Some((index1, units)) = slice.units_mut().get_mut(unit.geo().point())
                        {
                            if let Some(units) = units {
                                if let Some(index2) = units.iter().position(|u| u.id() == unit.id())
                                {
                                    units[index2] = unit.clone();
                                    new_index = Some(UnitVec2dIndex(index1, index2));
                                // Its a new unit
                                } else {
                                    units.push(unit.clone());
                                    new_index = Some(UnitVec2dIndex(index1, 0));
                                }
                            // There is no vector of unit yet here
                            } else {
                                *units = Some(vec![unit.clone()]);
                                new_index = Some(UnitVec2dIndex(index1, 0));
                            }
                            // If None, its outside of the slice
                        }

                        if let Some(new_index) = new_index {
                            slice.units_map_mut().insert(*unit.id(), new_index);
                        }
                    }
                    commands.trigger(GameSliceUpdated);
                }
                ClientStateMessage::RemoveUnit(point, unit_id) => {
                    // FIXME BS NOW: must update units_map
                    if let Some(ref mut slice) = &mut (game_slice.0) {
                        let mut is_empty = false;
                        if let Some((_, Some(units))) = slice.units_mut().get_mut(point) {
                            units.retain(|u| u.id() != unit_id);
                            is_empty = units.is_empty();

                            slice.units_map_mut().remove(unit_id);
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
