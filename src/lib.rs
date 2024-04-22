use queued_rust::SortedQueue;
use chrono::{DateTime, Utc};


mod event;
pub use event::Event;


pub struct Scheduler<F> where F: FnOnce(DateTime<Utc>) {
    queue: SortedQueue<Event<F>>, // A sorted queue of events
    counter: usize,
}

impl<F> Scheduler<F> where F: FnOnce(DateTime<Utc>) {
    pub fn new() -> Self {
        Self {
            queue: SortedQueue::new(),
            counter: 0,
        }
    }

    pub fn add(&mut self, end_date: DateTime<Utc>, func: F) {
        let item = Event::new(self.counter, end_date, func);
        self.counter += 1;
        self.queue.add(item);
    }

    /// Checks if any event has pasts its time and then runs their events
    pub fn check(&mut self) {
        // Iterates over whole loop as long as there are items
        while let Some(item) = self.queue.first() {
            if item.has_expired() { // Checks if item has past expiration
                if let Some(item) = self.queue.pop() {
                    item.execute(); // Executes
                }
            } else {
                break; // Exit
            }
        }
    }
}

#[cfg(test)]
mod tests;
