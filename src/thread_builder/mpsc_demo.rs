fn _hello() {
    println!("hello world!");
}

type _Task = Box<dyn FnOnce() + Send + 'static>;

enum _Msg {
    Call(_Task),
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
        let (tx, rx) = mpsc::channel::<_Msg>();
        let handle = thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                match msg {
                    _Msg::Call(task) => task(),
                    _Msg::Quit => break,
                }
            }
        });
        let closure = || println!("hello from closure!");

        tx.send(_Msg::Call(Box::new(_hello))).unwrap();
        tx.send(_Msg::Call(Box::new(closure))).unwrap();
        tx.send(_Msg::Call(Box::new(|| println!("hello from Box new!"))))
            .unwrap();
        tx.send(_Msg::Quit).unwrap();

        handle.join().unwrap();
    }
}
