use common::{
    game::{city::CityId, nation::flag::Flag, unit::UnitId},
    geo::{Geo, GeoContext},
    space::{CityVec2dIndex, UnitVec2dIndex},
    utils::Vec2d,
};
use rustc_hash::FxHashMap;

use crate::{
    effect::{CityEffect, Effect, RunnerEffect, StateEffect, TaskEffect, TasksEffect, UnitEffect},
    game::{city::City, unit::Unit},
    snapshot::Snapshot,
    task::{Concern, Task, TaskBox, TaskId},
};

// FIXME BS NOW: unit tests (try generate to juge)
#[derive(Default)]
pub struct Index {
    cities_index: FxHashMap<CityId, CityVec2dIndex>,
    units_index: FxHashMap<UnitId, UnitVec2dIndex>,
    flag_cities: FxHashMap<Flag, Vec<CityId>>,
    flag_units: FxHashMap<Flag, Vec<UnitId>>,
    city_tasks: FxHashMap<CityId, Vec<TaskId>>,
    unit_tasks: FxHashMap<UnitId, Vec<TaskId>>,
}

impl Index {
    pub fn build_from(
        cities: &Vec2d<Box<City>>,
        units: &Vec2d<Vec<Unit>>,
        tasks: &Vec<Box<dyn Task>>,
    ) -> Self {
        let mut index = Self::default();

        for city in cities.into_iter().flatten() {
            index.apply_new_city(city, cities);
        }

        for units_ in units.iter().flatten() {
            for unit in units_ {
                index.apply_new_unit(unit, units);
            }
        }

        let tasks: Vec<TaskBox> = tasks.iter().map(|bx| bx.boxed()).collect();
        for task in tasks {
            index.apply_new_task(&task);
        }

        index
    }

    pub fn reindex_units_at(&mut self, geos: Vec<GeoContext>, units: &Vec2d<Vec<Unit>>) {
        for geo in geos {
            if let Some(units_) = units.get_by_point(*geo.point()) {
                for (i, unit) in units_.iter().enumerate() {
                    self.units_index
                        .insert(*unit.id(), UnitVec2dIndex(units.index(*geo.point()), i));
                }
            }
        }
    }

    pub fn apply(
        &mut self,
        effects: &Vec<Effect>,
        cities: &Vec2d<Box<City>>,
        units: &Vec2d<Vec<Unit>>,
    ) {
        let mut reindex_units_at = vec![];

        for effect in effects {
            match effect {
                Effect::Runner(effect) => match effect {
                    RunnerEffect::Tasks(effect) => match effect {
                        TasksEffect::Remove(tasks) => {
                            for (task_id, concern) in tasks {
                                self.apply_remove_task(task_id, concern)
                            }
                        }
                        TasksEffect::Add(tasks) => {
                            for task in tasks {
                                self.apply_new_task(task)
                            }
                        }
                    },
                    RunnerEffect::Task(_, effect) => match effect {
                        TaskEffect::Push(task) => {
                            self.apply_new_task(task);
                        }
                        TaskEffect::Finished(task) => {
                            self.apply_remove_task(task.context().id(), &task.concern())
                        }
                        TaskEffect::Remove(uuid, concern) => {
                            self.apply_remove_task(uuid, concern);
                        }
                    },
                },
                Effect::State(effect) => match effect {
                    StateEffect::IncrementGameFrame => {}
                    StateEffect::Clients(_) => {}
                    StateEffect::Client(_, _) => {}
                    StateEffect::City(_, effect) => match effect {
                        CityEffect::New(city) => {
                            self.apply_new_city(city, cities);
                        }
                        CityEffect::Remove(city) => {
                            self.apply_remove_city(city, cities);
                        }
                        CityEffect::Replace(_) => {
                            // Tasks already added/removed by TasksEffect
                            // Nothing else to do
                        }
                    },
                    StateEffect::Unit(_, effect) => match effect {
                        UnitEffect::New(unit) => {
                            self.apply_new_unit(unit, units);
                        }
                        UnitEffect::Remove(unit) => {
                            reindex_units_at.push(*unit.geo()); // Because State Vec2d of units changed
                            self.apply_remove_unit(unit, units);
                        }
                        UnitEffect::Replace(unit) => {
                            self.apply_replace_unit(unit, units);
                        }
                    },
                    StateEffect::Testing => {}
                },
                Effect::Shines(_) => {}
            }
        }

        reindex_units_at.sort();
        reindex_units_at.dedup();
        self.reindex_units_at(reindex_units_at, units);
    }

