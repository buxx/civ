use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use common::rules::RuleSet;

#[derive(Clone)]
pub struct Context {
    rules: Box<dyn RuleSet + Send + Sync>,
    stop: Arc<AtomicBool>,
}

impl Context {
    pub fn new(rules: Box<dyn RuleSet + Send + Sync>) -> Self {
        Self {
            rules,
            stop: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn stop_is_required(&self) -> bool {
        self.stop.load(Ordering::Relaxed)
    }

    pub fn require_stop(&mut self) {
        self.stop.swap(true, Ordering::Relaxed);
    }

    pub fn rules(&self) -> &Box<dyn RuleSet + Send + Sync> {
        &self.rules
    }
}
