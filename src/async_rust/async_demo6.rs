use std::{sync::mpsc, thread, time::Duration};

enum Task {
    Calculate(i32),
}
async fn demo6_main() {
    let (tx, rx) = mpsc::channel::<Task>();
    let (tx_reply, mut rx_reply) = tokio::sync::mpsc::channel::<i32>(10);
    let handle = tokio::runtime::Handle::current();

    // 执行cpu密集的任务，不需要异步
    thread::spawn(move || {
        while let Ok(task) = rx.recv() {
            match task {
                Task::Calculate(n) => {
                    let tx_reply = tx_reply.clone();
                    let result = n * n;
                    handle.spawn(async move {
                        tx_reply.send(result).await.unwrap();
                    });
                },
            }
        }
    });

    tokio::spawn(async move {
        while let Some(result) = rx_reply.recv().await {
            println!("Result: {result}");
        }
    });

    let mut num = 1;
    loop {
        tokio::time::sleep(Duration::from_millis(500)).await;
        tx.send(Task::Calculate(num)).unwrap();
        num += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test(flavor = "multi_thread")]
    async fn test_demo6() {
        demo6_main().await;
    }
}
