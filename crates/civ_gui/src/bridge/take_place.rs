use bevy::{prelude::*, window::PrimaryWindow};
use common::network::message::ClientToServerEstablishmentMessage;

use crate::{menu::join::TakePlaceEvent, to_server, utils::gui::window::IntoResolution};

pub fn take_place(
    trigger: Trigger<TakePlaceEvent>,
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<&GlobalTransform>,
) {
    let flag = trigger.event().0;
    let window = windows.single();
    let cam_transform = cameras.single();
    let resolution = (window, cam_transform).resolution();

    info!("Taking place as {} ...", &flag);

    to_server!(
        commands,
        ClientToServerEstablishmentMessage::TakePlace(flag, resolution)
    );
}
