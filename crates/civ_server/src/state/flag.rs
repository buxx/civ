use common::game::{nation::flag::Flag, PlayerId};
use extfn::extfn;
use std::sync::RwLockReadGuard;

use crate::state::State;

#[extfn]
pub fn player_flag(self: &RwLockReadGuard<'_, State>, player_id: &PlayerId) -> Option<Flag> {
    self.clients()
        .player_state(player_id)
        .map(|s| s.flag())
        .cloned()
}
