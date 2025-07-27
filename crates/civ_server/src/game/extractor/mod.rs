use std::sync::RwLockReadGuard;

use common::{
    game::slice::{ClientCity, ClientUnit, GameSlice},
    space::window::Window,
    world::{partial::Slice, reader::WorldReader, CtxTile, Tile},
};
use extfn::extfn;

use crate::{runner::Runner, state::State};

use super::IntoClientModel;

#[extfn]
pub fn extract_units(self: &RwLockReadGuard<'_, State>, window: &Window) -> Vec<ClientUnit> {
    let index = self.index();
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
        .map(|(uuid, index)| self.unit(*index, &uuid).unwrap())
        .map(|unit| unit.clone().into_client(self))
        .collect::<Vec<ClientUnit>>()
}

#[extfn]
pub fn extract_cities(self: &RwLockReadGuard<'_, State>, window: &Window) -> Vec<ClientCity> {
    let index = self.index();
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
        .map(|(uuid, index)| self.city(*index, &uuid).unwrap())
        .map(|city| city.clone().into_client(self))
        .collect::<Vec<ClientCity>>()
}

#[extfn]
pub fn extract_tiles(
    self: &RwLockReadGuard<'_, WorldReader>,
    window: &Window,
) -> Slice<CtxTile<Tile>> {
    let tiles = self.window_tiles(window);
    Slice::new(
        *window.start(),
        (window.end().x - window.start().x + 1) as u64,
        (window.end().y - window.start().y + 1) as u64,
        tiles
            .into_iter()
            .map(|t| t.into())
            .collect::<Vec<CtxTile<Tile>>>(),
    )
}

#[extfn]
pub fn game_slice(self: &Runner, window: &Window) -> GameSlice {
    let state = self.context.state();
    let world = self
        .context
        .world
        .read()
        .expect("Consider world as always readable");
    GameSlice::new(
        world.extract_tiles(window),
        state.extract_cities(window),
        state.extract_units(window),
    )
}
