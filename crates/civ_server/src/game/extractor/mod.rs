use std::sync::MutexGuard;

use common::{
    game::slice::{ClientCity, ClientUnit, GameSlice},
    space::window::Window,
};
use uuid::Uuid;

use crate::state::State;

use super::{city::IntoClientCity, unit::IntoClientUnit};

pub struct Extractor<'a> {
    state: &'a MutexGuard<'a, State>,
}

impl<'a> Extractor<'a> {
    pub fn new(state: &'a MutexGuard<'a, State>) -> Self {
        Self { state }
    }

    pub fn game_slice(&self, _client_id: &Uuid, window: &Window) -> GameSlice {
        let index = self.state.index();

        let cities: Vec<ClientCity> = index
            .xy_cities(window)
            .iter()
            .map(|uuid| {
                (
                    *uuid,
                    index
                        .uuid_cities()
                        .get(uuid)
                        .expect("Index must respect cities integrity"),
                )
            })
            .map(|(uuid, index)| self.state.city(*index, &uuid).unwrap())
            .map(|city| city.into_client())
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
                        .expect("Index must respect units integrity"),
                )
            })
            .map(|(uuid, index)| self.state.unit(*index, &uuid).unwrap())
            .map(|unit| unit.into_client(self.state))
            .collect::<Vec<ClientUnit>>();
        GameSlice::new(cities, units)
    }
}
