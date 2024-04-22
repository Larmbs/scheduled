use chrono::{DateTime, Utc};
use std::cmp::Ordering;

/// Returns the current UTC time
fn get_current_time() -> DateTime<Utc> {
    Utc::now()
}

pub struct Event<F>
where
    F: FnOnce(DateTime<Utc>),
{
    time: DateTime<Utc>, // When it should activate
    func: F,             // What will be executed
}

impl<F> Event<F>
where
    F: FnOnce(DateTime<Utc>),
{
    /// Create a new event object
    pub fn new(time: DateTime<Utc>, func: F) -> Self {
        Event { time, func }
    }

    /// Returns true when timer has expired
    pub(crate) fn has_expired(&self) -> bool {
        // If the current time is greater or equal to the time specified output true
        get_current_time() >= self.time
    }

    /// Execute event (consumes object)
    pub(crate) fn execute(self) {
        (self.func)(self.time);
    }
}

impl<F> PartialEq for Event<F>
where
    F: FnOnce(DateTime<Utc>),
{
    /// Are events equal
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}
impl<F> Eq for Event<F> where F: FnOnce(DateTime<Utc>) {}
impl<F> PartialOrd for Event<F>
where
    F: FnOnce(DateTime<Utc>),
{
    /// Compares events
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<F> Ord for Event<F>
where
    F: FnOnce(DateTime<Utc>),
{
    /// Compares order of ids
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}
