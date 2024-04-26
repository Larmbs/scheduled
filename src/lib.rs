use chrono::{DateTime, Utc};
/// Queue meant to better organize project and speed up searching
use queued_rust::{QueueType, SortedQueue};

/// Event module represents events
mod event;
pub use event::Event;
pub use chrono::Duration;

pub struct Scheduler<F>
where
    F: FnOnce(DateTime<Utc>),
{
    queue: SortedQueue<Event<F>>, // A sorted queue of events
}

impl<F> Scheduler<F>
where
    F: FnOnce(DateTime<Utc>),
{
    pub fn new() -> Self {
        Self {
            queue: SortedQueue::new(),
        }
    }

    /// Add a new event that activates at certain date
    pub fn schedule_date(&mut self, end_date: DateTime<Utc>, func: F) {
        let item = Event::new(end_date, func);
        self.queue.add(item);
    }

    /// Add a new event that activates a certain time from now
    pub fn schedule_wait_from_now(&mut self, wait: Duration, func: F) {
        let end_date = Utc::now() + wait;
        let item = Event::new(end_date, func);
        self.queue.add(item);
    }

    /// Add a new event that activates a certain time from now
    pub fn schedule_event(&mut self, event: Event<F>) {
        self.queue.add(event);
    }

    /// Checks if any event has pasts its time and then runs their events
    pub fn check(&mut self) -> bool {
        let mut has_expired = false;
        // Iterates over whole loop as long as there are items
        while let Some(item) = self.queue.first() {
            if item.has_expired() {
                // Checks if item has past expiration
                if let Some(item) = self.queue.pop() {
                    item.execute(); // Executes
                    has_expired = true;
                }
            } else {
                break; // Exit
            }
        }

        has_expired
    }
}

#[cfg(test)]
mod tests;
