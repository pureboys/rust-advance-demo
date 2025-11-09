use std::sync::{OnceLock, atomic::AtomicU32};

static _LIST: _OnceList<u32> = _OnceList::_new();
static _COUNTER: AtomicU32 = AtomicU32::new(0);
const _LEN: u32 = 1000;

struct _OnceList<T> {
    data: OnceLock<T>,
    next: OnceLock<Box<_OnceList<T>>>,
}

impl<T> _OnceList<T> {
    const fn _new() -> _OnceList<T> {
        _OnceList {
            data: OnceLock::new(),
            next: OnceLock::new(),
        }
    }

    fn _push(&self, value: T) {
        if let Err(value) = self.data.set(value) {
            let next = self.next.get_or_init(|| Box::new(_OnceList::_new()));
            next._push(value);
        }
    }

    fn _contains(&self, example: &T) -> bool
    where
        T: PartialEq,
    {
        self.data
            .get()
            .map(|item| item == example)
            .filter(|v| *v)
            .unwrap_or_else(|| {
                self.next
                    .get()
                    .map(|next| next._contains(example))
                    .unwrap_or(false)
            })
    }
}

// 测试
#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        cell::OnceCell,
        sync::{OnceLock, atomic::Ordering},
        thread,
    };

    use crate::thread_builder::once_cell::_LEN;
    #[test]
    fn test_once_cell() {
        let mut cell = OnceCell::new();
        let _ = cell.set(String::from("hello"));

        if let Some(value_ref) = cell.get_mut() {
            value_ref.push('!')
        }

        if let Some(value) = cell.get() {
            println!("value: {value}");
        }
    }

    #[test]
    fn test_once_lock() {
        static LOCK: OnceLock<usize> = OnceLock::new();
        assert!(LOCK.get().is_none());
        thread::spawn(|| {
            let value = LOCK.get_or_init(|| 12345);
            assert_eq!(value, &12345);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn test_once_list() {
        thread::scope(|s| {
            for _ in 0..thread::available_parallelism().unwrap().get() {
                s.spawn(|| {
                    while let i @ 0.._LEN = _COUNTER.fetch_add(1, Ordering::Relaxed) {
                        _LIST._push(i);
                    }
                });
            }
        });
        for i in 0.._LEN {
            assert!(_LIST._contains(&i));
        }
    }
}
