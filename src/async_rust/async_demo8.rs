use std::{sync::LazyLock, time::Duration};

use tokio::sync::Mutex;

static _DATA: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

async fn _demo8_task1() {
    println!("Task1 try lock");
    let _guard = _DATA.lock().await;
    println!("Task1 locked, sleep 5s");
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("Task1 done");
}

async fn _demo8_task2() {
    tokio::time::sleep(Duration::from_millis(100)).await;
    println!("Task2 try lock");
    let _guard = _DATA.lock().await;
    println!("Task2 done");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test(flavor = "multi_thread")]
    async fn test_demo8() {
        tokio::join!(_demo8_task1(), _demo8_task2());
    }
}
