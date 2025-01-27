use bevy::prelude::*;

use common::network::message::{ClientStateMessage, ServerToClientInGameMessage};

use crate::{
    ingame::{GameFrame, GameSlice, GameWindow},
    network::InGameMessage,
};

use super::{GameSliceUpdated, GameWindowUpdated};

// TODO: To improve performances, separate each state message in event to lock ResMut only when needed
pub fn react_ingame(
    trigger: Trigger<InGameMessage>,
    mut commands: Commands,
    mut frame: ResMut<GameFrame>,
    mut game_slice: ResMut<GameSlice>,
    mut window: ResMut<GameWindow>,
) {
    match &trigger.event().0 {
        ServerToClientInGameMessage::State(message) => match message {
            ClientStateMessage::SetGameFrame(frame_) => {
                frame.0 = Some(*frame_);
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
