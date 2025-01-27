use std::sync::RwLockReadGuard;

use common::{geo::WorldPoint, rules::RuleSetBox, world::reader::WorldReader};
use rand::Rng;
use thiserror::Error;

use crate::state::State;

pub type PlacerBox = Box<dyn for<'a> Placer<'a> + Sync + Send>;

pub trait Placer<'a> {
    fn startup(
        &self,
        rules: &'a RuleSetBox,
        state: &'a RwLockReadGuard<State>,
        world: &'a RwLockReadGuard<WorldReader>,
    ) -> Result<WorldPoint, PlacerError>;
}

#[derive(Debug, Error)]
pub enum PlacerError {
    #[error("No place found")]
    NoPlaceFound,
}

pub struct RandomPlacer;

impl<'a> Placer<'a> for RandomPlacer {
    fn startup(
        &self,
        rules: &'a RuleSetBox,
        state: &'a RwLockReadGuard<State>,
        world: &'a RwLockReadGuard<WorldReader>,
    ) -> Result<WorldPoint, PlacerError> {
        // TODO: something more smart than this
        for _ in 0..1000 {
            let x = rand::thread_rng().gen_range(0..world.width());
            let y = rand::thread_rng().gen_range(0..world.height());
            let point = WorldPoint::new(x, y);

            if let Some(tile) = world.tile(x, y) {
                if rules.can_be_startup(tile) {
                    // TODO: is free land
                    if state.index().xy_cities(&point).is_none()
                        && state
                            .index()
                            .xy_units(&point)
                            .map(|units| units.len())
                            .unwrap_or(0)
                            == 0
                    {
                        return Ok(point);
                    }
                }
            }
        }

        Err(PlacerError::NoPlaceFound)
    }
}
