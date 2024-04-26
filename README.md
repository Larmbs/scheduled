# Scheduled
A simple Crate to allow scheduling of events on a UTC datetime system.
Users can schedule events and can setup loops to check for event expiration to run
functions

## Scheduler
This is an object that contains a list of events that are soon to happen.
this scheduler manages these events and allows users to create their own events.

Example
```rust
use scheduled::Scheduler;

fn main() {
    let mut scheduler = Scheduler::new();

    scheduler.schedule_wait_from_now(chrono::Duration::seconds(5), |_| {println!("Hello World")});

    loop {
        if scheduler.check() {
            println!("Executed event");
        }
        break;
    }
}
```