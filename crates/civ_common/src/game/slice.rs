use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    geo::{GeoContext, ImaginaryWorldPoint, WorldPoint},
    world::{slice::Slice, CtxTile, Tile},
};

use super::{
    city::{CityExploitation, CityId, CityProduction},
    nation::flag::Flag,
    tasks::client::{city::production::ClientCityProductionTask, ClientTask},
    unit::{UnitCan, UnitId, UnitType},
    GameFrame,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GameSlice {
    original: ImaginaryWorldPoint,
    width: u64,
    height: u64,
    tiles: Slice<CtxTile<Tile>>,
    cities: Slice<Option<ClientCity>>,
    units: Slice<Vec<ClientUnit>>,
}

impl GameSlice {
    pub fn new(
        original: ImaginaryWorldPoint,
        width: u64,
        height: u64,
        tiles: Slice<CtxTile<Tile>>,
        cities: Slice<Option<ClientCity>>,
        units: Slice<Vec<ClientUnit>>,
    ) -> Self {
        Self {
            original,
            width,
            height,
            tiles,
            cities,
            units,
        }
    }

    pub fn try_world_point_for_center_rel(&self, pos: (isize, isize)) -> Option<WorldPoint> {
        let original_x = self.original.x as isize;
        let original_y = self.original.y as isize;
        let rel_center_x = (self.width / 2) as isize;
        let rel_center_y = (self.height / 2) as isize;
        let rel_x = rel_center_x + pos.0;
        let rel_y = rel_center_y + pos.1;
        let world_x = original_x + rel_x;
        let world_y = original_y + rel_y;

        if world_x < 0
            || world_y < 0
            || world_x > (original_x + (self.width as isize - 1))
            || world_y > (original_y + (self.height as isize - 1))
        {
            return None;
        }

        Some(WorldPoint::new(world_x as u64, world_y as u64))
    }

    pub fn imaginary_world_point_for_center_rel(&self, pos: (isize, isize)) -> ImaginaryWorldPoint {
        let original_x = self.original.x as isize;
        let original_y = self.original.y as isize;
        let rel_center_x = (self.width / 2) as isize;
        let rel_center_y = (self.height / 2) as isize;
        let rel_x = rel_center_x + pos.0;
        let rel_y = rel_center_y + pos.1;
        let world_x = original_x + rel_x;
        let world_y = original_y + rel_y;

        ImaginaryWorldPoint::new(world_x as i64, world_y as i64)
    }

    pub fn tiles(&self) -> &Slice<CtxTile<Tile>> {
        &self.tiles
    }

    pub fn center(&self) -> ImaginaryWorldPoint {
        self.imaginary_world_point_for_center_rel((0, 0))
    }

    pub fn cities(&self) -> &Slice<Option<ClientCity>> {
        &self.cities
    }

    pub fn cities_mut(&mut self) -> &mut Slice<Option<ClientCity>> {
        &mut self.cities
    }

    pub fn units(&self) -> &Slice<Vec<ClientUnit>> {
        &self.units
    }

    pub fn units_mut(&mut self) -> &mut Slice<Vec<ClientUnit>> {
        &mut self.units
    }
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientCity {
    id: CityId,
    flag: Flag,
    name: String,
    geo: GeoContext,
    production: CityProduction,
    exploitation: CityExploitation,
    tasks: ClientCityTasks,
}

impl ClientCity {
    pub fn id(&self) -> &CityId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn flag(&self) -> &Flag {
        &self.flag
    }

    pub fn geo(&self) -> &GeoContext {
        &self.geo
    }

    pub fn production_str(&self, frame: &GameFrame) -> String {
        format!(
            "{} ({}%)",
            self.production.current(),
            self.tasks.production.progress(frame)
        )
    }
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientCityTasks {
    production: ClientCityProductionTask,
}

impl ClientCityTasks {
    pub fn new(production: ClientCityProductionTask) -> Self {
        Self { production }
    }
}

#[derive(Serialize, Deserialize, Clone, Builder, Debug, PartialEq)]
pub struct ClientUnit {
    id: UnitId,
    flag: Flag,
    type_: UnitType,
    geo: GeoContext,
    task: Option<ClientTask>,
    can: Vec<UnitCan>,
}

impl ClientUnit {
    pub fn id(&self) -> &UnitId {
        &self.id
    }

    pub fn flag(&self) -> &Flag {
        &self.flag
    }

    pub fn geo(&self) -> &GeoContext {
        &self.geo
    }

    pub fn geo_mut(&mut self) -> &mut GeoContext {
        &mut self.geo
    }

    pub fn type_(&self) -> &UnitType {
        &self.type_
    }

    pub fn task(&self) -> &Option<ClientTask> {
        &self.task
    }

    pub fn can(&self) -> &[UnitCan] {
        &self.can
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case((0, 0), (9, 9), (0, 0), (4, 4))]
    #[case((0, 0), (9, 9), (0, -4), (4, 0))]
    #[case((0, 0), (9, 9), (-4, 0), (0, 4))]
    #[case((0, 0), (9, 9), (-4, -4), (0, 0))]
    #[case((0, 0), (9, 9), (4, 4), (8, 8))]
    #[case((0, 0), (10, 10), (0, 0), (5,5))]
    #[case((0, 0), (10, 10), (0, -4), (5, 1))]
    #[case((0, 0), (10, 10), (-4, 0), (1, 5))]
    #[case((0, 0), (10, 10), (-4, -4), (1, 1))]
    #[case((0, 0), (10, 10), (4, 4), (9, 9))]
    #[case((0, 0), (15, 12), (0, 0), (7, 6))]
    fn test_partial_world_point_for_center_rel(
        #[case] point: (u64, u64),
        #[case] size: (u64, u64),
        #[case] rel: (isize, isize),
        #[case] expected: (u64, u64),
    ) {
        let world = GameSlice::new(
            point.into(),
            size.0,
            size.1,
            Slice::zero(),
            Slice::zero(),
            Slice::zero(),
        );
        assert_eq!(
            world.try_world_point_for_center_rel(rel),
            Some(expected.into())
        );
    }

    #[rstest]
    #[case((-2, -2), (10, 100))]
    #[case((-1, -2), (11, 100))]
    #[case((0, -2), (12, 100))]
    #[case((1, -2), (13, 100))]
    #[case((-2, -1), (10, 101))]
    #[case((-1, -1), (11, 101))]
    #[case((0, -1), (12, 101))]
    #[case((1, -1), (13, 101))]
    #[case((-2, 0), (10, 102))]
    #[case((-1, 0), (11, 102))]
    #[case((0, 0), (12, 102))]
    #[case((1, 0), (13, 102))]
    #[case((-2, 1), (10, 103))]
    #[case((-1, 1), (11, 103))]
    #[case((0, 1), (12, 103))]
    #[case((1, 1), (13, 103))]
    fn test_try_world_point_for_center_rel_by_one(
        #[case] rel: (isize, isize),
        #[case] abs: (u64, u64),
    ) {
        let world = GameSlice::new(
            ImaginaryWorldPoint::new(10, 100),
            4,
            4,
            Slice::zero(),
            Slice::zero(),
            Slice::zero(),
        );
        assert_eq!(world.try_world_point_for_center_rel(rel), Some(abs.into()));
    }

    #[rstest]
    #[case((-2, -2), (10, 100))]
    #[case((-1, -2), (11, 100))]
    #[case((0, -2), (12, 100))]
    #[case((1, -2), (13, 100))]
    #[case((-2, -1), (10, 101))]
    #[case((-1, -1), (11, 101))]
    #[case((0, -1), (12, 101))]
    #[case((1, -1), (13, 101))]
    #[case((-2, 0), (10, 102))]
    #[case((-1, 0), (11, 102))]
    #[case((0, 0), (12, 102))]
    #[case((1, 0), (13, 102))]
    #[case((-2, 1), (10, 103))]
    #[case((-1, 1), (11, 103))]
    #[case((0, 1), (12, 103))]
    #[case((1, 1), (13, 103))]
    fn test_try_world_point_for_center_rel_by_two(
        #[case] rel: (isize, isize),
        #[case] abs: (u64, u64),
    ) {
        let world = GameSlice::new(
            ImaginaryWorldPoint::new(10, 100),
            4,
            4,
            Slice::zero(),
            Slice::zero(),
            Slice::zero(),
        );
        assert_eq!(world.try_world_point_for_center_rel(rel), Some(abs.into()));
    }
}
