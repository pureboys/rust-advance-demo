use std::sync::LazyLock;

fn init() -> i32 {
    println!("initializing...");
    23
}

static NUMBER: LazyLock<i32> = LazyLock::new(|| {
    println!("initializing...");
    100
});

// test 测试
#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::LazyCell, thread};

    #[test]
    fn test_lazy_cell() {
        let lazy_cell = LazyCell::new(init);
        println!("=================");
        println!("{}", *lazy_cell);
        println!("{}", *lazy_cell);
    }

    #[test]
    fn test_lazy_lock() {
        let handles = (0..5)
            .map(|_| {
                thread::spawn(|| {
                    println!("Thead sees NUMBER:{}", *NUMBER);
                })
            })
            .collect::<Vec<_>>();
        handles.into_iter().for_each(|h| h.join().unwrap());
    }
}
