use std::sync::RwLockReadGuard;

use crate::state::State;

pub mod city;
pub mod extractor;
pub mod task;
pub mod unit;

pub trait IntoClientModel<T> {
    fn into_client(self, state: &RwLockReadGuard<State>) -> T;
}
