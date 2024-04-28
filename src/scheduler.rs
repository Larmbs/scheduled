use super::{Event, EventFunc};
use queued_rust::SortedQueue;

use chrono::{DateTime, Duration, Utc};

/// A scheduler for managing and executing events based on their activation time.
pub struct Scheduler {
    queue: SortedQueue<Event>, // A sorted queue of events
}

impl Scheduler {
    /// Constructs a new Scheduler instance.
    pub fn new() -> Self {
        Self {
            queue: SortedQueue::new(false),
        }
    }

    /// Gets the count of events in the scheduler's queue.
    pub fn get_event_count(&self) -> usize {
        self.queue.len()
    }

    /// Returns true if the scheduler's queue is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.len() == 0
    }

    /// Adds a new event that activates at a certain date.
    pub fn schedule_date(&mut self, end_date: DateTime<Utc>, func: EventFunc) {
        let item = Event::new(end_date, func);
        self.queue.add(item);
    }

    /// Adds a new event that activates after a certain duration from the current time.
    pub fn schedule_wait_from_now(&mut self, wait: Duration, func: EventFunc) {
        let end_date = Utc::now() + wait;
        let item = Event::new(end_date, func);
        self.queue.add(item);
    }

    /// Adds a new event to the scheduler.
    pub fn schedule_event(&mut self, event: Event) {
        self.queue.add(event);
    }

    /// Checks if any event in the scheduler's queue has expired and executes them.
    ///
    /// Returns true if at least one event has expired and executed, false otherwise.
    pub fn check(&mut self) -> bool {
        let mut has_expired = false;
        // Iterates over the queue as long as there are items
        while let Some(item) = self.queue.first() {
            if item.has_expired() {
                // Checks if the item has expired
                if let Some(item) = self.queue.pop() {
                    item.execute(); // Executes the event
                    has_expired = true;
                }
            } else {
                break; // Exit loop if no more expired events
            }
        }

        has_expired
    }
}
