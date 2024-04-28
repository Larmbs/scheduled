/// Provides utilities for managing scheduled events.
mod scheduler;

/// Represents an event that can be scheduled.
mod event;

/// Defines the expected type for event functions.
pub type EventFunc = Box<dyn FnOnce(DateTime<Utc>)>;

use chrono::{DateTime, Utc};

pub use scheduler::Scheduler;
pub use event::Event;

/// Specifies a duration in time between events.
pub use chrono::Duration;

#[cfg(test)]
mod tests;
