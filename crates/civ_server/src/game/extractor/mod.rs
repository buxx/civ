use common::{
    game::slice::GameSlice,
    space::window::{DisplayStep, Window},
};
use uuid::Uuid;

use crate::runner::RunnerContext;

use super::city::City;

pub struct Extractor {
    context: RunnerContext,
}

impl Extractor {
    pub fn new(context: RunnerContext) -> Self {
        Self { context }
    }

    pub fn game_slice(&self, client_id: &Uuid, window: &Window) -> GameSlice {
        let state = self.context.state();
        let step = DisplayStep::from_shape(window.shape());
        let index = state.index();
        let cities: Vec<City> = index
            .xy_cities(window)
            .iter()
            .map(|uuid| {
                (
                    *uuid,
                    index
                        .uuid_cities()
                        .get(uuid)
                        .expect("Index must respect integrity"),
                )
            })
            .map(|(uuid, index)| state.city(*index, &uuid).unwrap())
            .cloned()
            .collect();
        let units: Vec<City> = index
            .xy_units(window)
            .iter()
            .map(|uuid| {
                (
                    *uuid,
                    index
                        .uuid_units()
                        .get(uuid)
                        .expect("Index must respect integrity"),
                )
            })
            .map(|(uuid, index)| state.unit(*index, &uuid).unwrap())
            .cloned()
            .collect();
        GameSlice::new(cities, units)
    }
}
