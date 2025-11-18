use std::{thread, time::Duration};

async fn _hello(task: u64, time: u64) {
    println!("Task {task} started {:?}", std::thread::current().id());
    thread::sleep(Duration::from_millis(time));
    println!("Task {task} finished");
}

async fn _hello_async(task: u64, time: u64) {
    println!("Task {task} started {:?}", std::thread::current().id());
    tokio::time::sleep(Duration::from_millis(time)).await;
    println!("Task {task} finished");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async() {
        tokio::join!(
            _hello(1, 200),
            _hello(2, 200),
            _hello(3, 200),
            _hello(4, 200)
        );
        println!("All tasks finished");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async2() {
        let _ = tokio::join!(
            tokio::spawn(_hello(1, 200)),
            tokio::spawn(_hello(2, 200)),
            tokio::spawn(_hello(3, 200)),
            tokio::spawn(_hello(4, 200))
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async3() {
        let _ = tokio::join!(
            _hello_async(1, 200),
            _hello_async(2, 200),
            _hello_async(3, 200),
            _hello_async(4, 200)
        );
    }

}
