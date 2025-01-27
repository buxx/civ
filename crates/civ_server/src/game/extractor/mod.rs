use std::sync::RwLockReadGuard;

use common::{
    game::slice::{ClientCity, ClientUnit, GameSlice},
    geo::{ImaginaryWorldPoint, WorldPoint},
    network::Client,
    space::window::Window,
    world::{partial::PartialWorld, reader::WorldReader, CtxTile, Tile},
};

use crate::state::State;

use super::IntoClientModel;

pub struct Extractor<'a> {
    state: RwLockReadGuard<'a, State>,
    world: RwLockReadGuard<'a, WorldReader>,
}

impl<'a> Extractor<'a> {
    pub fn new(state: RwLockReadGuard<'a, State>, world: RwLockReadGuard<'a, WorldReader>) -> Self {
        Self { state, world }
    }

    pub fn game_slice(&self, _client: &Client, window: &Window) -> GameSlice {
        let world = self.world(window);
        let cities: Vec<ClientCity> = self.cities(window);
        let units: Vec<ClientUnit> = self.units(window);
        GameSlice::new(world, cities, units)
    }

    fn world(&self, window: &Window) -> PartialWorld {
        let tiles = self.world.window_tiles(window);
        PartialWorld::new(
            *window.start(),
            (window.end().x - window.start().x + 1) as u64,
            (window.end().y - window.start().y + 1) as u64,
            tiles
                .into_iter()
                .map(|t| t.into())
                .collect::<Vec<CtxTile<Tile>>>(),
        )
    }

    fn cities(&self, window: &Window) -> Vec<ClientCity> {
        let index = self.state.index();
        index
            .window_cities(window)
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
            .map(|city| city.clone().into_client(&self.state))
            .collect::<Vec<ClientCity>>()
    }

    fn units(&self, window: &Window) -> Vec<ClientUnit> {
        let index = self.state.index();
        index
            .window_units(window)
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
            .map(|unit| unit.clone().into_client(&self.state))
            .collect::<Vec<ClientUnit>>()
    }
}
