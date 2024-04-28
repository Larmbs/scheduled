use super::EventFunc;
use chrono::{DateTime, Utc};
use std::cmp::Ordering;

/// Returns the current UTC time.
fn get_current_time() -> DateTime<Utc> {
    Utc::now()
}

/// Represents an event that is scheduled to occur at a specific time.
pub struct Event {
    time: DateTime<Utc>, // When it should activate
    func: EventFunc,     // What will be executed
}

impl Event {
    /// Create a new event object.
    pub fn new(time: DateTime<Utc>, func: EventFunc) -> Self {
        Event { time, func }
    }

    /// Returns true when the event's time has expired.
    pub(crate) fn has_expired(&self) -> bool {
        // If the current time is greater or equal to the time specified, output true.
        get_current_time() >= self.time
    }

    /// Execute the event (consumes the object).
    pub(crate) fn execute(self) {
        (self.func)(self.time);
    }
}

impl PartialEq for Event {
    /// Determines if two events are equal based on their time.
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    /// Compares events based on their time.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    /// Compares the order of events based on their time.
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}
