async fn _hi() {
    println!("hi");
}

#[cfg(test)]
mod tests {
    use tokio::runtime;

    use crate::async_rust::tokio_demo::_hi;

    #[test]
    fn test_tokio() {
        let rt = runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(_hi())
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_current_thread() {
        _hi().await
    }

    #[test]
    fn multi_thread() {
        let rt = runtime::Builder::new_multi_thread()
            .worker_threads(10)
            .thread_stack_size(5 * 1024*1024)
            .event_interval(20)
            .max_blocking_threads(256)
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(_hi())
    }
}
