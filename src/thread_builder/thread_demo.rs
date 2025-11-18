use std::thread;

fn _another_thread() {
    println!(
        "In thread:{}",
        thread::current().name().unwrap_or("Unnamed")
    );
}

// 创建测试
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_another_thread() {
        let handle = thread::Builder::new()
            .name("Thread1".into())
            .stack_size(4 * 1024 * 1024)
            .spawn(_another_thread)
            .unwrap();
        handle.join().unwrap();
    }

    // 作用域线程
    #[test]
    fn test_scoped_thread() {
        let mut handles = Vec::new();
        for i in 0..5 {
            let handle = thread::spawn(move || {
                thread::sleep(Duration::from_secs(1));
                println!("Normal thread:{}", i);
            });
            handles.push(handle);
        }
        handles.into_iter().for_each(|h| h.join().unwrap());
    }

    // scope 线程
    #[test]
    fn test_scope_thread() {
        const CHUNK_SIZE: usize = 10;
        let numbers: Vec<u32> = (1..10000).collect();
        let chunks = numbers.chunks(CHUNK_SIZE);

        let sum = thread::scope(|s| {
            let mut handles = Vec::new();
            for chunk in chunks {
                let handle = s.spawn(move || chunk.iter().sum::<u32>());
                handles.push(handle);
            }
            handles.into_iter().map(|h| h.join().unwrap()).sum::<u32>()
        });
        println!("Total sum:{}", sum);
    }
}
