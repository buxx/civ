use std::collections::HashMap;

use common::space::window::{DisplayStep, Window};
use uuid::Uuid;

use crate::{
    game::{city::City, physics::Physics, unit::Unit},
    task::effect::IndexEffect,
};

#[derive(Default)]
pub struct Index {
    uuid_cities: HashMap<Uuid, usize>,
    uuid_units: HashMap<Uuid, usize>,
    xy_cities: HashMap<(u64, u64), Uuid>,
    xy_units: HashMap<(u64, u64), Vec<Uuid>>,
}

impl Index {
    pub fn refresh_cities_indexes(&mut self, cities: &Vec<City>) {
        self.uuid_cities.clear();
        for (i, city) in cities.iter().enumerate() {
            self.uuid_cities.insert(city.id(), i);
        }
    }

    pub fn refresh_units_indexes(&mut self, units: &Vec<Unit>) {
        self.uuid_units.clear();
        for (i, unit) in units.iter().enumerate() {
            self.uuid_units.insert(unit.id(), i);
        }
    }

    pub fn xy_cities(&self, window: &Window) -> Vec<Uuid> {
        if !DisplayStep::from_shape(window.shape()).include_cities() {
            return vec![];
        }

        let mut cities = vec![];
        for x in window.start_x()..window.end_x() {
            for y in window.start_y()..window.end_y() {
                if let Some(uuid) = self.xy_cities.get(&(x, y)) {
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
                if let Some(uuids) = self.xy_units.get(&(x, y)) {
                    units.extend(uuids);
                }
            }
        }

        units
    }

    pub fn apply(&mut self, effects: Vec<IndexEffect>, cities: &Vec<City>, units: &Vec<Unit>) {
        let mut refresh_cities_index = false;
        let mut refresh_units_index = false;

        for effect in effects {
            match effect {
                IndexEffect::NewlyCity(city) => {
                    self.xy_cities.insert(city.physics().xy(), city.id());
                    refresh_cities_index = true;
                }
                IndexEffect::RemovedCity(city) => {
                    self.xy_cities
                        .remove(&city.physics().xy())
                        .expect("Index integrity");
                    refresh_cities_index = true;
                }
                IndexEffect::NewlyUnit(unit) => {
                    self.xy_units
                        .entry(unit.physics().xy())
                        .or_default()
                        .push(unit.id());
                    refresh_units_index = true;
                }
                IndexEffect::RemovedUnit(unit) => {
                    self.xy_units
                        .entry(unit.physics().xy())
                        .or_default()
                        .retain(|id| id != &unit.id());
                    refresh_cities_index = true;
                }
                IndexEffect::MovedUnit(unit, from_, to_) => {
                    self.xy_units
                        .entry(from_)
                        .or_default()
                        .retain(|id| id != &unit.id());
                    self.xy_units.entry(to_).or_default().push(unit.id());
                }
            }
        }

        if refresh_cities_index {
            self.refresh_cities_indexes(cities);
        }

        if refresh_units_index {
            self.refresh_units_indexes(units);
        }
    }

    pub fn uuid_cities(&self) -> &HashMap<Uuid, usize> {
        &self.uuid_cities
    }

    pub fn uuid_units(&self) -> &HashMap<Uuid, usize> {
        &self.uuid_units
    }
}
