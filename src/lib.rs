use chrono::{DateTime, Utc};

mod event;
mod scheduler;

/// Expected event function type
pub type EventFunc = Box<dyn FnOnce(DateTime<Utc>)>;

pub use chrono::Duration;
pub use event::Event; // Create a event type
pub use scheduler::Scheduler; // Allows you to schedule events // Specify a duration in time between events

#[cfg(test)]
mod tests;
