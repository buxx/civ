use clients::Clients;
use common::{
    game::{
        city::CityId,
        nation::flag::Flag,
        server::ServerResume,
        slice::{ClientCity, ClientUnit},
        unit::{TaskType, UnitId},
        GameFrame, PlayerId,
    },
    geo::{Geo, GeoVec},
    network::Client,
    rules::RuleSetBox,
    space::{window::Window, CityVec2dIndex, D2Size, UnitVec2dIndex},
    utils::Vec2d,
    world::slice::Slice,
};
use index::Index;
use log::error;
use thiserror::Error;

use crate::{
    effect::{CityEffect, Effect, StateEffect, TaskEffect, TasksEffect, UnitEffect},
    game::{city::City, unit::Unit, IntoClientModel},
    snapshot::Snapshot,
    task::{Task, TaskBox, TaskId},
};

pub mod clients;
pub mod flag;
pub mod index;

pub struct State {
    frame_i: GameFrame,
    clients: Clients,
    index: Index,
    tasks: Vec<TaskBox>,
    // Don't store a Vec2d<Box<City>> to not allocate useless memory
    cities: Vec2d<Box<City>>,
    cities_count: usize,
    units: Vec2d<Vec<Unit>>,
    units_count: usize,
    world_size: D2Size,
    testing: u64,
}

impl State {
    pub fn empty(world_size: D2Size) -> Self {
        Self {
            frame_i: GameFrame(0),
            clients: Clients::default(),
            index: Index::default(),
            tasks: vec![],
            cities: Vec2d::from(world_size, Vec::<City>::new()),
            cities_count: 0,
            units: Vec2d::from(world_size, Vec::<GeoVec<Unit>>::new()),
            units_count: 0,
            world_size,
            testing: 0,
        }
    }

