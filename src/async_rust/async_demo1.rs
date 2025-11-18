use futures::join;

async fn _hi() {
    println!("hi");
}

async fn _hello() {
    println!("hello");
}

fn _hello_sync() {
    println!("hello sync");
}

async fn _do_mul() {
    join!(_hi(), _hello());
    let sum = _add(1, 2).await;
    println!("sum: {sum}");
    let (a, b) = join!(_add(1, 2), _add(3, 4));
    println!("a: {a}, b: {b}");
}

async fn _add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[test]
    fn test_hi() {
        let func = _hi();
        block_on(func);
    }

    #[test]
    fn test_do_mul() {
        block_on(_do_mul());
    }
}
