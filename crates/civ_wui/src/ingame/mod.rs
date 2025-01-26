use bevy::prelude::*;
use common::game::{slice::GameSlice as BaseGameSlice, GameFrame as BaseGameFrame};
use common::space::window::Window as BaseWindow;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameWindow>()
            .init_resource::<GameFrame>()
            .init_resource::<GameSlice>();
    }
}

#[derive(Resource, Default)]
pub struct GameFrame(pub Option<BaseGameFrame>);

#[derive(Resource, Default)]
pub struct GameSlice(pub Option<BaseGameSlice>);

#[derive(Resource, Default)]
pub struct GameWindow(pub Option<BaseWindow>);
