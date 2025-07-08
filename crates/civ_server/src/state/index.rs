use std::collections::HashMap;

use common::{
    game::{city::CityId, nation::flag::Flag, unit::UnitId},
    geo::{Geo, WorldPoint},
    space::window::Window,
};

use crate::{
    effect::{CityEffect, Effect, StateEffect, TaskEffect, TasksEffect, UnitEffect},
    game::{city::City, unit::Unit},
    snapshot::Snapshot,
    task::{Concern, TaskBox, TaskId},
};

#[derive(Default)]
pub struct Index {
    cities_index: HashMap<CityId, usize>,
    units_index: HashMap<UnitId, usize>,
    xy_cities: HashMap<WorldPoint, CityId>,
    xy_units: HashMap<WorldPoint, Vec<UnitId>>,
    flag_cities: HashMap<Flag, CityId>,
    flag_units: HashMap<Flag, Vec<UnitId>>,
    city_tasks: HashMap<CityId, Vec<TaskId>>,
    unit_tasks: HashMap<UnitId, Vec<TaskId>>,
}

impl Index {
    pub fn reindex_cities(&mut self, cities: &[City]) {
        self.cities_index.clear();
        self.xy_cities.clear();
        self.flag_cities.clear();

        for (i, city) in cities.iter().enumerate() {
            self.cities_index.insert(*city.id(), i);
            self.xy_cities.insert(*city.geo().point(), *city.id());
            self.flag_cities.insert(*city.flag(), *city.id());
        }
    }

    pub fn reindex_units(&mut self, units: &[Unit]) {
        self.units_index.clear();
        self.xy_units.clear();
        self.flag_units.clear();

        for (i, unit) in units.iter().enumerate() {
            self.units_index.insert(*unit.id(), i);
            self.xy_units
                .entry(*unit.geo().point())
                .or_default()
                .push(*unit.id());
            self.flag_units
                .entry(*unit.flag())
                .or_default()
                .push(*unit.id());
        }
    }

    pub fn reindex_tasks(&mut self, tasks: &Vec<TaskBox>) {
        self.city_tasks.clear();
        self.unit_tasks.clear();

        for task in tasks {
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
    }

    pub fn xy_cities(&self, point: &WorldPoint) -> Option<&CityId> {
        self.xy_cities.get(point)
    }

    pub fn window_cities(&self, window: &Window) -> Vec<CityId> {
        if !window.step().include_cities() {
            return vec![];
        }

        let mut cities = vec![];
        for x in window.start().x..window.end().x {
            for y in window.start().y..window.end().y {
                if let Some(uuid) = self.xy_cities.get(&WorldPoint::new(x as u64, y as u64)) {
                    cities.push(*uuid);
                }
            }
        }

        cities
    }

    pub fn xy_units(&self, point: &WorldPoint) -> Option<&Vec<UnitId>> {
        self.xy_units.get(point)
    }

    pub fn window_units(&self, window: &Window) -> Vec<UnitId> {
        if !window.step().include_units() {
            return vec![];
        }

        let mut units = vec![];
        for x in window.start().x..window.end().x {
            for y in window.start().y..window.end().y {
                if let Some(uuids) = self.xy_units.get(&WorldPoint::new(x as u64, y as u64)) {
                    units.extend(uuids);
                }
            }
        }

        units
    }

    pub fn apply(&mut self, effects: &Vec<Effect>, cities: &[City], units: &[Unit]) {
        let mut reindex_cities = false;
        let mut reindex_units = false;

        for effect in effects {
            match effect {
                Effect::State(effect) => match effect {
                    StateEffect::IncrementGameFrame => {}
                    StateEffect::Clients(_) => {}
                    StateEffect::Client(_, _) => {}
                    StateEffect::Tasks(effect) => match effect {
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
                    StateEffect::Task(_, effect) => match effect {
                        TaskEffect::Push(task) => self.apply_new_task(task),
                        TaskEffect::Finished(task) => {
                            self.apply_remove_task(task.context().id(), &task.concern())
                        }
                        TaskEffect::Remove(uuid, concern) => self.apply_remove_task(uuid, concern),
                    },
                    StateEffect::City(_, effect) => match effect {
                        CityEffect::New(_) | CityEffect::Remove(_) => {
                            // FIXME: probably too much impacting ? Do only necessary here ?
                            reindex_cities = true;
                        }
                        CityEffect::Replace(_) => {
                            // Tasks already added/removed by TasksEffect
                        }
                    },
                    StateEffect::Unit(_, effect) => match effect {
                        UnitEffect::New(_) | UnitEffect::Remove(_) => {
                            // FIXME: probably too much impacting ? Do only necessary here ?
                            reindex_units = true;
                        }
                        UnitEffect::Replace(_unit) => {
                            //
                        }
                    },
                    StateEffect::Testing => {}
                },
                Effect::Shines(_) => {}
                Effect::Action(_) => {}
            }
        }

        if reindex_cities {
            self.reindex_cities(cities);
        }

        if reindex_units {
            self.reindex_units(units);
        }
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

    pub fn uuid_cities(&self) -> &HashMap<CityId, usize> {
        &self.cities_index
    }

    pub fn uuid_cities_mut(&mut self) -> &mut HashMap<CityId, usize> {
        &mut self.cities_index
    }

    pub fn uuid_units(&self) -> &HashMap<UnitId, usize> {
        &self.units_index
    }

    pub fn city_tasks(&self, city_id: &CityId) -> Vec<TaskId> {
        match self.city_tasks.get(city_id) {
            Some(uuids) => uuids.to_vec(),
            None => vec![],
        }
    }

    pub fn unit_tasks(&self, unit_id: &UnitId) -> Vec<TaskId> {
        match self.unit_tasks.get(unit_id) {
            Some(uuids) => uuids.to_vec(),
            None => vec![],
        }
    }
}

impl From<&Snapshot> for Index {
    fn from(value: &Snapshot) -> Self {
        let mut index = Self::default();

        index.reindex_cities(value.cities());
        index.reindex_units(value.units());

        let tasks: Vec<TaskBox> = value.tasks().iter().map(|bx| bx.boxed()).collect();
        index.reindex_tasks(&tasks);

        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::game::unit::_Factori_Builder_Unit;

    #[test]
    fn test_apply_unit_replace() {
        // Given
        let mut index = Index::default();
        let unit = create!(Unit);
        index.reindex_units(&[unit.clone()]);

        // When
        index.apply(
            &vec![Effect::State(StateEffect::Unit(
                *unit.id(),
                UnitEffect::Replace(unit.clone()),
            ))],
            &[],
            &[unit.clone()],
        );

        // Then
        assert_eq!(
            index.xy_units.get(unit.geo().point()),
            Some(&vec![*unit.id()])
        );
    }
}
