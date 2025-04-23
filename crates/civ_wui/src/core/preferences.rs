use bevy::prelude::*;
use derive_more::Constructor;

use crate::user::preferences::Preferences;

#[derive(Debug, Deref, Resource, Constructor)]
pub struct PreferencesResource(pub Preferences);
