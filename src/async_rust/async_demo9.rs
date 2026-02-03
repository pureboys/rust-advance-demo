use std::time::Duration;
use tokio::sync::{broadcast, mpsc};

async fn _do_work() {
    tokio::time::sleep(Duration::from_secs(3)).await;
}

async fn _timeout(secs: f32) {
    tokio::time::sleep(Duration::from_secs_f32(secs)).await;
}

async fn _receiver(mut rx: mpsc::Receiver<u32>, mut broadcast_rx: broadcast::Receiver<u32>) {
    loop {
        tokio::select! {
            Some(msg) = rx.recv() => println!("mpsc received: {msg}"),
            Ok(msg) = broadcast_rx.recv() => println!("broadcast received: {msg}"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[tokio::test(flavor = "multi_thread")]
    async fn test_demo9() {
        tokio::select! {
            _ = _do_work() => println!("_do_work completed first"),
            _ = _timeout(1.0) => println!("timeout completed first")
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_demo9_receiver() {
        let (tx, rx) = mpsc::channel::<u32>(1);
        let (broadcast_tx, broadcast_rx) = broadcast::channel::<u32>(1);

        tokio::spawn(_receiver(rx, broadcast_rx));
        for c in 0..10 {
            if c % 2 == 0 {
                tx.send(c).await.unwrap();
            } else {
                broadcast_tx.send(c).unwrap();
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
