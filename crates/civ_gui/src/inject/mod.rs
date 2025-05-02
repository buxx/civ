use bevy::prelude::*;
use bon::Builder;
use common::game::slice::GameSlice as BaseGameSlice;

#[derive(Clone, Resource, Default, Builder)]
pub struct Injection {
    game_slice: Option<BaseGameSlice>,
}

impl Injection {
    pub fn game_slice(&self) -> Option<&BaseGameSlice> {
        self.game_slice.as_ref()
    }
}
