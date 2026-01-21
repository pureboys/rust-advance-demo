async fn demo7_main() {
    let (tx, mut rx) = tokio::sync::broadcast::channel::<String>(16);
    for n in 0..20 {
        let mut messages = tx.subscribe();
        tokio::spawn(async move {
            while let Ok(msg) = messages.recv().await {
                println!("{n}: {msg}");
            }
        });
    }
    tx.send("Hello channel".to_string()).unwrap();
    while let Ok(msg) = rx.recv().await {
        println!("Main: {msg}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test(flavor = "multi_thread")]
    async fn test_demo7() {
        demo7_main().await;
    }
}
