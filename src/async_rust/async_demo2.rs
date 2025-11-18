async fn _hello() {
    println!("hello")
}

async fn _run() {
    for i in 0..10 {
        println!("{i}");
        tokio::task::yield_now().await;
    }
}

async fn _add(a: i32, b: i32) -> i32 {
    println!("add {a} + {b}");
    a + b
}

#[cfg(test)]
mod tests {
    use tokio::task::JoinSet;

    use super::*;

    #[tokio::test]
    async fn test_async() {
        let handle = tokio::spawn(_run());
        _hello().await;
        handle.await.unwrap();
    }

    #[tokio::test]
    async fn test_add() {
        let result = tokio::join!(_add(1, 2), _add(3, 4));
        println!("result: {result:#?}");
    }

    #[tokio::test]
    async fn test_join_set() {
        let mut set = JoinSet::new();
        for i in 0..10 {
            set.spawn(_add(i, 2));
        }
        while let Some(result) = set.join_next().await {
            println!("result: {result:#?}");
        }
    }
    #[tokio::test]
    async fn test_join2() {
        let _ = tokio::join!(
            tokio::spawn(_hello()),
            tokio::spawn(_run()),
            tokio::spawn(_run()),
        );
        println!("Finish")
    }
}
