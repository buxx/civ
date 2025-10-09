use bevy::prelude::*;
use bevy_egui::{EguiContextSettings, EguiContexts};
use common::{
    game::{
        city::CityId,
        slice::{ClientCity, ClientUnit},
        unit::UnitId,
    },
    geo::{Geo, GeoContext},
    world::{CtxTile, Tile},
};

use crate::{
    core::GameSlicePropagated,
    ingame::{GameFrameResource, GameSliceResource},
};

use super::{DrawUiComponent, EGUI_DISPLAY_FACTOR};

pub mod unit;

#[macro_export]
macro_rules! add_city_component {
    ($app:expr, $resource:ty) => {
        $app.init_resource::<$resource>()
            .add_systems(
                Update,
                ($crate::ingame::interact::draw_component::<$resource>,)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_observer(
                $crate::ingame::interact::city_resource_on_event::<
                    <$resource as $crate::ingame::interact::UiComponentResource>::OnEvent,
                    $resource,
                    <$resource as $crate::ingame::interact::UiComponentResource>::Resource,
                >,
            )
            // FIXME: also on SetCity/RemoveCity
            .add_observer(
                $crate::ingame::interact::rebuild_city_resource_on_slice_propagated::<
                    $resource,
                    <$resource as $crate::ingame::interact::UiComponentResource>::Resource,
                >,
            )
    };
}

#[macro_export]
macro_rules! add_unit_component {
    ($app:expr, $resource:ty) => {
        $app.init_resource::<$resource>()
            .add_systems(
                Update,
                ($crate::ingame::interact::draw_component::<$resource>,)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_observer(
                $crate::ingame::interact::unit_resource_on_event::<
                    <$resource as $crate::ingame::interact::UiComponentResource>::OnEvent,
                    $resource,
                    <$resource as $crate::ingame::interact::UiComponentResource>::Resource,
                >,
            )
            // FIXME: also on SetUnit/RemoveUnit
            .add_observer(
                $crate::ingame::interact::rebuild_unit_resource_on_slice_propagated::<
                    $resource,
                    <$resource as $crate::ingame::interact::UiComponentResource>::Resource,
                >,
            )
    };
}

#[macro_export]
macro_rules! add_tile_component {
    ($app:expr, $resource:ty) => {
        use bevy_egui::EguiContextPass;

        $app.init_resource::<$resource>()
            .add_systems(
                EguiContextPass,
                ($crate::ingame::interact::draw_component::<$resource>,)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_observer(
                $crate::ingame::interact::tile_resource_on_event::<
                    <$resource as $crate::ingame::interact::UiComponentResource>::OnEvent,
                    $resource,
                    <$resource as $crate::ingame::interact::UiComponentResource>::Resource,
                >,
            )
            .add_observer(
                $crate::ingame::interact::rebuild_tile_resource_on_slice_propagated::<
                    $resource,
                    <$resource as $crate::ingame::interact::UiComponentResource>::Resource,
                >,
            )
    };
}

pub trait UiComponentResource: Resource {
    type Resource: DrawUiComponent;
    type OnEvent: Event;

    fn _component(&self) -> &Option<Self::Resource>;
    fn component_mut(&mut self) -> &mut Option<Self::Resource>;
}

#[macro_export]
macro_rules! impl_ui_component_resource {
    ($type:ty, $inner:ty, $event:ty) => {
        impl $crate::ingame::interact::UiComponentResource for $type {
            type Resource = $inner;
            type OnEvent = $event;

            fn _component(&self) -> &Option<$inner> {
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
    frame: Res<GameFrameResource>,
    windows: Query<&Window>,
) {
    let mut disband = false;

    if let (Some(component), Some(frame)) = (resource.component_mut(), frame.0) {
        if let Ok((mut egui_settings, _)) = egui.single_mut() {
            egui_settings.scale_factor = EGUI_DISPLAY_FACTOR;
        }

        let Ok(window) = windows.single() else { return };
        disband = component.draw(contexts.ctx_mut(), window, &mut commands, frame);
    }

    if disband {
        *resource.component_mut() = None;
    }
}

pub trait WithUnitId {
    fn unit_id(&self) -> &UnitId;
}

pub trait WithCityId {
    fn city_id(&self) -> &CityId;
}

// TODO: move in more generic mod
pub trait Set<T> {
    fn set(&mut self, value: T);
}

// TODO: move in more generic mod
pub trait Get<T> {
    fn get(&self) -> &T;
}

pub fn city_resource_on_event<
    E: Event + WithCityId,
    R: Resource + Set<Option<T>>,
    T: From<ClientCity>,
>(
    trigger: Trigger<E>,
    slice: Res<GameSliceResource>,
    mut resource: ResMut<R>,
) {
    if let Some(slice) = &slice.0 {
        if let Some(city) = slice.city(trigger.event().city_id()) {
            resource.set(Some(T::from(city.clone())));
        }
    }
}

pub fn unit_resource_on_event<
    E: Event + WithUnitId,
    R: Resource + Set<Option<T>>,
    T: From<ClientUnit>,
>(
    trigger: Trigger<E>,
    slice: Res<GameSliceResource>,
    mut resource: ResMut<R>,
) {
    if let Some(slice) = &slice.0 {
        if let Some(unit) = slice.unit(trigger.event().unit_id()) {
            resource.set(Some(T::from(unit.clone())));
        }
    }
}

pub fn tile_resource_on_event<
    E: Event + Geo,
    R: Resource + Set<Option<T>>,
    T: for<'a> TryFrom<(GeoContext, &'a CtxTile<Tile>)>,
>(
    trigger: Trigger<E>,
    slice: Res<GameSliceResource>,
    mut resource: ResMut<R>,
) {
    if let Some(slice) = &slice.0 {
        let geo = *trigger.event().geo();
        if let Some(tile) = slice.tiles().get(geo.point()) {
            resource.set(T::try_from((geo, tile)).ok());
        }
    }
}

/// Implement a trigger on `GameSlicePropagated` event to rebuild
/// automatically the given resource.
pub fn rebuild_unit_resource_on_slice_propagated<
    R: Resource + Get<Option<T>> + Set<Option<T>>,
    T: From<ClientUnit> + WithUnitId,
>(
    _trigger: Trigger<GameSlicePropagated>,
    slice: Res<GameSliceResource>,
    mut resource: ResMut<R>,
) {
    if let (Some(slice), Some(resource_)) = (&slice.0, &resource.get()) {
        if let Some(unit) = slice.unit(resource_.unit_id()) {
            resource.set(Some(T::from(unit.clone())));
        } else {
            resource.set(None);
        }
    }
}

/// Implement a trigger on `GameSlicePropagated` event to rebuild
/// automatically the given resource.
pub fn rebuild_city_resource_on_slice_propagated<
    R: Resource + Get<Option<T>> + Set<Option<T>>,
    T: From<ClientCity> + WithCityId,
>(
    _trigger: Trigger<GameSlicePropagated>,
    slice: Res<GameSliceResource>,
    mut resource: ResMut<R>,
) {
    if let (Some(slice), Some(resource_)) = (&slice.0, &resource.get()) {
        if let Some(city) = slice.city(resource_.city_id()) {
            resource.set(Some(T::from(city.clone())));
        } else {
            resource.set(None);
        }
    }
}

/// Implement a trigger on `GameSlicePropagated` event to rebuild
/// automatically the given resource.
pub fn rebuild_tile_resource_on_slice_propagated<
    R: Resource + Get<Option<T>> + Set<Option<T>>,
    T: for<'a> TryFrom<(GeoContext, &'a CtxTile<Tile>)> + Geo,
>(
    _trigger: Trigger<GameSlicePropagated>,
    slice: Res<GameSliceResource>,
    mut resource: ResMut<R>,
) {
    if let (Some(slice), Some(resource_)) = (&slice.0, &resource.get()) {
        let geo = *resource_.geo();
        if let Some(tile) = slice.tiles().get(geo.point()) {
            resource.set(T::try_from((geo, tile)).ok());
        } else {
            resource.set(None);
        }
    }
}
