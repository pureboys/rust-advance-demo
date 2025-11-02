// test 测试
#[cfg(test)]
mod tests {

    use std::{
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        },
        thread,
        time::Duration,
    };

    #[test]
    fn test_thread_park() {
        let flag = Arc::new(AtomicBool::new(false));
        let flag2 = Arc::clone(&flag);

        let parked_thread = thread::spawn(move || {
            while !flag2.load(Ordering::Relaxed) {
                println!("Parking thread");
                thread::park();
                println!("Unparked thread");
            }
            println!("Flag received")
        });

        thread::sleep(Duration::from_millis(100));
        flag.store(true, Ordering::Relaxed);
        parked_thread.thread().unpark();
        parked_thread.join().unwrap();
    }
}
