use bevy::{prelude::*, window::PrimaryWindow};
use common::network::message::ClientToServerEstablishmentMessage;

use crate::{menu::join::TakePlaceEvent, to_server, utils::gui::window::IntoResolution};

pub fn take_place(
    trigger: Trigger<TakePlaceEvent>,
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<&GlobalTransform, With<Camera>>,
) {
    let flag = trigger.event().0;
    let Ok(window) = windows.single() else { return };
    let Ok(cam_transform) = cameras.single() else {
        return;
    };
    let resolution = (window, cam_transform).resolution();

    info!("Taking place as {} ...", &flag);

    to_server!(
        commands,
        ClientToServerEstablishmentMessage::TakePlace(flag, resolution)
    );
}
