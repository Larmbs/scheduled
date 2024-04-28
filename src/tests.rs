use super::*;
use chrono::TimeDelta;

#[test]
fn schedule_duration() {
    // Creating scheduler obj
    let mut scheduler = Scheduler::new();

    // Creating events
    scheduler.schedule_wait_from_now(
        TimeDelta::milliseconds(20),
        Box::new(|_| {
            println!("hello");
        }),
    ); // Make sure to wrap in box
    scheduler.schedule_wait_from_now(
        TimeDelta::milliseconds(40),
        Box::new(|_| {
            println!("hi");
        }),
    );

    // Testing to see if event count is expected
    let mut count = 0;
    loop {
        if scheduler.check() {
            // Method that checks if events have expired
            count += 1;
        }
        if scheduler.is_empty() {
            // Method that checks if all events have been used up
            break;
        }
    }

    assert_eq!(2, count);
}

#[test]
fn test_schedule_date() {
    let mut scheduler = Scheduler::new();
    let end_date = Utc::now() + Duration::seconds(60);
    scheduler.schedule_date(end_date, Box::new(|_| println!("Event executed")));

    assert_eq!(scheduler.get_event_count(), 1);
    assert!(!scheduler.is_empty());
}

#[test]
fn test_schedule_wait_from_now() {
    let mut scheduler = Scheduler::new();
    scheduler.schedule_wait_from_now(
        Duration::seconds(60),
        Box::new(|_| println!("Event executed")),
    );

    assert_eq!(scheduler.get_event_count(), 1);
    assert!(!scheduler.is_empty());
}

#[test]
fn test_schedule_event() {
    let mut scheduler = Scheduler::new();
    let event = Event::new(
        Utc::now() + Duration::seconds(60),
        Box::new(|_| println!("Event executed")),
    );
    scheduler.schedule_event(event);

    assert_eq!(scheduler.get_event_count(), 1);
    assert!(!scheduler.is_empty());
}

#[test]
fn test_check_no_expired_event() {
    let mut scheduler = Scheduler::new();
    scheduler.schedule_wait_from_now(
        Duration::seconds(60),
        Box::new(|_| println!("Event executed")),
    );

    assert!(!scheduler.check());
    assert_eq!(scheduler.get_event_count(), 1);
}

#[test]
fn test_check_expired_event() {
    let mut scheduler = Scheduler::new();
    scheduler.schedule_date(Utc::now() - Duration::seconds(60), Box::new(|_| {
        println!("Event executed")
    }));

    assert!(scheduler.check());
    assert_eq!(scheduler.get_event_count(), 0);
    assert!(scheduler.is_empty());
}

#[test]
fn test_check_multiple_expired_events() {
    let mut scheduler = Scheduler::new();
    scheduler.schedule_date(Utc::now() - Duration::seconds(60), Box::new(|_| {
        println!("Event executed")
    }));
    scheduler.schedule_date(Utc::now() - Duration::seconds(30), Box::new(|_| {
        println!("Event executed")
    }));

    assert!(scheduler.check());
    assert_eq!(scheduler.get_event_count(), 0);
    assert!(scheduler.is_empty());
}
