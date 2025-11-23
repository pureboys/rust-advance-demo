#[cfg(test)]
mod tests {
    use tokio::{
        io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
        net::TcpListener,
        signal,
        sync::broadcast,
    };
    use tokio_util::sync::CancellationToken;

    #[tokio::test]
    async fn test_tokio_main() {
        let subscriber = tracing_subscriber::FmtSubscriber::new();
        tracing::subscriber::set_global_default(subscriber).unwrap();

        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        let (tx, _rx) = broadcast::channel(10);
        let token = CancellationToken::new();
        let cancel_token = token.clone();
        tokio::spawn({
            tracing::info!("spawning new task!");
            async move {
                match signal::ctrl_c().await {
                    Ok(()) => {
                        tracing::warn!("Ctrl+C received");
                        cancel_token.cancel();
                    }
                    Err(e) => {
                        tracing::error!("Error receiving signal: {e}");
                    }
                }
            }
        });
        loop {
            let token = token.clone();
            let tx = tx.clone();
            let mut rx = tx.subscribe();
            let (mut socket, address) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                let (stream_reader, mut stream_writer) = socket.split();
                let mut message = String::new();
                let mut reader = BufReader::new(stream_reader);
                loop {
                    tokio::select! {
                        result = reader.read_line(&mut message) => {
                            tracing::info!("read_line result: {result:#?}");
                            if result.is_err() {
                                break;
                            }
                            if result.unwrap() == 0 {
                                break;
                            }
                            tx.send((message.clone(), address)).unwrap();
                            message.clear();
                        }
                        result = rx.recv() => {
                            if result.is_err() {
                                break;
                            }
                            let (received_message, sender_address) = result.unwrap();
                            if sender_address != address {
                                tracing::info!("received message from {sender_address}: {received_message}");
                                stream_writer.write_all(received_message.as_bytes()).await.unwrap();
                            }
                        }
                        _ = token.cancelled() => {
                            tracing::warn!("Client disconnected: {address}");
                            return;
                        }
                    }
                }
            });
        }
    }
}
