#[derive(Default)]
pub struct Context {
    stop: bool,
}

impl Context {
    pub fn new() -> Self {
        Self { stop: false }
    }

    pub fn stop_is_required(&self) -> bool {
        self.stop
    }

    pub fn require_stop(&mut self) {
        self.stop = true
    }
}
