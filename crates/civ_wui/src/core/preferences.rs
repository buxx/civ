use bevy::prelude::*;
use derive_more::Constructor;

use crate::user::preferences::Preferences;

#[derive(Debug, Deref, DerefMut, Resource, Constructor)]
pub struct PreferencesResource(pub Preferences);
