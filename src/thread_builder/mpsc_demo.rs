fn hello() {
    println!("hello world!");
}

type Task = Box<dyn FnOnce() + Send + 'static>;

enum Msg {
    Call(Task),
    Quit,
}

// 测试
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn test_mpsc() {
        let (tx, rx) = mpsc::channel::<Msg>();
        let handle = thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                match msg {
                    Msg::Call(task) => task(),
                    Msg::Quit => break,
                }
            }
        });
        let closure = || println!("hello from closure!");

        tx.send(Msg::Call(Box::new(hello))).unwrap();
        tx.send(Msg::Call(Box::new(closure))).unwrap();
        tx.send(Msg::Call(Box::new(|| println!("hello from Box new!"))))
            .unwrap();
        tx.send(Msg::Quit).unwrap();

        handle.join().unwrap();
    }
}
