use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContextSettings, EguiContexts,
};

use super::{
    join, root, single,
    state::{MenuStateResource, Screen},
};

fn set_scale_factor(mut settings: Query<(&mut EguiContextSettings, &Window)>) {
    if let Ok((mut settings, _)) = settings.single_mut() {
        settings.scale_factor = 2.0;
    }
}

pub fn gui(
    commands: Commands,
    mut contexts: EguiContexts,
    egui: Query<(&mut EguiContextSettings, &Window)>,
    mut state: ResMut<MenuStateResource>,
) -> Result {
    set_scale_factor(egui);

    egui::TopBottomPanel::top("menu").show(contexts.ctx_mut()?, |ui| {
        if let Some((label, progress)) = &state.progress {
            ui.label(format!("{} {}%", label, (progress * 100.) as isize));
            return;
        }
        if state.connecting {
            ui.label("Connecting...");
            return;
        }
        if state.taking_place {
            ui.label("Taking place...");
            return;
        }

        match &mut state.screen {
            Screen::Root => root::draw(ui, &mut state.root, commands),
            Screen::Single => single::draw(ui, &mut state.single, commands),
            Screen::Join => join::draw(ui, &mut state.join, commands),
        }
    });

    Ok(())
}