    pub fn build_from(
        frame_i: GameFrame,
        world_size: D2Size,
        clients: Clients,
        cities: Vec<City>,
        units: Vec<GeoVec<Unit>>,
        tasks: &Vec<Box<dyn Task>>,
    ) -> Self {
        let cities_count = cities.len();
        let cities = Vec2d::from(world_size, cities);
        let units_count = units.len();
        let units = Vec2d::from(world_size, units);
        let index = Index::build_from(&cities, &units, tasks);
        let tasks: Vec<TaskBox> = tasks.iter().map(|bx| bx.boxed()).collect();

        Self::new(
            frame_i,
            clients,
            index,
            tasks,
            cities,
            cities_count,
            units,
            units_count,
            world_size,
            0,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        frame_i: GameFrame,
        clients: Clients,
        index: Index,
        tasks: Vec<TaskBox>,
        cities: Vec2d<Box<City>>,
        cities_count: usize,
        units: Vec2d<Vec<Unit>>,
        units_count: usize,
        world_size: D2Size,
        testing: u64,
    ) -> Self {
        // let units_: Vec<GeoVec<Unit>> = units
        //     .into_iter()
        //     .fold(
        //         HashMap::new(),
        //         |mut map: HashMap<GeoContext, Vec<Unit>>, unit| {
        //             map.entry(*unit.geo()).or_default().push(unit);
        //             map
        //         },
        //     )
        //     .into_iter()
        //     .map(GeoVec::from)
        //     .collect();

        Self {
            frame_i,
            clients,
            index,
            tasks,
            cities,
            cities_count,
            units,
            units_count,
            // cities: Vec2d::from(world_size, cities),
            // units: Vec2d::from(world_size, units_),
            world_size,
            testing,
        }
    }

    pub fn frame(&self) -> &GameFrame {
        &self.frame_i
    }

    pub fn tasks(&self) -> &Vec<TaskBox> {
        &self.tasks
    }

    pub fn tasks_mut(&mut self) -> &mut Vec<TaskBox> {
        &mut self.tasks
    }

    pub fn clients(&self) -> &Clients {
        &self.clients
    }

    pub fn clients_mut(&mut self) -> &mut Clients {
        &mut self.clients
    }

    pub fn increment_frame(&mut self) {
        self.frame_i += GameFrame(1);
    }

    pub fn apply(&mut self, effects: &Vec<Effect>) {
        let mut remove_tasks = vec![];

        for effect in effects {
            match effect {
                Effect::State(effect) => match effect {
                    StateEffect::IncrementGameFrame => {
                        self.increment_frame();
                    }
                    StateEffect::Clients(effect) => {
                        self.clients.apply(effect).unwrap();
                    }
                    StateEffect::Client(client, effect) => {
                        self.clients.apply_client(client, effect).unwrap();
                    }
                    StateEffect::Task(uuid, effect) => match effect {
                        TaskEffect::Push(task) => self.tasks.push(task.clone()),
                        TaskEffect::Finished(_) => remove_tasks.push(uuid),
                        TaskEffect::Remove(_, _) => remove_tasks.push(uuid),
                    },
                    StateEffect::Tasks(effect) => match effect {
                        TasksEffect::Remove(tasks) => {
                            remove_tasks
                                .extend(tasks.iter().map(|(i, _)| i).collect::<Vec<&TaskId>>());
                        }
                        TasksEffect::Add(tasks) => self.tasks.extend(tasks.clone()),
                    },
                    StateEffect::City(_, effect) => match effect {
                        CityEffect::New(city) => {
                            *self.cities.get_by_point_mut(*city.geo().point()) =
                                Some(Box::new(city.clone()));
                            self.cities_count += 1;
                        }
                        CityEffect::Replace(city) => {
                            *self.find_city_mut(city.id()).unwrap() = city.clone();
                        }
                        CityEffect::Remove(city) => {
                            *self.cities.get_by_point_mut(*city.geo().point()) = None;
                            self.cities_count -= 1;
                        }
                    },
                    StateEffect::Unit(unit_id, effect) => match effect {
                        UnitEffect::New(unit) => {
                            self.units_count += 1;

                            if let Some(units) = self.units.get_by_point_mut(*unit.geo().point()) {
                                units.push(unit.clone());
                            } else {
                                *self.units.get_by_point_mut(*unit.geo().point()) =
                                    Some(vec![unit.clone()]);
                            }
                        }
                        UnitEffect::Remove(unit) => {
                            self.units_count -= 1;

                            if let Some(units) = self.units.get_by_point_mut(*unit.geo().point()) {
                                units.retain(|u| u.id() != unit.id());
                                if units.is_empty() {
                                    *self.units.get_by_point_mut(*unit.geo().point()) = None;
                                }
                            }
                        }
                        UnitEffect::Replace(unit) => {
                            *self.find_unit_mut(unit_id).unwrap() = unit.clone();
                        }
                    },
                    StateEffect::Testing => {
                        self.testing += 1;
                    }
                },
                Effect::Shines(_) => {}
            }
        }

        if !remove_tasks.is_empty() {
            // TODO: this is not a good performance way (idea: transport tasks index in tick)
            self.tasks
                .retain(|task| !remove_tasks.contains(&task.context().id()));
        }

        // Update index must be after because based on &self.cities and &self.units
        self.index.apply(effects, &self.cities, &self.units);
    }

    pub fn cities(&self) -> &Vec2d<Box<City>> {
        &self.cities
    }

    pub fn client_cities_slice(&self, window: &Window) -> Slice<Option<ClientCity>> {
        let cities = self
            .cities
            .slice(window)
            .into_iter()
            .map(|c| c.map(|c| c.into_client(self)))
            .collect();
        Slice::new(
            *window.start(),
            (window.end().x - window.start().x + 1) as u64,
            (window.end().y - window.start().y + 1) as u64,
            cities,
        )
    }

    pub fn city(&self, index: CityVec2dIndex, city_id: &CityId) -> Result<&City, StateError> {
        if let Some(city) = self.cities.get(index.0) {
            if city.id() == city_id {
                return Ok(city);
            }
        }

        Err(StateError::NotFound(NotFound::City(index, *city_id)))
    }

    pub fn city_mut(
        &mut self,
        index: CityVec2dIndex,
        city_id: &CityId,
    ) -> Result<&mut City, StateError> {
        if let Some(city) = self.cities.get_mut(index.0) {
            if city.id() == city_id {
                return Ok(city);
            }
        }

        Err(StateError::NotFound(NotFound::City(index, *city_id)))
    }

    pub fn find_city(&self, city_id: &CityId) -> Result<&City, StateError> {
        let unit_index = self
            .index()
            .cities_index()
            .get(city_id)
            .ok_or(StateError::NoLongerExist(NoLongerExist::City(*city_id)))?;
        self.city(*unit_index, city_id)
    }

    pub fn find_city_mut(&mut self, city_id: &CityId) -> Result<&mut City, StateError> {
        let unit_index = self
            .index()
            .cities_index()
            .get(city_id)
            .ok_or(StateError::NoLongerExist(NoLongerExist::City(*city_id)))?;
        self.city_mut(*unit_index, city_id)
    }

    // TODO: utility of unit_id ?!
    pub fn unit(&self, index: UnitVec2dIndex, unit_id: &UnitId) -> Result<&Unit, StateError> {
        if let Some(units) = self.units.get(index.0) {
            if let Some(unit) = units.get(index.1) {
                if unit.id() == unit_id {
                    return Ok(unit);
                }
            }
        }

        Err(StateError::NotFound(NotFound::Unit(index, *unit_id)))
    }

    pub fn unit_mut(
        &mut self,
        index: UnitVec2dIndex,
        unit_id: &UnitId,
    ) -> Result<&mut Unit, StateError> {
        if let Some(units) = self.units.get_mut(index.0) {
            if let Some(unit) = units.get_mut(index.1) {
                if unit.id() == unit_id {
                    return Ok(unit);
                }
            }
        }

        Err(StateError::NotFound(NotFound::Unit(index, *unit_id)))
    }

    pub fn find_unit(&self, unit_id: &UnitId) -> Result<&Unit, StateError> {
        let unit_index = self
            .index()
            .units_index()
            .get(unit_id)
            .ok_or(StateError::NoLongerExist(NoLongerExist::Unit(*unit_id)))?;
        self.unit(*unit_index, unit_id)
    }

    pub fn find_unit_mut(&mut self, unit_id: &UnitId) -> Result<&mut Unit, StateError> {
        let unit_index = self
            .index()
            .units_index()
            .get(unit_id)
            .ok_or(StateError::NoLongerExist(NoLongerExist::Unit(*unit_id)))?;
        self.unit_mut(*unit_index, unit_id)
    }

    pub fn units(&self) -> &Vec2d<Vec<Unit>> {
        &self.units
    }

    pub fn client_units_slice(&self, window: &Window) -> Slice<Option<Vec<ClientUnit>>> {
        let units = self
            .units
            .slice(window)
            .into_iter()
            .map(|u| u.map(|u| u.into_iter().map(|u| u.into_client(self)).collect()))
            .collect();
        Slice::new(
            *window.start(),
            (window.end().x - window.start().x + 1) as u64,
            (window.end().y - window.start().y + 1) as u64,
            units,
        )
    }

    pub fn units_mut(&mut self) -> &mut Vec2d<Vec<Unit>> {
        &mut self.units
    }

    pub fn index(&self) -> &Index {
        &self.index
    }

    pub fn index_mut(&mut self) -> &mut Index {
        &mut self.index
    }

    pub fn testing(&self) -> u64 {
        self.testing
    }

    pub fn client_flag(&self, client: &Client) -> Result<&Flag, StateError> {
        Ok(self
            .clients
            .player_state(client.player_id())
            .ok_or(StateError::NoLongerExist(NoLongerExist::Player(
                *client.player_id(),
            )))?
            .flag())
    }

    pub fn server_resume(&self, rules: &RuleSetBox) -> ServerResume {
        let flags = self.clients.flags();
        ServerResume::new(rules.clone().into(), flags)
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot::from(self)
    }

    /// Replace found task by given considering its type as differentiators.
    pub fn with_replaced_task_type(mut self, type_: TaskType, task: TaskBox) -> State {
        self.tasks.retain(|t| t.type_() != type_);
        self.tasks.push(task);
        self
    }

    pub fn world_size(&self) -> D2Size {
        self.world_size
    }

    pub fn cities_count(&self) -> usize {
        self.cities_count
    }

    pub fn units_count(&self) -> usize {
        self.units_count
    }
}

#[derive(Error, Debug)]
pub enum StateError {
    #[error("Not found: {0}")]
    NotFound(NotFound),
    #[error("No longer exist: {0}")]
    NoLongerExist(NoLongerExist),
}

#[derive(Error, Debug)]
pub enum NoLongerExist {
    #[error("No city for uuid {0}")]
    City(CityId),
    #[error("No unit for uuid {0}")]
    Unit(UnitId),
    #[error("No player state for uuid {0}")]
    Player(PlayerId),
}

#[derive(Error, Debug)]
pub enum NotFound {
    #[error("No city for index {0} and uuid {1}")]
    City(CityVec2dIndex, CityId),
    #[error("No unit for index {0} and uuid {1}")]
    Unit(UnitVec2dIndex, UnitId),
}

#[cfg(test)]
mod test {
    use common::{
        game::unit::UnitType,
        geo::{GeoContext, ImaginaryWorldPoint, WorldPoint},
        space::window::DisplayStep,
    };

