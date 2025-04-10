use std::fmt::Display;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContextSettings, EguiContexts};
use common::game::nation::flag::Flag;
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::state::{Client, Server};
#[cfg(target_arch = "wasm32")]
use crate::utils::cookies::Cookies;

use super::{Connect, Connecting, TakePlace, TakingPlace};

#[derive(Resource, Deref, Default)]
pub struct PlayerIdInput(pub String);

impl PlayerIdInput {
    #[cfg(target_arch = "wasm32")]
    pub fn new(value: String) -> Self {
        Self(value)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_cookies() -> Self {
        Self(
            Cookies
                .get_player_id()
                .and_then(|i| Ok(i.and_then(|i| Some(i.to_string()))))
                .unwrap_or(Some("".to_string()))
                .unwrap_or("".to_string()),
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_cookies() -> Self {
        Self("".to_string())
    }
}
#[derive(Resource, Deref, Default)]
pub struct KeepConnectedInput(pub bool);

impl KeepConnectedInput {
    #[cfg(target_arch = "wasm32")]
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_cookies() -> Self {
        Self(
            Cookies
                .get_keep_connected()
                .and_then(|i| Ok(i))
                .unwrap_or(Some(false))
                .unwrap_or(false),
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_cookies() -> Self {
        Self(false)
    }
}

#[derive(Resource, Deref, DerefMut, Default, PartialEq, Hash, Eq)]
pub struct FlagInput(pub Option<Flag>);

impl Display for FlagInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.map(|f| f.to_string()).unwrap_or_default())
    }
}

pub fn manage_gui(
    commands: Commands,
    contexts: EguiContexts,
    context_settings: Query<(&mut EguiContextSettings, &Window)>,
    player_id: ResMut<PlayerIdInput>,
    keep_connected: ResMut<KeepConnectedInput>,
    flag: ResMut<FlagInput>,
    _client: Res<Client>,
    server: Res<Server>,
    connecting: Res<Connecting>,
    taking_place: Res<TakingPlace>,
) {
    set_scale_factor(context_settings);
    draw_window(
        commands,
        contexts,
        server,
        connecting,
        taking_place,
        player_id,
        keep_connected,
        flag,
    );
}

fn set_scale_factor(mut context_settings: Query<(&mut EguiContextSettings, &Window)>) {
    if let Ok((mut egui_settings, _)) = context_settings.get_single_mut() {
        egui_settings.scale_factor = 2.0;
    }
}

fn draw_window(
    mut commands: Commands,
    mut contexts: EguiContexts,
    server: Res<Server>,
    connecting: Res<Connecting>,
    taking_place: Res<TakingPlace>,
    mut player_id: ResMut<PlayerIdInput>,
    mut keep_connected: ResMut<KeepConnectedInput>,
    mut flag: ResMut<FlagInput>,
) {
    egui::TopBottomPanel::top("menu").show(contexts.ctx_mut(), |ui| {
        if connecting.0 {
            ui.label("Connecting...");
            return;
        }
        if taking_place.0 {
            ui.label("Taking place...");
            return;
        }

        ui.horizontal_wrapped(|ui| {
            ui.label("PlayerId: ");
            if server.resume().is_some() {
                ui.label(player_id.0.clone());
            } else {
                ui.text_edit_singleline(&mut player_id.0);
                if ui.button("🔄").clicked() {
                    player_id.0 = Uuid::new_v4().to_string();
                }
                // Display connect button if server resume has not been received yet
                if ui.button("Connect").clicked() {
                    commands.trigger(Connect)
                }
                ui.checkbox(&mut keep_connected.0, "Keep connected");
            }
        });

        if let Some(resume) = server.resume() {
            if let Some(None) = server.flag() {
                ui.horizontal_wrapped(|ui| {
                    egui::ComboBox::from_label("Flag")
                        .selected_text(flag.to_string())
                        .show_ui(ui, |ui| {
                            for flag_ in Flag::iter() {
                                if !resume.flags().contains(&flag_) {
                                    ui.selectable_value(
                                        &mut flag.0,
                                        Some(flag_),
                                        flag_.to_string(),
                                    );
                                }
                            }
                        });

                    if ui.button("Join").clicked() {
                        commands.trigger(TakePlace)
                    }
                });
            }
        }
    });
}
