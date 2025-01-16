use std::{
    fmt::Display,
    thread,
    time::{Duration, Instant},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub struct TimeoutReached<T: Display>(T);

impl<T: Display> Display for TimeoutReached<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Timeout reached: {}", self.0))
    }
}

pub fn wait<F: Fn() -> bool, E: Display>(
    kind: E,
    timeout: Duration,
    callback: F,
) -> Result<(), TimeoutReached<E>> {
    let start = Instant::now();
    while !callback() {
        if start.elapsed() >= timeout {
            return Err(TimeoutReached(kind));
        }
        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

pub fn wait_and_get<T, F: Fn() -> Option<T>, E: Display>(
    kind: E,
    timeout: Duration,
    callback: F,
) -> Result<T, TimeoutReached<E>> {
    let start = Instant::now();

    loop {
        if let Some(value) = callback() {
            return Ok(value);
        }

        if start.elapsed() >= timeout {
            return Err(TimeoutReached(kind));
        }

        thread::sleep(Duration::from_millis(100));
    }
}
