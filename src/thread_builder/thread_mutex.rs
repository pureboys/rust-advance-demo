// 测试
#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};

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
        drop(f);

        *rc.borrow_mut() += 1;
        println!("{rc:#?}");
    }
}