    fn apply_new_task(&mut self, task: &TaskBox) {
        match task.concern() {
            Concern::Unit(uuid) => self
                .unit_tasks
                .entry(uuid)
                .or_default()
                .push(*task.context().id()),
            Concern::City(uuid) => self
                .city_tasks
                .entry(uuid)
                .or_default()
                .push(*task.context().id()),
            Concern::Nothing => {}
        }
    }

    fn apply_remove_task(&mut self, task_id: &TaskId, concern: &Concern) {
        match concern {
            Concern::Unit(uuid) => self
                .unit_tasks
                .entry(*uuid)
                .or_default()
                .retain(|id| id != task_id),
            Concern::City(uuid) => self
                .city_tasks
                .entry(*uuid)
                .or_default()
                .retain(|id| id != task_id),
            Concern::Nothing => {}
        }
    }

    fn apply_new_city(&mut self, city: &City, cities: &Vec2d<Box<City>>) {
        let index = cities.index(*city.geo().point());
        self.cities_index.insert(*city.id(), CityVec2dIndex(index));
        self.flag_cities
            .entry(*city.flag())
            .or_default()
            .push(*city.id());
        // self.city_tasks already updated by TaskEffect::Push
    }

    fn apply_remove_city(&mut self, city: &City, _cities: &Vec2d<Box<City>>) {
        self.cities_index.remove(city.id());
        self.flag_cities
            .get_mut(city.flag())
            .map(|vec| {
                vec.retain(|i| i != city.id());
                vec.is_empty()
            })
            .filter(|is_empty| *is_empty)
            .map(|_| self.flag_cities.remove(city.flag()));
        // self.city_tasks already updated by TaskEffect::Finished and TaskEffect::Remove
    }

    fn apply_new_unit(&mut self, unit: &Unit, units: &Vec2d<Vec<Unit>>) {
        let index = units.index(*unit.geo().point());
        let index2 = units
            .get(index)
            .as_ref()
            .and_then(|u| u.iter().position(|u| u.id() == unit.id()))
            .expect("Assume integrity of units parameters");
        self.units_index
            .insert(*unit.id(), UnitVec2dIndex(index, index2));
        self.flag_units
            .entry(*unit.flag())
            .or_default()
            .push(*unit.id());
        // self.unit_tasks already updated by TaskEffect::Push
    }

    fn apply_remove_unit(&mut self, unit: &Unit, _units: &Vec2d<Vec<Unit>>) {
        self.units_index.remove(unit.id());
        self.flag_units
            .get_mut(unit.flag())
            .map(|vec| {
                vec.retain(|i| i != unit.id());
                vec.is_empty()
            })
            .filter(|is_empty| *is_empty)
            .map(|_| self.flag_units.remove(unit.flag()));
        // self.unit_tasks already updated by TaskEffect::Finished and TaskEffect::Remove
    }

    fn apply_replace_unit(&mut self, unit: &Unit, units: &Vec2d<Vec<Unit>>) {
        self.apply_remove_unit(unit, units);
        self.apply_new_unit(unit, units);
    }

    pub fn cities_index(&self) -> &FxHashMap<CityId, CityVec2dIndex> {
        &self.cities_index
    }

    pub fn units_index(&self) -> &FxHashMap<UnitId, UnitVec2dIndex> {
        &self.units_index
    }

    pub fn city_tasks(&self, city_id: &CityId) -> Vec<TaskId> {
        match self.city_tasks.get(city_id) {
            Some(uuids) => uuids.to_vec(),
            None => vec![],
        }
    }

