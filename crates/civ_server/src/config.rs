use std::path::PathBuf;

#[derive(Clone)]
pub struct ServerConfig {
    snapshot_to: Option<PathBuf>,
}

impl ServerConfig {
    pub fn new(snapshot_to: Option<PathBuf>) -> Self {
        Self { snapshot_to }
    }

    pub fn snapshot_to(&self) -> Option<&PathBuf> {
        self.snapshot_to.as_ref()
    }
}