    use super::*;

    pub fn build_unit(geo: GeoContext) -> Unit {
        Unit::builder()
            .id(UnitId::default())
            .geo(geo)
            .type_(UnitType::Warriors)
            .flag(Flag::Abkhazia)
            .can(vec![])
            .build()
    }

    #[test]
    fn test_units_slice() {
        // Given
        let frame = GameFrame(0);
        let size = D2Size::new(5, 5);
        let clients = Clients::default();
        let cities = vec![];
        let unit1_geo = GeoContext::new(WorldPoint::new(2, 2));
        let unit = build_unit(unit1_geo);
        let units = vec![GeoVec::new(unit1_geo, vec![unit.clone()])];
        let tasks = vec![];
        let state = State::build_from(frame, size, clients, cities, units, &tasks);
        let unitc: ClientUnit = unit.into_client(&state);

        // When/Then
        let window_start = ImaginaryWorldPoint::new(0, 0);
        let window_end = ImaginaryWorldPoint::new(4, 4);
        let window = Window::new(window_start, window_end, DisplayStep::Close);
        let slice = state.client_units_slice(&window);

        assert_eq!(
            slice.items(),
            &[
                //
                None,
                None,
                None,
                None,
                None,
                //
                None,
                None,
                None,
                None,
                None,
                //
                None,
                None,
                Some(vec![unitc.clone()]),
                None,
                None,
                //
                None,
                None,
                None,
                None,
                None,
                //
                None,
                None,
                None,
                None,
                None,
            ],
        );

        // When/Then
        let window_start = ImaginaryWorldPoint::new(0, 2);
        let window_end = ImaginaryWorldPoint::new(4, 4);
        let window = Window::new(window_start, window_end, DisplayStep::Close);
        let slice = state.client_units_slice(&window);

        assert_eq!(
            slice.items(),
            &[
                //
                None,
                None,
                Some(vec![unitc.clone()]),
                None,
                None,
                //
                None,
                None,
                None,
                None,
                None,
                //
                None,
                None,
                None,
                None,
                None,
            ],
        );

        // When/Then
        let window_start = ImaginaryWorldPoint::new(2, 2);
        let window_end = ImaginaryWorldPoint::new(4, 4);
        let window = Window::new(window_start, window_end, DisplayStep::Close);
        let slice = state.client_units_slice(&window);

        assert_eq!(
            slice.items(),
            &[
                //
                Some(vec![unitc]),
                None,
                None,
                //
                None,
                None,
                None,
                //
                None,
                None,
                None,
            ],
        );
    }
}
