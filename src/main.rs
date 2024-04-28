use scheduled::Scheduler;

fn main() {
    let mut scheduler = Scheduler::new();

    scheduler.schedule_wait_from_now(
        chrono::Duration::seconds(5),
        Box::new(|_| println!("Hello World")),
    );

    loop {
        if scheduler.check() {
            println!("Executed event");
        }
        break;
    }
}
