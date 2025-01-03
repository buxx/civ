use std::collections::HashMap;

use common::{
    geo::{Geo, WorldPoint},
    space::window::{DisplayStep, Window},
};
use uuid::Uuid;

use crate::{
    game::{city::City, unit::Unit},
    task::{
        effect::{CityEffect, Effect, StateEffect, TaskEffect, UnitEffect},
        Concern, TaskBox,
    },
};

#[derive(Default)]
pub struct Index {
    cities_index: HashMap<Uuid, usize>,
    units_index: HashMap<Uuid, usize>,
    cities_xy: HashMap<Uuid, WorldPoint>, // TODO: not used, no ?
    units_xy: HashMap<Uuid, WorldPoint>,  // TODO: not used, no ?
    xy_cities: HashMap<WorldPoint, Uuid>,
    xy_units: HashMap<WorldPoint, Vec<Uuid>>,
    city_tasks: HashMap<Uuid, Vec<Uuid>>,
    unit_tasks: HashMap<Uuid, Vec<Uuid>>,
}

impl Index {
    pub fn reindex_cities(&mut self, cities: &Vec<City>) {
        self.cities_index.clear();
        self.cities_xy.clear();
        self.xy_cities.clear();

        for (i, city) in cities.iter().enumerate() {
            self.cities_index.insert(city.id(), i);
            self.cities_xy.insert(city.id(), *city.geo().point());
            self.xy_cities.insert(*city.geo().point(), city.id());
        }
    }

    pub fn reindex_units(&mut self, units: &Vec<Unit>) {
        self.units_index.clear();
        self.units_xy.clear();
        self.xy_units.clear();

        for (i, unit) in units.iter().enumerate() {
            self.units_index.insert(unit.id(), i);
            self.units_xy.insert(unit.id(), *unit.geo().point());
            self.xy_units
                .entry(*unit.geo().point())
                .or_default()
                .push(unit.id());
        }
    }

    // TODO: call when restored from backup
    pub fn reindex_tasks(&mut self, tasks: &Vec<TaskBox>) {
        self.city_tasks.clear();
        self.unit_tasks.clear();

        for task in tasks {
            match task.concern() {
                Concern::Unit(uuid) => self
                    .unit_tasks
                    .entry(uuid)
                    .or_default()
                    .push(task.context().id()),
                Concern::City(uuid) => self
                    .city_tasks
                    .entry(uuid)
                    .or_default()
                    .push(task.context().id()),
            }
        }
    }

    pub fn xy_cities(&self, window: &Window) -> Vec<Uuid> {
        if !DisplayStep::from_shape(window.shape()).include_cities() {
            return vec![];
        }

        let mut cities = vec![];
        for x in window.start_x()..window.end_x() {
            for y in window.start_y()..window.end_y() {
                if let Some(uuid) = self.xy_cities.get(&WorldPoint::new(x, y)) {
                    cities.push(*uuid);
                }
            }
        }

        cities
    }

    pub fn xy_units(&self, window: &Window) -> Vec<Uuid> {
        if !DisplayStep::from_shape(window.shape()).include_units() {
            return vec![];
        }

        let mut units = vec![];
        for x in window.start_x()..window.end_x() {
            for y in window.start_y()..window.end_y() {
                if let Some(uuids) = self.xy_units.get(&WorldPoint::new(x, y)) {
                    units.extend(uuids);
                }
            }
        }

        units
    }

    pub fn apply(&mut self, effects: &Vec<Effect>, cities: &Vec<City>, units: &Vec<Unit>) {
        let mut reindex_cities = false;
        let mut reindex_units = false;

        for effect in effects {
            match effect {
                Effect::State(effect) => match effect {
                    StateEffect::Client(_, _) => {}
                    StateEffect::Task(_, effect) => match effect {
                        TaskEffect::Push(task) => match task.concern() {
                            Concern::Unit(uuid) => self
                                .unit_tasks
                                .entry(uuid)
                                .or_default()
                                .push(task.context().id()),
                            Concern::City(uuid) => self
                                .city_tasks
                                .entry(uuid)
                                .or_default()
                                .push(task.context().id()),
                        },
                        TaskEffect::Finished(task) => match task.concern() {
                            Concern::Unit(uuid) => self
                                .unit_tasks
                                .entry(uuid)
                                .or_default()
                                .retain(|id| *id != task.context().id()),
                            Concern::City(uuid) => self
                                .city_tasks
                                .entry(uuid)
                                .or_default()
                                .retain(|id| *id != task.context().id()),
                        },
                    },
                    StateEffect::City(_, effect) => match effect {
                        CityEffect::New(_) | CityEffect::Remove(_) => {
                            reindex_cities = true;
                        }
                    },
                    StateEffect::Unit(_, effect) => match effect {
                        UnitEffect::New(_) | UnitEffect::Remove(_) => {
                            reindex_units = true;
                        }
                        UnitEffect::Move(unit, _) => {
                            self.xy_units
                                .entry(*unit.geo().point())
                                .or_default()
                                .retain(|id| id != &unit.id());

                            self.units_xy.remove(&unit.id());
                        }
                    },
                },
            }
        }

        if reindex_cities {
            self.reindex_cities(cities);
        }

        if reindex_units {
            self.reindex_units(units);
        }
    }

    pub fn uuid_cities(&self) -> &HashMap<Uuid, usize> {
        &self.cities_index
    }

    pub fn uuid_units(&self) -> &HashMap<Uuid, usize> {
        &self.units_index
    }

    pub fn city_tasks(&self, city_id: &Uuid) -> Vec<Uuid> {
        match self.city_tasks.get(city_id) {
            Some(uuids) => uuids.to_vec(),
            None => vec![],
        }
    }

    pub fn unit_tasks(&self, unit_id: &Uuid) -> Vec<Uuid> {
        match self.unit_tasks.get(unit_id) {
            Some(uuids) => uuids.to_vec(),
            None => vec![],
        }
    }
}
