use bevy::prelude::*;
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
    pub grid: FxHashMap<Hex, GridHex>,
    pub center: ImaginaryWorldPoint,
    pub relative_layout: HexLayout,
    pub absolute_layout: HexLayout,
    pub cities_index: FxHashMap<CityId, Hex>,
    pub units_index: FxHashMap<UnitId, Hex>,
}

impl std::ops::Deref for Grid {
    type Target = FxHashMap<Hex, GridHex>;

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
        grid: FxHashMap<Hex, GridHex>,
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

// #[cfg(test)]
// mod test {
//     use common::{
//         game::{
//             city::{CityExploitation, CityProduction, CityProductionTons},
//             nation::flag::Flag,
//             slice::ClientCityTasks,
//             tasks::client::{
//                 city::production::ClientCityProductionTask, ClientTask, ClientTaskType,
//             },
//             unit::UnitType,
//             GameFrame,
//         },
//         geo::GeoContext,
//         world::TerrainType,
//     };
//     use hexx::hex;

//     use crate::assets::tile::{absolute_layout, relative_layout};

//     use super::*;

//     fn build_city(point: WorldPoint) -> ClientCity {
//         ClientCity::builder()
//             .id(CityId::default())
//             .flag(Flag::Abkhazia)
//             .name("MyCity".to_string())
//             .geo(GeoContext::new(point))
//             .production(CityProduction::new(vec![]))
//             .exploitation(CityExploitation::new(CityProductionTons(0)))
//             .tasks(ClientCityTasks::new(ClientCityProductionTask::new(
//                 GameFrame(0),
//                 GameFrame(0),
//             )))
//             .build()
//     }

//     fn build_unit(point: WorldPoint) -> ClientUnit {
//         ClientUnit::builder()
//             .id(UnitId::default())
//             .flag(Flag::Abkhazia)
//             .type_(UnitType::Warriors)
//             .geo(GeoContext::new(point))
//             .task(ClientTask::new(
//                 ClientTaskType::Idle,
//                 GameFrame(0),
//                 GameFrame(0),
//             ))
//             .can(vec![])
//             .build()
//     }

//     // #[test]
//     // fn test_grid_indexes() {
//     //     // Given
//     //     let unit_point = WorldPoint::new(1, 2);
//     //     let unit = build_unit(unit_point);
//     //     let city_point = WorldPoint::new(4, 5);
//     //     let city = build_city(city_point);

//     //     let mut grid = FxHashMap::default();
//     //     grid.insert(
//     //         hex(unit_point.x as i32, unit_point.y as i32),
//     //         GridHex::new(
//     //             unit_point.into(),
//     //             unit_point,
//     //             GridHexResource::new(0, CtxTile::Visible(Tile::new(TerrainType::Plain))),
//     //             None,
//     //             Some(GridHexResource::new(1, vec![unit.clone()])),
//     //         ),
//     //     );
//     //     grid.insert(
//     //         hex(city_point.x as i32, city_point.y as i32),
//     //         GridHex::new(
//     //             unit_point.into(),
//     //             unit_point,
//     //             GridHexResource::new(0, CtxTile::Visible(Tile::new(TerrainType::Plain))),
//     //             Some(GridHexResource::new(2, city.clone())),
//     //             None,
//     //         ),
//     //     );

//     //     // When
//     //     let grid = Grid::new(
//     //         grid,
//     //         unit_point.into(),
//     //         relative_layout(&unit_point.into()),
//     //         absolute_layout(),
//     //     );

//     //     // Then
//     //     assert_eq!(
//     //         grid.city_index(city.id()),
//     //         Some(&hex(city_point.x as i32, city_point.y as i32))
//     //     );
//     //     assert_eq!(
//     //         grid.unit_index(unit.id()),
//     //         Some(&hex(unit_point.x as i32, unit_point.y as i32))
//     //     );
//     // }
// }
