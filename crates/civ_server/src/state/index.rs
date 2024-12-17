use std::collections::HashMap;

use common::{
    geo::Geo,
    space::window::{DisplayStep, Window},
};
use uuid::Uuid;

use crate::{
    game::{city::City, unit::Unit},
    task::effect::IndexEffect,
};

#[derive(Default)]
pub struct Index {
    cities_index: HashMap<Uuid, usize>,
    units_index: HashMap<Uuid, usize>,
    cities_xy: HashMap<Uuid, (u64, u64)>,
    units_xy: HashMap<Uuid, (u64, u64)>,
    xy_cities: HashMap<(u64, u64), Uuid>,
    xy_units: HashMap<(u64, u64), Vec<Uuid>>,
}

impl Index {
    pub fn refresh_all_cities(&mut self, cities: &Vec<City>) {
        self.cities_index.clear();
        self.cities_xy.clear();

        for (i, city) in cities.iter().enumerate() {
            self.cities_index.insert(city.id(), i);
            self.cities_xy.insert(city.id(), city.geo().xy());
        }
    }

    pub fn refresh_all_units(&mut self, units: &Vec<Unit>) {
        self.units_index.clear();
        self.units_xy.clear();

        for (i, unit) in units.iter().enumerate() {
            self.units_index.insert(unit.id(), i);
            self.units_xy.insert(unit.id(), unit.geo().xy());
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
                IndexEffect::NewCity(city) => {
                    self.xy_cities.insert(city.geo().xy(), city.id());
                    refresh_cities_index = true;
                }
                IndexEffect::RemovedCity(uuid) => {
                    let city_xy = self.cities_xy.get(&uuid).expect("Index integrity");
                    self.xy_cities.remove(&city_xy).expect("Index integrity");
                    refresh_cities_index = true;
                }
                IndexEffect::NewUnit(unit) => {
                    self.xy_units
                        .entry(unit.geo().xy())
                        .or_default()
                        .push(unit.id());
                    refresh_units_index = true;
                }
                IndexEffect::RemovedUnit(uuid) => {
                    let unit_xy = self.units_xy.get(&uuid).expect("Index integrity");
                    self.xy_units
                        .entry(*unit_xy)
                        .or_default()
                        .retain(|id| id != &uuid);
                    refresh_cities_index = true;
                }
                IndexEffect::MovedUnit(uuid, to_) => {
                    let old_unit_xy = self.units_xy.get(&uuid).expect("Index integrity");
                    self.xy_units
                        .entry(*old_unit_xy)
                        .or_default()
                        .retain(|id| id != &uuid);
                    self.xy_units.entry(to_).or_default().push(uuid);
                }
            }
        }

        if refresh_cities_index {
            self.refresh_all_cities(cities);
        }

        if refresh_units_index {
            self.refresh_all_units(units);
        }
    }

    pub fn uuid_cities(&self) -> &HashMap<Uuid, usize> {
        &self.cities_index
    }

    pub fn uuid_units(&self) -> &HashMap<Uuid, usize> {
        &self.units_index
    }
}
