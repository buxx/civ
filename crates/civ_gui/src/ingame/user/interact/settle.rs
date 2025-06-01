use bevy::prelude::*;
use bevy_egui::{EguiContextSettings, EguiContexts};
use common::game::unit::UnitId;
use derive_more::Constructor;

use crate::{
    ingame::{DrawUiComponent, EGUI_DISPLAY_FACTOR},
    utils::gui::layout::fixed_window,
};

#[derive(Debug, Event, Deref)]
pub struct SetupSettleCityName(pub UnitId);

#[derive(Debug, Event)]
pub struct SetupSettle(pub UnitId, pub String);

#[derive(Debug, Constructor)]
pub struct SettleCityName {
    unit_id: UnitId,
    name: String,
}

#[derive(Debug, Resource, Default)]
pub struct SettleCityNameResource(pub Option<SettleCityName>);

pub fn on_setup_settle_city_name(
    trigger: Trigger<SetupSettleCityName>,
    mut modal: ResMut<SettleCityNameResource>,
) {
    let event = trigger.event();
    modal.0 = Some(SettleCityName::new(event.0, String::new()));
}

pub fn draw_settle_city_name(
    mut commands: Commands,
    mut egui: Query<(&mut EguiContextSettings, &Window)>,
    mut modal: ResMut<SettleCityNameResource>,
    mut contexts: EguiContexts,
    windows: Query<&Window>,
) {
    let mut disband = false;

    if let Some(modal) = &mut modal.0 {
        if let Ok((mut egui_settings, _)) = egui.get_single_mut() {
            egui_settings.scale_factor = EGUI_DISPLAY_FACTOR;
        }

        let window = windows.single();
        disband = modal.draw(contexts.ctx_mut(), window, &mut commands);
    }

    if disband {
        modal.0 = None;
    }
}

impl DrawUiComponent for SettleCityName {
    fn draw(
        &mut self,
        ctx: &bevy_egui::egui::Context,
        window: &Window,
        commands: &mut Commands,
    ) -> bool {
        let mut close = false;

        fixed_window()
            .ctx(ctx)
            .window(window)
            .factor(EGUI_DISPLAY_FACTOR)
            .ui(|ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.text_edit_singleline(&mut self.name);
                    if ui.button("Ok").clicked() {
                        close = true;
                        commands.trigger(SetupSettle(self.unit_id, self.name.clone()));
                    }
                });
            })
            .call();

        close
    }
}

pub fn on_setup_settle(trigger: Trigger<SetupSettle>) {
    let event = trigger.event();
    println!("setup settle: {event:?}");
}
