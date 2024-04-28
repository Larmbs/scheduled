use super::{Event, EventFunc};
use queued_rust::SortedQueue;

use chrono::{DateTime, Duration, Utc};

pub struct Scheduler {
    queue: SortedQueue<Event>, // A sorted queue of events
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            queue: SortedQueue::new(false),
        }
    }

    /// Gets the count of events in queue
    pub fn get_event_count(&self) -> usize {
        self.queue.len()
    }

    /// Returns true if queue is full
    pub fn is_empty(&self) -> bool {
        self.queue.len() == 0
    }

    /// Add a new event that activates at certain date
    pub fn schedule_date(&mut self, end_date: DateTime<Utc>, func: EventFunc) {
        let item = Event::new(end_date, func);
        self.queue.add(item);
    }

    /// Add a new event that activates a certain time from now
    pub fn schedule_wait_from_now(&mut self, wait: Duration, func: EventFunc) {
        let end_date = Utc::now() + wait;
        let item = Event::new(end_date, func);
        self.queue.add(item);
    }

    /// Add a new event that activates a certain time from now
    pub fn schedule_event(&mut self, event: Event) {
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
