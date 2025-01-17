use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use common::rules::RuleSetBox;

#[derive(Clone)]
pub struct Context {
    stop: Arc<AtomicBool>,
    connected: Arc<AtomicBool>,
    rule_set: RuleSetBox,
}

impl Context {
    pub fn new(stop: Arc<AtomicBool>, connected: Arc<AtomicBool>, rule_set: RuleSetBox) -> Self {
        Self {
            stop,
            connected,
            rule_set,
        }
    }

    pub fn stop_is_required(&self) -> bool {
        self.stop.load(Ordering::Relaxed)
    }

    pub fn require_stop(&self) {
        self.stop.swap(true, Ordering::Relaxed);
    }

    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Relaxed)
    }

    pub fn rule_set(&self) -> &RuleSetBox {
        &self.rule_set
    }
}
