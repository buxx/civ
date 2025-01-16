use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::Arguments;

#[derive(Debug, Clone)]
pub struct Context {
    args: Arguments,
    stop: Arc<AtomicBool>,
    connected: Arc<AtomicBool>,
}

impl Context {
    pub fn new(args: Arguments) -> Self {
        Self {
            args,
            stop: Default::default(),
            connected: Default::default(),
        }
    }

    pub fn args(&self) -> &Arguments {
        &self.args
    }

    pub fn stop(&self) -> &Arc<AtomicBool> {
        &self.stop
    }

    pub fn stop_is_requested(&self) -> bool {
        self.stop.load(Ordering::Relaxed)
    }

    pub fn connected(&self) -> &Arc<AtomicBool> {
        &self.connected
    }

    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Relaxed)
    }
}