    pub fn unit_tasks(&self, unit_id: &UnitId) -> &Vec<TaskId> {
        static EMPTY_VEC: Vec<TaskId> = Vec::new();
        self.unit_tasks.get(unit_id).unwrap_or(&EMPTY_VEC)
    }

    pub fn flag_units(&self) -> &FxHashMap<Flag, Vec<UnitId>> {
        &self.flag_units
    }
}

impl From<&Snapshot> for Index {
    fn from(value: &Snapshot) -> Self {
        Self::build_from(value.cities(), value.units(), &value.tasks().to_vec())
    }
}

#[cfg(test)]
mod test {
    use common::{
        game::unit::UnitType,
        geo::{GeoVec, WorldPoint},
        space::D2Size,
    };

    use super::*;

    pub fn build_unit(geo: GeoContext, flag: Flag) -> Unit {
        Unit::builder()
            .id(UnitId::default())
            .geo(geo)
            .type_(UnitType::Warriors)
            .flag(flag)
            .can(vec![])
            .build()
    }

    #[test]
    fn test_index_unit_manipulations() {
        let size = D2Size::new(2, 2);
        let cities = Vec2d::from(size, Vec::<City>::new());
        let unit1_geo = GeoContext::new(WorldPoint::new(0, 0));
        let unit1 = build_unit(unit1_geo, Flag::Abkhazia);
        let unit2_geo = GeoContext::new(WorldPoint::new(0, 0));
        let unit2 = build_unit(unit2_geo, Flag::Aborigines);
        let unit3_geo = GeoContext::new(WorldPoint::new(1, 1));
        let unit3 = build_unit(unit3_geo, Flag::Abkhazia);
        let units = Vec2d::from(
            size,
            vec![
                GeoVec::new(unit1_geo, vec![unit1.clone(), unit2.clone()]),
                GeoVec::new(unit3_geo, vec![unit3.clone()]),
            ],
        );

        // Empty index
        let mut index = Index::default();
        assert_eq!(index.units_index(), &FxHashMap::default());
        assert_eq!(index.flag_units(), &FxHashMap::default());
        assert_eq!(index.unit_tasks(unit1.id()), &vec![]);
        assert_eq!(index.unit_tasks(unit2.id()), &vec![]);
        assert_eq!(index.unit_tasks(unit3.id()), &vec![]);

        // Full filled index
        index.apply(
            &vec![
                StateEffect::Unit(*unit1.id(), UnitEffect::New(unit1.clone())).into(),
                StateEffect::Unit(*unit2.id(), UnitEffect::New(unit2.clone())).into(),
                StateEffect::Unit(*unit3.id(), UnitEffect::New(unit3.clone())).into(),
            ],
            &cities,
            &units,
        );

        assert_eq!(
            index.units_index().get(unit1.id()),
            Some(&UnitVec2dIndex(0, 0))
        );
        assert_eq!(
            index.units_index().get(unit2.id()),
            Some(&UnitVec2dIndex(0, 1))
        );
        assert_eq!(
            index.units_index().get(unit3.id()),
            // `3` is 1,1 in Vec2d (width=2, height=2) index
            Some(&UnitVec2dIndex(3, 0))
        );

        // Removed unit imply reindex
        let units = Vec2d::from(
            D2Size::new(2, 2),
            vec![
                GeoVec::new(unit1_geo, vec![unit2.clone()]),
                GeoVec::new(unit3_geo, vec![unit3.clone()]),
            ],
        );
        index.apply(
            &vec![StateEffect::Unit(*unit1.id(), UnitEffect::Remove(unit1.clone())).into()],
            &cities,
            &units,
        );

        assert_eq!(
            index.units_index().get(unit2.id()),
            // Second index (index in position vec of units) is now 0
            Some(&UnitVec2dIndex(0, 0))
        );
        assert_eq!(
            index.units_index().get(unit3.id()),
            Some(&UnitVec2dIndex(3, 0))
        );

        // TODO: test tasks & cities
    }
}
