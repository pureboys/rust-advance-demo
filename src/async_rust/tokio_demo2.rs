#[cfg(test)]
mod tests {
    use std::pin::Pin;

    
    #[tokio::test]
    async fn test_tokio_main() {
        let task = get_task("db");
        let result = task.await;
        println!("result: {result}");
    }

    fn get_task(source: &str) -> Pin<Box<dyn Future<Output = String>>> {
        match source {
            "db" => Box::pin(from_db()),
            "api" => Box::pin(from_api()),
            _ => Box::pin(async { "default result".to_string() }),
        }
    }
    async fn from_db() -> String {
        "db result".to_string()
    }
    async fn from_api() -> String {
        "api result".to_string()
    }
}
