use bevy::state::state::NextState;
use common::game::{nation::flag::Flag, server::ServerResume};

use crate::{menu::state::MenuStateResource, state::AppState};

pub fn react_server_resume_message(
    resume: &ServerResume,
    flag: &Option<Flag>,
    state: &mut MenuStateResource,
    next_state: &mut NextState<AppState>,
) {
    state.join.resume = Some(resume.clone());
    state.join.flag = *flag;

    if flag.is_some() {
        next_state.set(AppState::InGame);
    }
}
