use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Default, Clone)]
pub struct Context {
    stop: Arc<AtomicBool>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            stop: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn stop_is_required(&self) -> bool {
        self.stop.load(Ordering::Relaxed)
    }

    pub fn require_stop(&mut self) {
        self.stop.swap(true, Ordering::Relaxed);
    }
}
