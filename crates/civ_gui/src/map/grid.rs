use bevy::{prelude::*, utils::HashMap};
use common::{
    game::{
        city::CityId,
        slice::{ClientCity, ClientUnit},
        unit::UnitId,
    },
    geo::{ImaginaryWorldPoint, WorldPoint},
    world::{CtxTile, Tile},
};
use derive_more::Constructor;
use hexx::{Hex, HexLayout};
use rustc_hash::FxHashMap;

#[derive(Debug, Resource, Constructor, Default)]
pub struct GridResource(pub Option<Grid>);

#[derive(Debug)]
pub struct Grid {
    pub grid: HashMap<Hex, GridHex>,
    pub center: ImaginaryWorldPoint,
    pub relative_layout: HexLayout,
    pub absolute_layout: HexLayout,
    pub cities_index: FxHashMap<CityId, Hex>,
    pub units_index: FxHashMap<UnitId, Hex>,
}

impl std::ops::Deref for Grid {
    type Target = HashMap<Hex, GridHex>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl std::ops::DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

impl Grid {
    pub fn new(
        grid: HashMap<Hex, GridHex>,
        center: ImaginaryWorldPoint,
        relative_layout: HexLayout,
        absolute_layout: HexLayout,
    ) -> Self {
        let mut cities_index = FxHashMap::default();
        let mut units_index = FxHashMap::default();

        for (hex, grid) in &grid {
            if let Some(city) = &grid.city {
                cities_index.insert(*city.id(), *hex);
            }
            if let Some(units) = &grid.units {
                for unit in units.iter() {
                    units_index.insert(*unit.id(), *hex);
                }
            }
        }

        Self {
            grid,
            center,
            relative_layout,
            absolute_layout,
            cities_index,
            units_index,
        }
    }

    pub fn city_index(&self, city_id: &CityId) -> Option<&Hex> {
        self.cities_index.get(city_id)
    }

    pub fn unit_index(&self, unit_id: &UnitId) -> Option<&Hex> {
        self.units_index.get(unit_id)
    }
}

#[derive(Debug, Constructor)]
pub struct GridHex {
    pub _imaginary: ImaginaryWorldPoint,
    pub _point: WorldPoint,
    #[allow(unused)] // Only used in that feature for now
    pub tile: GridHexResource<CtxTile<Tile>>,
    pub city: Option<GridHexResource<ClientCity>>,
    pub units: Option<GridHexResource<Vec<ClientUnit>>>,
}

#[derive(Debug, Constructor)]
pub struct GridHexResource<T: Send + Sync> {
    #[allow(unused)] // Only used in that feature for now
    pub entity: Entity,
    pub item: T,
}

impl<T: Send + Sync> std::ops::Deref for GridHexResource<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct CurrentCursorHex(pub Option<Hex>);
