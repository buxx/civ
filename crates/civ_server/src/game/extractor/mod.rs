use common::{
    game::slice::{ClientCity, ClientUnit, GameSlice},
    space::window::Window,
};
use uuid::Uuid;

use crate::runner::RunnerContext;

pub struct Extractor {
    context: RunnerContext,
}

impl Extractor {
    pub fn new(context: RunnerContext) -> Self {
        Self { context }
    }

    pub fn game_slice(&self, _client_id: &Uuid, window: &Window) -> GameSlice {
        let state = self.context.state();
        let index = state.index();
        let cities: Vec<ClientCity> = index
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
            .map(|city| city.into())
            .collect::<Vec<ClientCity>>();
        let units: Vec<ClientUnit> = index
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
            .map(|unit| unit.into())
            .collect::<Vec<ClientUnit>>();
        GameSlice::new(cities, units)
    }
}
