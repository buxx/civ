use production::CityProductionTask;
use settle::Settle;

pub mod production;
pub mod settle;

pub enum TaskWrapper {
    Unit(UnitTaskWrapper),
    City(CityTaskWrapper),
}

pub enum UnitTaskWrapper {
    Settle(Settle),
}

pub enum CityTaskWrapper {
    Production(CityProductionTask),
}
