// 测试
#[cfg(test)]
mod tests {
    use std::{
        cell::{Cell, RefCell},
        sync::{Arc, Mutex, RwLock},
        thread,
    };

    #[test]
    fn test_cell() {
        let cell = Cell::new(5);
        assert_eq!(cell.get(), 5);

        assert_eq!(cell.replace(10), 5);
        assert_eq!(cell.get(), 10);

        let ten = cell.into_inner();
        assert_eq!(ten, 10);

        let cell = Cell::new(String::from("hello"));
        assert_eq!(cell.take(), "hello");
        assert_eq!(cell.take(), String::default());

        cell.set(String::from("world"));
        let word = cell.take();
        assert_eq!(word, "world");
    }

    // ref cell
    #[test]
    fn test_ref_cell() {
        let rc = RefCell::new(5);
        {
            let _five = rc.borrow();
            let _five1 = rc.borrow();
        }
        let mut f = rc.borrow_mut();
        *f += 6;

        // 正在借用，无法再次借用
        let v = rc.try_borrow();
        assert!(v.is_err());
        drop(f);

        *rc.borrow_mut() += 1;
        println!("{rc:#?}");
    }

    // mutex
    #[test]
    fn test_mutex() {
        static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());
        let mut handles = Vec::new();
        for _ in 0..10 {
            let h = thread::spawn(|| {
                let mut lock = NUMBERS.lock().unwrap();
                lock.push(1);
            });
            handles.push(h);
        }
        handles.into_iter().for_each(|h| h.join().unwrap());

        let lock = NUMBERS.lock().unwrap();
        println!("{lock:#?}");
    }

    // mutex2
    #[test]
    fn test_mutex2() {
        let data = Arc::new(Mutex::new(0));
        {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let mut lock = data.lock().unwrap();
                *lock += 1;
                panic!();
            })
            .join()
            .unwrap_err();
        }
        {
            let data = Arc::clone(&data);
            thread::spawn(move || match data.lock() {
                Ok(mut lock) => {
                    println!("Thread2:OK");
                    *lock += 10000;
                }
                Err(e) => {
                    println!("Thread2:Mutex poisoned");
                    let mut guard = e.into_inner();
                    *guard += 1;
                    println!("Thread2 2:New value {}", *guard);
                }
            })
            .join()
            .unwrap();
        }
    }

    // rwLock
    #[test]
    fn test_rw_lock() {
        let counter = Arc::new(RwLock::new(0));
        let mut handles = Vec::new();
        for i in 0..10 {
            let counter = Arc::clone(&counter);
            let h = thread::spawn(move || {
                let value = counter.read().unwrap();
                println!("Thread:{i}, value:{value}");
            });
            handles.push(h);
        }
        {
            let counter = Arc::clone(&counter);
            let h = thread::spawn(move || {
                let mut value = counter.write().unwrap();
                *value += 1;
                println!("write updated the value to {value}");
            });
            handles.push(h);
        }
        handles.into_iter().for_each(|h| h.join().unwrap());
        println!("Counter: {:?}", counter.read().unwrap());
    }
}
