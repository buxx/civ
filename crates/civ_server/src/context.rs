use common::rules::RuleSet;

// FIXME: replace Arc Mutex Context by a context with atomic bool, which can be cloned easly
pub struct Context {
    rules: Box<dyn RuleSet + Send>,
    stop: bool,
}

impl Context {
    pub fn new(rules: Box<dyn RuleSet + Send>) -> Self {
        Self { rules, stop: false }
    }

    pub fn stop_is_required(&self) -> bool {
        self.stop
    }

    pub fn require_stop(&mut self) {
        self.stop = true
    }

    pub fn rules(&self) -> &Box<dyn RuleSet + Send> {
        &self.rules
    }
}
