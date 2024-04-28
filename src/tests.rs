use super::*;
use chrono::TimeDelta;

#[test]
fn schedule_duration() {
    // Creating scheduler obj
    let mut scheduler = Scheduler::new();

    // Creating events
    scheduler.schedule_wait_from_now(TimeDelta::milliseconds(20), Box::new(|_| {println!("hello");})); // Make sure to wrap in box
    scheduler.schedule_wait_from_now(TimeDelta::milliseconds(40), Box::new(|_| {println!("hi");}));

    // Testing to see if event count is expected
    let mut count = 0;
    loop {
        if scheduler.check() {    // Method that checks if events have expired
            count += 1; 
        }
        if scheduler.is_empty() { // Method that checks if all events have been used up
            break;
        }
    }

    assert_eq!(2, count);
}
