use std::{thread, time::Duration};

use tokio::task::spawn_blocking;

async fn _delay(task: u64, time: u64) {
    println!("Task {task} started {:?}", std::thread::current().id());
    let result = spawn_blocking(move || {
        thread::sleep(Duration::from_secs(time));
        time
    })
    .await;
    println!("Task {task} result: {result:#?}");
    println!("Task {task} finished");
}

async fn _no_delay(task: u64, time: u64) {
    println!("Task {task} started {:?}", std::thread::current().id());
    spawn_blocking(move || {
        thread::sleep(Duration::from_secs(time));
        println!("Task {task} no delay");
        time
    });
    println!("Task {task} finished");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async() {
        tokio::join!(_delay(1, 1), _delay(2, 2), _delay(3, 3));
        println!("All tasks finished");
    }

    #[tokio::test(flavor = "multi_thread")] 
    async fn test_no_delay() {
        tokio::join!(_no_delay(1, 1), _no_delay(2, 2), _no_delay(3, 3));
        println!("All tasks finished");
    }

}
