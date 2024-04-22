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
    id: usize,
    time: DateTime<Utc>,
    func: F,
}

impl<F> Event<F>
where
    F: FnOnce(DateTime<Utc>),
{
    pub fn new(id: usize, time: DateTime<Utc>, func: F) -> Self {
        Event { id, time, func }
    }

    /// Returns true when timer has expired
    pub(crate) fn has_expired(&self) -> bool {
        // If the current time is greater or equal to the time specified output true
        get_current_time() >= self.time
    }

    /// Execute event (consumes object)
    pub(crate) fn execute(self) {
        (self.func)(get_current_time());
    }
}

impl<F> PartialEq for Event<F>
where
    F: FnOnce(DateTime<Utc>),
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<F> Eq for Event<F> where F: FnOnce(DateTime<Utc>) {}
impl<F> PartialOrd for Event<F>
where
    F: FnOnce(DateTime<Utc>),
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<F> Ord for Event<F>
where
    F: FnOnce(DateTime<Utc>),
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
