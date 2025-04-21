use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Ui},
    EguiContextSettings, EguiContexts,
};
use common::game::nation::flag::Flag;
use derive_more::Constructor;
use strum::IntoEnumIterator;
use uuid::Uuid;

#[cfg(target_arch = "wasm32")]
use crate::utils::cookies::Cookies;
use crate::{
    context::{Context, ContextResource, EntryPoint},
    embedded::{NewLocalGameConfig, StartNewLocalGame},
    network::{JoinServer, NetworkConfig, ServerAddress},
    state::{Client, ServerResource},
};

use super::{Connect, Connecting, GuiStateResource, TakePlace, TakingPlace};

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

#[derive(Debug, Default, Constructor)]
pub struct GuiState {
    screen: GuiScreen,
}

impl From<Context> for GuiState {
    fn from(value: Context) -> Self {
        match value.entry_point() {
            EntryPoint::Root => GuiState::new(GuiScreen::Root(RootState::new())),
            EntryPoint::Server => GuiState::new(GuiScreen::Server(value.into())),
        }
    }
}

#[derive(Debug)]
pub enum GuiScreen {
    Root(RootState),
    Local(LocalState),
    Server(ServerState),
}

impl Default for GuiScreen {
    fn default() -> Self {
        GuiScreen::Root(RootState::default())
    }
}

#[derive(Debug, Default, Constructor)]
pub struct RootState {}

#[derive(Debug, Default, Constructor)]
pub struct LocalState {}

#[derive(Debug, Constructor)]
pub struct ServerState {
    pub address: String,
}

#[derive(Resource, Deref, DerefMut, Default, PartialEq, Hash, Eq)]
pub struct FlagInput(pub Option<Flag>);

impl Display for FlagInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.map(|f| f.to_string()).unwrap_or_default())
    }
}

fn set_scale_factor(mut context_settings: Query<(&mut EguiContextSettings, &Window)>) {
    if let Ok((mut egui_settings, _)) = context_settings.get_single_mut() {
        egui_settings.scale_factor = 2.0;
    }
}

pub fn manage_gui(
    mut commands: Commands,
    context: Res<ContextResource>,
    mut contexts: EguiContexts,
    context_settings: Query<(&mut EguiContextSettings, &Window)>,
    mut gui: ResMut<GuiStateResource>,
    mut server: ResMut<ServerResource>,
    //
    player_id: ResMut<PlayerIdInput>,
    keep_connected: ResMut<KeepConnectedInput>,
    flag: ResMut<FlagInput>,
    _client: Res<Client>,
    mut connecting: ResMut<Connecting>,
    taking_place: Res<TakingPlace>,
) {
    set_scale_factor(context_settings);
    egui::TopBottomPanel::top("menu").show(contexts.ctx_mut(), |ui| {
        let screen = &mut gui.screen;

        if connecting.0 {
            ui.label("Connecting...");
            return;
        }
        if taking_place.0 {
            ui.label("Taking place...");
            return;
        }

        if let Some(event) = match screen {
            GuiScreen::Root(state) => draw_root(ui, state),
            GuiScreen::Local(state) => draw_local(ui, state),
            GuiScreen::Server(state) => draw_server(ui, state, &mut server, &mut commands),
        } {
            match event {
                GuiEvent::Switch(switch) => match switch {
                    Switch::SinglePlayer => {
                        gui.screen = GuiScreen::Local(context.clone().into());
                    }
                    Switch::JoinServer => {
                        gui.screen = GuiScreen::Server(context.clone().into());
                    }
                },
                GuiEvent::StartLocalGame => {
                    commands.trigger(StartNewLocalGame::new(NewLocalGameConfig));
                }
                GuiEvent::Join(address) => {
                    commands.trigger(JoinServer::new(
                        NetworkConfig::builder().server_address(address).build(),
                    ));
                    connecting.0 = true;
                }
            }
        }
    });
}

enum GuiEvent {
    Switch(Switch),
    StartLocalGame,
    Join(ServerAddress),
}

enum Switch {
    SinglePlayer,
    JoinServer,
}

fn draw_root(ui: &mut Ui, _state: &mut RootState) -> Option<GuiEvent> {
    let mut event = None;

    ui.vertical_centered(|ui| {
        if ui.button("Local game").clicked() {
            event = Some(GuiEvent::Switch(Switch::SinglePlayer));
        }
        if ui.button("Join server").clicked() {
            event = Some(GuiEvent::Switch(Switch::JoinServer));
        }
    });

    event
}

fn draw_local(ui: &mut Ui, _state: &mut LocalState) -> Option<GuiEvent> {
    let mut event = None;

    ui.vertical_centered(|ui| {
        if ui.button("Start new game").clicked() {
            event = Some(GuiEvent::StartLocalGame)
        }
    });

    event
}

fn draw_server(
    ui: &mut Ui,
    state: &mut ServerState,
    server: &mut ServerResource,
    commands: &mut Commands,
) -> Option<GuiEvent> {
    let mut event = None;

    ui.vertical_centered(|ui| {
        ui.horizontal_centered(|ui| {
            ui.text_edit_singleline(&mut state.address);
            if ui.button("Join").clicked() {
                event = Some(GuiEvent::Join(ServerAddress::new(state.address.clone())))
            }

            ui.label("PlayerId: ");
            if server.resume.is_some() {
                ui.label(server.player_id.clone());
            } else {
                ui.text_edit_singleline(&mut server.player_id);
                if ui.button("🔄").clicked() {
                    server.player_id = Uuid::new_v4().to_string();
                }
                // Display connect button if server resume has not been received yet
                if ui.button("Connect").clicked() {
                    commands.trigger(Connect)
                }
                ui.checkbox(&mut server.keep_connected, "Keep connected");
            }

            if let Some(resume) = &mut server.resume {
                if let Some(None) = server.flag {
                    ui.horizontal_wrapped(|ui| {
                        egui::ComboBox::from_label("Flag")
                            .selected_text("TODO FLAG")
                            .show_ui(ui, |ui| {
                                for flag_ in Flag::iter() {
                                    if !resume.flags().contains(&flag_) {
                                        ui.selectable_value(
                                            &mut server.flag,
                                            Some(Some(flag_)), // TODO ?
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
        })
    });

    event
}
