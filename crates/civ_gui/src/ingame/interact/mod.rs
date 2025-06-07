use bevy::prelude::*;
use bevy_egui::{EguiContextSettings, EguiContexts};
use common::game::{slice::ClientUnit, unit::UnitId};

use crate::{core::GameSlicePropagated, ingame::GameSliceResource};

use super::{DrawUiComponent, EGUI_DISPLAY_FACTOR};

pub mod unit;

#[macro_export]
macro_rules! add_component {
    ($app:expr, $resource:ty) => {
        $app.init_resource::<$resource>()
            .add_systems(
                Update,
                ($crate::ingame::interact::draw_component::<$resource>,)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_observer(
                $crate::ingame::interact::on_event::<
                    <$resource as $crate::ingame::interact::UiComponentResource>::OnEvent,
                    $resource,
                    <$resource as $crate::ingame::interact::UiComponentResource>::Resource,
                >,
            )
            .add_observer(
                $crate::ingame::interact::on_slice_propagated::<
                    $resource,
                    <$resource as $crate::ingame::interact::UiComponentResource>::Resource,
                >,
            )
    };
}

pub trait UiComponentResource: Resource {
    type Resource: DrawUiComponent;
    type OnEvent: Event;

    fn component(&self) -> &Option<Self::Resource>;
    fn component_mut(&mut self) -> &mut Option<Self::Resource>;
}

#[macro_export]
macro_rules! impl_ui_component_resource {
    ($type:ty, $inner:ty, $event:ty) => {
        impl $crate::ingame::interact::UiComponentResource for $type {
            type Resource = $inner;
            type OnEvent = $event;

            fn component(&self) -> &Option<$inner> {
                &self.0
            }

            fn component_mut(&mut self) -> &mut Option<$inner> {
                &mut self.0
            }
        }

        impl $crate::ingame::interact::Set<Option<$inner>> for $type {
            fn set(&mut self, value: Option<$inner>) {
                self.0 = value;
            }
        }

        impl $crate::ingame::interact::Get<Option<$inner>> for $type {
            fn get(&self) -> &Option<$inner> {
                &self.0
            }
        }
    };
}

pub fn draw_component<R: UiComponentResource>(
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

pub trait WithUnitId {
    fn unit_id(&self) -> &UnitId;
}

// TODO: move in more generic mod
pub trait Set<T> {
    fn set(&mut self, value: T);
}

// TODO: move in more generic mod
pub trait Get<T> {
    fn get(&self) -> &T;
}

pub trait FromUnit {
    fn from_unit(unit: &ClientUnit) -> Self;
}

// TODO: move into "unit" related mod
pub fn on_event<E: Event + WithUnitId, R: Resource + Set<Option<T>>, T: FromUnit>(
    trigger: Trigger<E>,
    slice: Res<GameSliceResource>,
    mut resource: ResMut<R>,
) {
    if let Some(slice) = &slice.0 {
        if let Some(unit) = slice.unit(trigger.event().unit_id()) {
            resource.set(Some(T::from_unit(unit)));
        }
    }
}

pub fn on_slice_propagated<
    R: Resource + Get<Option<T>> + Set<Option<T>>,
    T: FromUnit + WithUnitId,
>(
    _trigger: Trigger<GameSlicePropagated>,
    slice: Res<GameSliceResource>,
    mut resource: ResMut<R>,
) {
    if let (Some(slice), Some(resource_)) = (&slice.0, &resource.get()) {
        if let Some(unit) = slice.unit(resource_.unit_id()) {
            resource.set(Some(T::from_unit(unit)));
        } else {
            resource.set(None);
        }
    }
}
