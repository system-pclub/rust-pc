use futures::{future, stream, Stream}; // 0.1.29
use std::time::Duration;
use tokio; // 0.1.22
use tokio_timer::Interval; // 0.1

fn main() {
    tokio::run({
        let mut some_vars = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let interval_timer = tokio_timer::Timer::default();

        let timer = interval_timer
            .interval(Duration::from_millis(1000))
            .map_err(|_| {
                println!("Errored out");
            });
        let some_vars = stream::iter_ok(some_vars.into_iter().rev());
        let combined = timer.zip(some_vars);

        combined.for_each(move |(_, item)| {
            eprintln!("Woke up");

            tokio::spawn(future::lazy(move || {
                println!("{:?}", item);
                Ok(())
            }));
            Ok(())
        })
    });
}