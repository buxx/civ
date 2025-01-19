use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use common::rules::RuleSetBox;

use crate::config::ServerConfig;

#[derive(Clone)]
pub struct Context {
    config: ServerConfig,
    rules: RuleSetBox,
    stop: Arc<AtomicBool>,
}

impl Context {
    pub fn new(rules: RuleSetBox, config: ServerConfig) -> Self {
        Self {
            config,
            rules,
            stop: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn stop_is_required(&self) -> bool {
        self.stop.load(Ordering::Relaxed)
    }

    pub fn require_stop(&self) {
        self.stop.swap(true, Ordering::Relaxed);
    }

    pub fn rules(&self) -> &RuleSetBox {
        &self.rules
    }

    pub fn config(&self) -> &ServerConfig {
        &self.config
    }
}
