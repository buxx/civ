use common::{game::slice::GameSlice, space::window::Window};

use crate::runner::RunnerContext;

impl RunnerContext {
    pub fn game_slice(&self, window: &Window) -> GameSlice {
        let state = self.state();
        let world = self
            .world
            .read()
            .expect("Consider world as always readable");

        let tiles = world.slice(window);
        let cities = state.client_cities_slice(window);
        let units = state.client_units_slice(window);
        drop(state);

        GameSlice::new(
            *window.start(),
            (window.end().x - window.start().x + 1) as u64,
            (window.end().y - window.start().y + 1) as u64,
            tiles.clone(),
            cities,
            units,
        )
    }
}
