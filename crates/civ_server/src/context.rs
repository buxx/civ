use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use common::rules::RuleSetBox;

#[derive(Clone)]
pub struct Context {
    rules: RuleSetBox,
    stop: Arc<AtomicBool>,
}

impl Context {
    pub fn new(rules: RuleSetBox) -> Self {
        Self {
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
}
