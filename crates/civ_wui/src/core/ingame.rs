use bevy::prelude::*;

use common::network::message::{ClientStateMessage, ServerToClientInGameMessage};

use crate::{
    ingame::{GameFrame, GameSlice, GameWindow},
    network::InGameMessage,
};

// TODO: To improve performances, separate each state message in event to lock ResMut only when needed
pub fn react_ingame(
    trigger: Trigger<InGameMessage>,
    mut frame: ResMut<GameFrame>,
    mut game_slice: ResMut<GameSlice>,
    mut window: ResMut<GameWindow>,
) {
    match &trigger.event().0 {
        ServerToClientInGameMessage::State(message) => match message {
            ClientStateMessage::SetGameFrame(frame_) => {
                //
                frame.0 = Some(*frame_)
            }
            ClientStateMessage::SetGameSlice(game_slice_) => {
                //
                game_slice.0 = Some(game_slice_.clone())
            }
            ClientStateMessage::SetWindow(window_) => {
                //
                window.0 = Some(window_.clone())
            }
            ClientStateMessage::SetCity(city) => {
                //
                if let Some(ref mut slice) = &mut (game_slice.0) {
                    slice.cities_mut().retain(|c| c.id() != city.id());
                    slice.cities_mut().push(city.clone());
                }
            }
            ClientStateMessage::RemoveCity(city_id) => {
                //
                if let Some(ref mut slice) = &mut (game_slice.0) {
                    slice.cities_mut().retain(|c| c.id() != city_id);
                }
            }
            ClientStateMessage::SetUnit(unit) => {
                //
                if let Some(ref mut slice) = &mut (game_slice.0) {
                    slice.units_mut().retain(|u| u.id() != unit.id());
                    slice.units_mut().push(unit.clone());
                }
            }
            ClientStateMessage::RemoveUnit(unit_id) => {
                //
                if let Some(ref mut slice) = &mut (game_slice.0) {
                    slice.units_mut().retain(|u| u.id() != unit_id);
                }
            }
        },
        ServerToClientInGameMessage::Notification(_level, _) => {}
    }
}
