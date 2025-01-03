use std::sync::RwLockReadGuard;

use common::{
    game::slice::{ClientCity, ClientUnit, GameSlice},
    geo::WorldPoint,
    space::window::Window,
    world::{partial::PartialWorld, Tile},
};
use uuid::Uuid;

use crate::{state::State, world::reader::WorldReader};

use super::{city::IntoClientCity, unit::IntoClientUnit};

// FIXME: lifetime not required ?
pub struct Extractor<'a, 'b> {
    state: &'a RwLockReadGuard<'a, State>,
    world: &'b RwLockReadGuard<'b, WorldReader>,
}

impl<'a, 'b> Extractor<'a, 'b> {
    pub fn new(
        state: &'a RwLockReadGuard<'a, State>,
        world: &'b RwLockReadGuard<'b, WorldReader>,
    ) -> Self {
        Self { state, world }
    }

    pub fn game_slice(&self, _client_id: &Uuid, window: &Window) -> GameSlice {
        let world = self.world(window);
        let cities: Vec<ClientCity> = self.cities(window);
        let units: Vec<ClientUnit> = self.units(window);
        GameSlice::new(world, cities, units)
    }

    fn world(&self, window: &Window) -> PartialWorld {
        let tiles = self.world.window_tiles(window);
        PartialWorld::new(
            WorldPoint::new(window.start_x(), window.start_y()),
            window.end_x() - window.start_x(),
            window.end_y() - window.start_y(),
            tiles.into_iter().cloned().collect::<Vec<Tile>>(),
        )
    }

    fn cities(&self, window: &Window) -> Vec<ClientCity> {
        let index = self.state.index();
        index
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
            .collect::<Vec<ClientCity>>()
    }

    fn units(&self, window: &Window) -> Vec<ClientUnit> {
        let index = self.state.index();
        index
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
            .collect::<Vec<ClientUnit>>()
    }
}
