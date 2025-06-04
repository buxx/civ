use bevy::prelude::*;
use bevy_egui::{EguiContextSettings, EguiContexts};

use super::{DrawUiComponent, EGUI_DISPLAY_FACTOR};

pub mod unit;

#[macro_export]
macro_rules! add_component {
    ($app:expr, $resource:ty, $system:expr, $observer:expr) => {
        $app.init_resource::<$resource>()
            .add_systems(Update, ($system,).run_if(in_state(AppState::InGame)))
            .add_observer($observer)
    };
}

pub trait UiComponentResource<T: DrawUiComponent>: Resource {
    fn component(&self) -> &Option<T>;
    fn component_mut(&mut self) -> &mut Option<T>;
}

#[macro_export]
macro_rules! impl_ui_component_resource {
    ($type:ty, $inner:ty) => {
        impl UiComponentResource<$inner> for $type {
            fn component(&self) -> &Option<$inner> {
                &self.0
            }

            fn component_mut(&mut self) -> &mut Option<$inner> {
                &mut self.0
            }
        }
    };
}

pub fn draw_component<R: UiComponentResource<C>, C: DrawUiComponent>(
    mut commands: Commands,
    mut egui: Query<(&mut EguiContextSettings, &Window)>,
    mut resource: ResMut<R>,
    mut contexts: EguiContexts,
    windows: Query<&Window>,
) {
    let mut disband = false;

    if let Some(component) = resource.component_mut() {
        if let Ok((mut egui_settings, _)) = egui.get_single_mut() {
            egui_settings.scale_factor = EGUI_DISPLAY_FACTOR;
        }

        let window = windows.single();
        disband = component.draw(contexts.ctx_mut(), window, &mut commands);
    }

    if disband {
        *resource.component_mut() = None;
    }
}
