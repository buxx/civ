use bevy::prelude::*;
use common::game::{nation::flag::Flag, server::ServerResume};

use crate::{
    context::{Context, EntryPoint},
    network::ServerAddress,
};

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct MenuStateResource(pub MenuState);

#[derive(Debug)]
pub struct MenuState {
    pub screen: Screen,
    pub root: RootState,
    pub single: SingleState,
    pub join: JoinState,
    pub connecting: bool,
    pub taking_place: bool,
}

impl MenuState {
    pub fn from_context(context: &Context) -> Self {
        let screen = match context.entry_point() {
            EntryPoint::Root => Screen::Root,
            EntryPoint::Join => Screen::Join,
        };
        let root = RootState::default();
        let single = SingleState::default();
        let join = JoinState::from_context(context);

        Self {
            screen,
            root,
            single,
            join,
            connecting: false,
            taking_place: false,
        }
    }
}

#[derive(Debug)]
pub enum Screen {
    Root,
    Single,
    Join,
}
