static _DATA: [i32; 5] = [1, 2, 3, 4, 5];

// 测试
#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::Arc, thread};

    #[test]
    fn test_shared_data() {
        let mut handles = Vec::new();
        for _ in 0..100 {
            let h = thread::spawn(|| {
                println!("DATA:{_DATA:#?}");
            });
            handles.push(h);
        }
        handles.into_iter().for_each(|h| h.join().unwrap());
    }

    // box leak
    #[test]
    fn test_box_leak() {
        let data: &'static [i32; 5] = Box::leak(Box::new([1, 2, 3, 4, 5]));
        let mut handles = Vec::new();
        for _ in 0..100 {
            let h = thread::spawn(move || {
                println!("DATA:{data:#?}");
            });
            handles.push(h);
        }
        handles.into_iter().for_each(|h| h.join().unwrap());
    }

    // arc T
    #[test]
    fn test_arc_thread() {
        let data = Arc::new([1, 2, 3, 4, 5]);
        let mut handles = Vec::new();
        for _ in 0..100 {
            let local_data = data.clone();
            let h = thread::spawn(move || {
                println!("DATA:{local_data:#?}");
            });
            handles.push(h);
        }
    }
}
