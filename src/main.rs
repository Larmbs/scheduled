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
