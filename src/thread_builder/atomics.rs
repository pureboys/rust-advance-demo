use std::sync::atomic::{AtomicUsize, Ordering};

fn _incr(a: &AtomicUsize) {
    let mut current = a.load(Ordering::Relaxed);
    loop {
        let new = current + 1;
        match a.compare_exchange(current, new, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return,
            Err(v) => {
                println!("value changed {current} -> {v}");
                current = v;
            }
        }
    }
}

// 测试
#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        sync::atomic::{AtomicUsize, Ordering},
        thread,
        time::Duration,
    };

    #[test]
    fn test_atomic() {
        let done = AtomicUsize::new(0);
        thread::scope(|s| {
            for _t in 0..10 {
                s.spawn(|| {
                    for _i in 0..100 {
                        thread::sleep(Duration::from_millis(20));
                        // 不能拆开，因为是原子操作，不能拆开，否则会破坏原子性
                        // let current = done.load(Ordering::Relaxed);
                        // done.store(current + 1, Ordering::Relaxed);
                        done.fetch_add(1, Ordering::Relaxed);
                    }
                });
            }
            loop {
                let n = done.load(Ordering::Relaxed);
                if n == 1000 {
                    break;
                }
                println!("progress: {n}/1000 done!");
                thread::sleep(Duration::from_secs(1));
            }
        });
        println!("All done!");
    }

    // incr test
    #[test]
    fn test_incr() {
        let counter = AtomicUsize::new(0);
        thread::scope(|s| {
            for _ in 0..1000 {
                s.spawn(|| {
                    _incr(&counter);
                });
            }
        });
        println!("counter: {}", counter.load(Ordering::Relaxed));
    }
}
