# Scheduled

The Scheduled Crate provides utilities for managing scheduled events in Rust applications.
This is just a simple implementation of a event queue and is meant to be used in any automated applications.

## Features
This scheduler uses chronos time system to specify dates and times so it is best to pair this with their types.
- Schedule events to occur at specific dates or after certain durations.
- Manage a queue of events, executing them when their activation time is reached.
- Customizable event functions with the `EventFunc` type.

Example
```rust
use scheduled::{Scheduler, EventFunc};
use chrono::{Utc, Duration};

fn main() {
    // Create a scheduler
    let mut scheduler = Scheduler::new();

    // Schedule an event to occur at a specific date
    let end_date = Utc::now() + Duration::seconds(4);
    let event_func: EventFunc = Box::new(|time| println!("Event executed at {}", time));
    scheduler.schedule_date(end_date, event_func);

    // Schedule another event to occur after a certain duration from the current time
    let wait_duration = Duration::seconds(5);
    let event_func: EventFunc = Box::new(|time| println!("Another event executed at {}", time));
    scheduler.schedule_wait_from_now(wait_duration, event_func);

    loop {
        if scheduler.check() {
            // Print the count of remaining events in the scheduler
            println!("Remaining events in scheduler: {}", scheduler.get_event_count());
        } else {
            if scheduler.is_empty() {
                break;
            };
        };
    }

    println!("Done executing events");
}

```

## Structs

### Scheduler
This struct is responsible for managing the events you have as well as adding them and executing them.

### Event
Internal representation of an event.
It is a pair of do funcs and a date time.
