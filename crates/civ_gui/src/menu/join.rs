use bevy::prelude::*;
use bevy_egui::egui::{self, Ui};
use common::{
    game::{nation::flag::Flag, server::ServerResume, PlayerId},
    network::ServerAddress,
};
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{
    context::Context,
    user::{SetKeepConnectedEvent, SetPlayerIdEvent},
};

#[derive(Event)]
pub struct ConnectEvent(pub ServerAddress);

#[derive(Event)]
pub struct JoinEvent(pub PlayerId);

#[derive(Event)]
pub struct TakePlaceEvent(pub Flag);

#[derive(Debug)]
pub struct JoinState {
    pub address: ServerAddress,
    pub connected: bool,
    pub player_id: String,
    pub resume: Option<ServerResume>,
    pub flag: Option<Flag>,
    pub keep_connected: bool,
}

impl JoinState {
    pub fn from_context(context: &Context) -> Self {
        Self {
            address: context.default_server_address(),
            connected: Default::default(),
            // FIXME BS NOW: when switch on Join screen, must be updated with Preferences
            player_id: Default::default(),
            resume: Default::default(),
            flag: Default::default(),
            // FIXME BS NOW: when switch on Join screen, must be updated with Preferences
            keep_connected: Default::default(),
        }
    }
}

pub fn draw(ui: &mut Ui, state: &mut JoinState, mut commands: Commands) {
    ui.vertical_centered(|ui| {
        if !state.connected {
            ui.text_edit_singleline(&mut state.address.0);

            if ui.button("Connect").clicked() {
                commands.trigger(ConnectEvent(state.address.clone()));
            }
        } else {
            if state.resume.is_some() {
                ui.label("PlayerId");
                ui.label(state.player_id.clone());
            } else {
                ui.label("PlayerId");
                ui.text_edit_singleline(&mut state.player_id);
                if ui.button("🔄").clicked() {
                    state.player_id = Uuid::new_v4().to_string();
                    commands.trigger(SetPlayerIdEvent(
                        ServerAddress(state.address.0.clone()),
                        PlayerId(Uuid::parse_str(&state.player_id).unwrap()),
                    ));
                }
                if ui.button("Connect").clicked() {
                    // FIXME BS NOW: trigger must modify ClientResource (used by bridge)
                    commands.trigger(JoinEvent(PlayerId(
                        Uuid::parse_str(&state.player_id).unwrap(),
                    )));
                }
                if ui
                    .checkbox(&mut state.keep_connected, "Keep connected")
                    .changed()
                {
                    commands.trigger(SetKeepConnectedEvent(
                        ServerAddress(state.address.0.clone()),
                        state.keep_connected,
                    ));
                };
            }

            if let Some(resume) = &state.resume {
                ui.horizontal_wrapped(|ui| {
                    egui::ComboBox::from_label("Flag")
                        .selected_text(state.flag.map(|f| f.to_string()).unwrap_or("".to_string()))
                        .show_ui(ui, |ui| {
                            for flag_ in Flag::iter() {
                                if !resume.flags().contains(&flag_) {
                                    ui.selectable_value(
                                        &mut state.flag,
                                        Some(flag_),
                                        flag_.to_string(),
                                    );
                                }
                            }
                        });

                    if ui.button("Join").clicked() {
                        commands.trigger(TakePlaceEvent(state.flag.unwrap()));
                    }
                });
            }
        }
    });
}

// pub fn connect(
//     trigger: Trigger<ConnectEvent>,
//     commands: Commands,
//     mut state: ResMut<MenuStateResource>,
//     // to_server_sender: Res<ClientToServerSenderResource>,
//     // player_id_input: Res<PlayerIdInput>,
//     // keep_connected_input: Res<KeepConnectedInput>,
//     mut client: ResMut<ClientResource>,
//     // mut connecting: ResMut<ConnectingResource>,
//     // mut bridge: ResMut<BridgeResource>,
// ) {
//     let address = trigger.event().0;
//     state.0.connecting = true;
//     commands.trigger(SendMessageToServerEvent(
//         ClientToServerNetworkMessage::Hello(
//             client.0,
//             // FIXME
//             Resolution::new(1, 1),
//         )
//         .into(),
//     ));
// }

// pub fn take_place(
//     trigger: Trigger<TakePlaceEvent>,
//     commands: Commands,
//     mut state: ResMut<MenuStateResource>,
//     mut client: ResMut<ClientResource>,
// ) {
//     todo!()
// }
