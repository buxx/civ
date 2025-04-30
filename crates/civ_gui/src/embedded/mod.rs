use bevy::prelude::*;
use derive_more::Constructor;

#[derive(Default)]
pub struct EmbeddedPlugin;

#[derive(Event, Constructor)]
pub struct StartNewLocalGame(NewLocalGameConfig);

pub struct NewLocalGameConfig;

impl Plugin for EmbeddedPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(react_start_new_local_game);
    }
}

fn react_start_new_local_game(_trigger: Trigger<StartNewLocalGame>, mut _commands: Commands) {
    todo!()
}
