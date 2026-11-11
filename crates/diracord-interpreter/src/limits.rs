//! Execution limits and the meter that enforces them.

use crate::error::RuntimeError;
use diracord_diagnostics::{codes, Span};
use std::time::{Duration, Instant};

/// Bounds for one run. Defaults fit a small host (~100 MB for many bots).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Limits {
    /// Maximum statements + expressions evaluated.
    pub max_steps: u64,
    /// Maximum time spent running code (waiting for the host does not count).
    pub max_time: Duration,
    /// Maximum bytes held in variables (approximate).
    pub max_memory: usize,
    /// Maximum host actions (`reply`, `ban`...) per run.
    pub max_actions: u32,
    /// Maximum size of one value saved with `set`.
    pub max_saved_value: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            max_steps: 100_000,
            max_time: Duration::from_secs(3),
            max_memory: 4 * 1024 * 1024,
            max_actions: 50,
            max_saved_value: 64 * 1024,
        }
    }
}

/// Counts what a run uses.
pub(crate) struct Meter {
    pub(crate) steps: u64,
    pub(crate) actions: u32,
    start: Instant,
    host_time: Duration,
}

impl Meter {
    pub(crate) fn new() -> Self {
        Meter {
            steps: 0,
            actions: 0,
            start: Instant::now(),
            host_time: Duration::ZERO,
        }
    }

    /// One step of work at `span`.
    pub(crate) fn tick(&mut self, limits: &Limits, span: Span) -> Result<(), RuntimeError> {
        self.steps += 1;
        if self.steps > limits.max_steps {
            return Err(RuntimeError::limit(
                codes::E0205,
                format!(
                    "O código passou do limite de {} passos e foi parado",
                    limits.max_steps
                ),
                span,
                "procure um `while` que nunca termina",
            ));
        }
        if self.steps.is_multiple_of(256) && self.cpu_time() > limits.max_time {
            return Err(RuntimeError::limit(
                codes::E0204,
                format!(
                    "O código passou do limite de {:?} e foi parado",
                    limits.max_time
                ),
                span,
                "procure laços muito longos",
            ));
        }
        Ok(())
    }

    /// Time spent running code, excluding host calls.
    pub(crate) fn cpu_time(&self) -> Duration {
        self.start.elapsed().saturating_sub(self.host_time)
    }

    pub(crate) fn add_host_time(&mut self, d: Duration) {
        self.host_time += d;
    }
}
